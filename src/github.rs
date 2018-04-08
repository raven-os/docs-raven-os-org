use std::io::Read;
use std::slice;

use super::RAVEN_DOCS_TOKEN;

use crypto;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;

use rocket::data::{self, FromData};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Data, Outcome};

const X_GITHUB_EVENT: &str = "X-GitHub-Event";
const X_HUB_SIGNATURE: &str = "X-Hub-Signature";

/// GitHub events.
///
/// Currently, on the `Push` event is of interest, but this may change in the future.
#[derive(Debug)]
pub enum GitHubEvent {
    Push,
    Delete,
    Ping,
}

impl<'r, 'a> FromRequest<'r, 'a> for GitHubEvent {
    type Error = ();

    fn from_request(request: &'r Request<'a>) -> request::Outcome<GitHubEvent, Self::Error> {
        if let Some(event) = request.headers().get(X_GITHUB_EVENT).nth(0) {
            match event {
                "push" => Outcome::Success(GitHubEvent::Push),
                "delete" => Outcome::Success(GitHubEvent::Delete),
                "ping" => Outcome::Success(GitHubEvent::Ping),
                _ => Outcome::Failure((Status::BadRequest, ())),
            }
        } else {
            Outcome::Failure((Status::BadRequest, ()))
        }
    }
}

/// Payload of a GitHub webhook.
/// It's signature must be validated before being used.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct GitHubPayload(pub String);

impl FromData for GitHubPayload {
    type Error = ();

    fn from_data(request: &Request, data: Data) -> data::Outcome<GitHubPayload, Self::Error> {
        if let Some(signature) = request.headers().get(X_HUB_SIGNATURE).nth(0) {
            let mut body = String::new();
            if data.open().read_to_string(&mut body).is_ok() {
                if test_signature(signature, body.as_bytes(), RAVEN_DOCS_TOKEN.as_bytes()) {
                    Outcome::Success(GitHubPayload(body))
                } else {
                    Outcome::Failure((Status::BadRequest, ()))
                }
            } else {
                Outcome::Failure((Status::BadRequest, ()))
            }
        } else {
            Outcome::Failure((Status::BadRequest, ()))
        }
    }
}

/// Tests if the given signature is valid for the given body and secret value.
fn test_signature(signature: &str, body: &[u8], secret: &[u8]) -> bool {
    let sha = Sha1::new();
    let mut hmac = Hmac::new(sha, secret);
    hmac.input(body);

    if let Some(signature) = signature.splitn(2, '=').collect::<Vec<_>>().get(1) {
        let hmac_result = hmac.result();
        let result = hmac_result.code();
        let mut hex = Vec::with_capacity(result.len() * 2);
        for b in result {
            hex.push(b"0123456789abcdef"[(b >> 4) as usize]);
            hex.push(b"0123456789abcdef"[(b & 0xF) as usize]);
        }
        unsafe {
            crypto::util::fixed_time_eq(
                signature.as_bytes(),
                slice::from_raw_parts(hex.as_ptr(), hex.len()),
            )
        }
    } else {
        false
    }
}
