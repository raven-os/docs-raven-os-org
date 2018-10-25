// Features
#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

pub mod github;
pub mod routes;

use std::env;
use std::process;

use lazy_static::lazy_static;
use regex::Regex;
use rocket_contrib::Template;

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

fn main() {
    // Trigger lazy statics now, and not when needed.
    lazy_static::initialize(&RAVEN_DOCS_TOKEN);
    lazy_static::initialize(&RAVEN_DOCS_PATH);
    lazy_static::initialize(&REGEX_IDENTIFIER_NAME);

    // Mount & go
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/api/", routes![routes::api::github_webhook,])
        .mount(
            "/",
            routes![
                routes::front::projects,
                routes::front::branches,
                routes::front::content_path,
                routes::front::content_index,
            ],
        )
        .launch();
}
