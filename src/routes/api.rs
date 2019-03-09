use std::fs;
use std::path::PathBuf;
use std::process::Command;

use rocket::post;
use rocket::response::status::{Accepted, BadRequest};
use serde_json::Value;

use crate::github::{GitHubEvent, GitHubPayload};
use crate::{RAVEN_DOCS_PATH, REGEX_IDENTIFIER_NAME};

type ApiResult = Result<Accepted<()>, BadRequest<()>>;

/// Handles the delete event: removes the documentation folder for the deleted branch.
fn delete_event(data: Value) -> ApiResult {
    let repository = data.get("repository").and_then(|x| x.get("name"));
    let ref_type = data.get("ref_type");
    let branch = data.get("ref");
    if let (Some(Value::String(repo)), Some(Value::String(ref_type)), Some(Value::String(branch))) =
        (repository, ref_type, branch)
    {
        if REGEX_IDENTIFIER_NAME.is_match(repo)
            && REGEX_IDENTIFIER_NAME.is_match(branch)
            && ref_type == "branch"
        {
            println!("Removing doc for {}:{}", repo, branch);
            let mut path = PathBuf::from(&*RAVEN_DOCS_PATH);
            path.push(repo);
            path.push(branch);

            let _ = fs::remove_dir_all(path); // Ignore errors
            return Ok(Accepted::<()>(None));
        }
    }
    Err(BadRequest::<()>(None))
}

/// Handles the push event: updates the documentation.
fn push_event(data: Value) -> ApiResult {
    let repository = data.get("repository").and_then(|x| x.get("name"));
    let owner = data
        .get("repository")
        .and_then(|x| x.get("owner"))
        .and_then(|x| x.get("name"));
    let repo_ref = data.get("ref");
    if let (Some(Value::String(repo)), Some(Value::String(owner)), Some(Value::String(repo_ref))) =
        (repository, owner, repo_ref)
    {
        if let Some(branch) = repo_ref.splitn(3, '/').nth(2) {
            if REGEX_IDENTIFIER_NAME.is_match(repo)
                && REGEX_IDENTIFIER_NAME.is_match(owner)
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
#[post("/github", data = "<payload>")]
pub fn github_webhook(event: Option<GitHubEvent>, payload: GitHubPayload) -> ApiResult {
    if let Ok(data) = serde_json::from_str::<Value>(&payload.0) {
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
