use std::fs;
use std::path::PathBuf;

use rocket::get;
use rocket::response::NamedFile;

use rocket_contrib::templates::Template;
use serde_json::json;

use crate::{RAVEN_DOCS_PATH, REGEX_IDENTIFIER_NAME};

/// Static file serving for a documentation
#[get("/<project>/<branch>/<path..>")]
pub fn content_path(project: String, branch: String, path: PathBuf) -> Option<NamedFile> {
    if REGEX_IDENTIFIER_NAME.is_match(&project) && REGEX_IDENTIFIER_NAME.is_match(&branch) {
        let mut path = PathBuf::from(&*RAVEN_DOCS_PATH)
            .join(&project)
            .join(branch)
            .join(path);
        if path.is_dir() {
            path = path.join("index.html");
        }
        NamedFile::open(path).ok()
    } else {
        None
    }
}

/// Static file serving for a documentation
#[get("/<project>/<branch>")]
pub fn content_index(project: String, branch: String) -> Option<NamedFile> {
    content_path(project, branch, PathBuf::new())
}

/// Shows all available branches
#[get("/<project>")]
pub fn branches(project: String) -> Template {
    let mut branches = Vec::new();
    if REGEX_IDENTIFIER_NAME.is_match(&project) {
        let path = PathBuf::from(&*RAVEN_DOCS_PATH).join(&project);

        if let Ok(rows) = fs::read_dir(&path) {
            rows.filter_map(|e| Some(e.ok()?.path().file_name()?.to_string_lossy().to_string()))
                .for_each(|entry| {
                    branches.push(entry);
                });
        }
    }
    branches.sort();
    Template::render(
        "branches_listing",
        json!({
            "project": project,
            "branches_len": branches.len(),
            "branches": branches,
        }),
    )
}

/// Shows all available projects
#[get("/")]
pub fn projects() -> Template {
    let path = PathBuf::from(&*RAVEN_DOCS_PATH);
    let mut projects = Vec::new();

    if let Ok(rows) = fs::read_dir(&path) {
        rows.filter_map(|e| Some(e.ok()?.path().file_name()?.to_string_lossy().to_string()))
            .for_each(|entry| {
                projects.push(entry);
            });
    }
    projects.sort();
    Template::render(
        "project_listing",
        json!({
            "projects_len": projects.len(),
            "projects": projects,
        }),
    )
}
