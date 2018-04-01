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
// Features
#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process::{self, Command};

use regex::Regex;
use rocket_contrib::Json;

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
    static ref REGEX_PROJECT_NAME: Regex = Regex::new(r"^[a-z0-9\-_]+$").unwrap();
}

/// Pulls and updates the given project
#[get("/<project>/<token>")]
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn pull(project: String, token: String) -> Json<HashMap<&'static str, &'static str>> {
    let status = {
        if token == *RAVEN_DOCS_TOKEN && REGEX_PROJECT_NAME.is_match(&project) {
            let p = Path::new("scripts/project/")
                .join(project)
                .with_extension("sh");
            if p.exists() {
                Command::new(p)
                    .env_remove("RAVEN_DOCS_TOKEN")
                    .spawn()
                    .is_ok()
            } else {
                false
            }
        } else {
            false
        }
    };
    if status {
        let mut hm = HashMap::new();
        hm.insert("status", "success");
        Json(hm)
    } else {
        let mut hm = HashMap::new();
        hm.insert("status", "fail");
        Json(hm)
    }
}

fn main() {
    // Trigger lazy statics now, and not when needed.
    lazy_static::initialize(&RAVEN_DOCS_TOKEN);
    lazy_static::initialize(&RAVEN_DOCS_PATH);

    // Mount & go
    rocket::ignite().mount("/", routes![pull]).launch();
}
