// Rustc
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
// Clippy
#![cfg_attr(feature = "cargo-clippy", warn(fallible_impl_from))]
#![cfg_attr(feature = "cargo-clippy", warn(int_plus_one))]
#![cfg_attr(feature = "cargo-clippy", warn(mem_forget))]
#![cfg_attr(feature = "cargo-clippy", warn(mut_mut))]
#![cfg_attr(feature = "cargo-clippy", warn(mutex_integer))]
#![cfg_attr(feature = "cargo-clippy", warn(pub_enum_variant_names))]
#![cfg_attr(feature = "cargo-clippy", warn(range_plus_one))]
#![cfg_attr(feature = "cargo-clippy", warn(used_underscore_binding))]
#![cfg_attr(feature = "cargo-clippy", warn(wrong_pub_self_convention))]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
// Features
#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate crypto;
extern crate rocket;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde_json as json;

pub mod github;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};

use regex::Regex;
use rocket::response::status::{Accepted, BadRequest};

use self::github::{GitHubEvent, GitHubPayload};

type ApiResult = Result<Accepted<()>, BadRequest<()>>;

// The different environment variables we are using.
//
// The idea here is to check at boot-time if the variable is set and not every time we
// need it.
lazy_static! {
    static ref RAVEN_DOCS_TOKEN: String = {
        match env::var("RAVEN_DOCS_TOKEN") {
            Ok(s) => s,
            Err(_) => {
                eprintln!("error: the RAVEN_DOCS_TOKEN variable is not set.");
                process::exit(1);
            }
        }
    };
    static ref RAVEN_DOCS_PATH: String = {
        match env::var("RAVEN_DOCS_PATH") {
            Ok(s) => s,
            Err(_) => {
                eprintln!("error: the RAVEN_DOCS_PATH variable is not set.");
                process::exit(1);
            }
        }
    };
    static ref REGEX_IDENTIFIER_NAME: Regex = Regex::new(r"^[A-Za-z0-9\-_]+$").unwrap();
}

/// Handles the delete event: removes the documentation folder for the deleted branch.
fn delete_event(data: json::Value) -> ApiResult {
    use json::Value;

    let repository = data.get("repository").and_then(|x| x.get("name"));
    let ref_type = data.get("ref_type");
    let branch = data.get("ref");
    if let (Some(Value::String(repo)), Some(Value::String(ref_type)), Some(Value::String(branch))) =
        (repository, ref_type, branch)
    {
        if REGEX_IDENTIFIER_NAME.is_match(repo) && REGEX_IDENTIFIER_NAME.is_match(branch)
            && ref_type == "branch"
        {
            println!("Removing doc for {}:{}", repo, branch);
            let mut path = PathBuf::from(&*RAVEN_DOCS_PATH);
            path.push(repo);
            path.push(branch);

            return fs::remove_dir_all(path)
                .map(|_| Accepted::<()>(None))
                .map_err(|_| BadRequest::<()>(None));
        }
    }
    Err(BadRequest::<()>(None))
}

/// Handles the push event: updates the documentation.
fn push_event(data: json::Value) -> ApiResult {
    use json::Value;

    let repository = data.get("repository").and_then(|x| x.get("name"));
    let owner = data.get("repository")
        .and_then(|x| x.get("owner"))
        .and_then(|x| x.get("name"));
    let repo_ref = data.get("ref");
    if let (Some(Value::String(repo)), Some(Value::String(owner)), Some(Value::String(repo_ref))) =
        (repository, owner, repo_ref)
    {
        if let Some(branch) = repo_ref.splitn(3, '/').nth(2) {
            if REGEX_IDENTIFIER_NAME.is_match(repo) && REGEX_IDENTIFIER_NAME.is_match(owner)
                && REGEX_IDENTIFIER_NAME.is_match(branch)
            {
                println!("Updating doc for {}/{}:{}", owner, repo, branch);
                return Command::new("./scripts/doc.sh")
                    .arg(owner)
                    .arg(repo)
                    .arg(branch)
                    .env_remove("RAVEN_DOCS_TOKEN")
                    .spawn()
                    .map(|_| Accepted::<()>(None))
                    .map_err(|_| BadRequest::<()>(None));
            }
        }
    }
    Err(BadRequest::<()>(None))
}

/// Pulls and updates the given project
#[post("/webhook", data = "<payload>")]
fn webhook(event: Option<GitHubEvent>, payload: GitHubPayload) -> ApiResult {
    if let Ok(data) = json::from_str::<json::Value>(&payload.0) {
        match event {
            Some(GitHubEvent::Push) => push_event(data),
            Some(GitHubEvent::Delete) => delete_event(data),
            Some(GitHubEvent::Ping) => Ok(Accepted::<()>(None)),
            _ => Err(BadRequest::<()>(None)),
        }
    } else {
        Err(BadRequest::<()>(None))
    }
}

fn main() {
    // Trigger lazy statics now, and not when needed.
    lazy_static::initialize(&RAVEN_DOCS_TOKEN);
    lazy_static::initialize(&RAVEN_DOCS_PATH);
    lazy_static::initialize(&REGEX_IDENTIFIER_NAME);

    // Mount & go
    rocket::ignite().mount("/", routes![webhook]).launch();
}
