#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde;
extern crate url;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde_json;

use reqwest::Response;
use rocket::config::{Config, Environment};
use rocket::response::Stream;

mod circleci;
mod utils;

#[cfg(not(debug_assertions))]
const ROCKET_ENVIRONMENT: Environment = Environment::Production;

#[cfg(debug_assertions)]
const ROCKET_ENVIRONMENT: Environment = Environment::Development;

#[get("/<org>/<repo>/<build>/<path>")]
pub fn get_asset_for_build(
    org: String,
    repo: String,
    build: String,
    path: String,
) -> Option<Stream<Response>> {
    if !utils::is_valid_build_num(&build) {
        return None;
    }
    let url = circleci::get_build_asset_url(org, repo, build);
    let artifacts = circleci::get_artifacts_from(url)?;
    let artifact = utils::filter_artifacts(artifacts, path);
    return artifact.map(|a| Stream::chunked(circleci::fetch_artifact(a), 4096));
}

fn main() {
    let config = Config::build(ROCKET_ENVIRONMENT)
        .port(utils::get_port())
        .unwrap();
    rocket::custom(config)
        .mount("/", routes![get_asset_for_build])
        .launch();
}
