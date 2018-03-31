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

use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{self, Command};

use rocket::request::Form;
use rocket::response::{NamedFile, Redirect};
use rocket_contrib::Json;

// The different environment variables we are using.
//
// The idea here is to check at boot-time if the variable is set and not every time we
// need it.
lazy_static! {
    pub static ref RAVEN_DOCS_TOKEN: String = {
        match env::var("RAVEN_DOCS_TOKEN") {
            Ok(s) => s,
            Err(_) => {
                eprintln!("error: the RAVEN_DOCS_TOKEN variable is not set.");
                process::exit(1);
            }
        }
    };
    pub static ref RAVEN_DOCS_PATH: String = {
        match env::var("RAVEN_DOCS_PATH") {
            Ok(s) => s,
            Err(_) => {
                eprintln!("error: the RAVEN_DOCS_PATH variable is not set.");
                process::exit(1);
            }
        }
    };
}

#[derive(FromForm)]
struct Token {
    token: String,
}

/// Pulls and updates the given project
#[post("/pull/<project..>", data = "<datas>")]
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn pull(project: PathBuf, datas: Form<Token>) -> Json<HashMap<String, String>> {
    let status = {
        if datas.get().token == *RAVEN_DOCS_TOKEN {
            let p = Path::new("pull").join(project).with_extension("sh");
            if p.exists() {
                match Command::new(p).env_remove("RAVEN_DOCS_TOKEN").spawn() {
                    Ok(_) => Ok(()),
                    Err(_) => Err("failed to start"),
                }
            } else {
                Err("bad project")
            }
        } else {
            Err("bad token")
        }
    };
    let result = match status {
        Ok(_) => vec![(String::from("status"), String::from("success"))],
        Err(s) => vec![
            (String::from("status"), String::from("fail")),
            (String::from("data"), String::from(s)),
        ],
    };
    Json(result.iter().cloned().collect())
}

/// Serve documentation files
#[get("/<target..>")]
fn files(mut target: PathBuf) -> Result<Option<NamedFile>, Redirect> {
    let path = Path::new(&(*RAVEN_DOCS_PATH)).join(target.clone());
    if path.is_dir() {
        target.push("index.html");
        if let Some(path) = target.to_str() {
            Err(Redirect::to(path))
        } else {
            Err(Redirect::to("/"))
        }
    } else {
        Ok(NamedFile::open(path).ok())
    }
}

fn main() {
    // Trigger lazy statics now, and not when needed.
    lazy_static::initialize(&RAVEN_DOCS_TOKEN);
    lazy_static::initialize(&RAVEN_DOCS_PATH);

    // Mount & go
    rocket::ignite().mount("/", routes![pull, files]).launch();
}
