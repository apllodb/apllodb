//! Rustdoc web server for apllodb

use actix_files::NamedFile;
use actix_web::{
    dev::ServiceRequest,
    middleware::{Condition, Logger, NormalizePath},
    web, App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};
use env_logger::Env;
use envconfig::Envconfig;
use envconfig_derive::Envconfig;
use lazy_static::lazy_static;
use log::info;
use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

#[cfg(debug_assertions)]
const IP: &str = "127.0.0.1";

#[cfg(not(debug_assertions))]
const IP: &str = "0.0.0.0";

#[cfg(debug_assertions)]
const DOCPATH: &str = "target/doc";

#[cfg(not(debug_assertions))]
const DOCPATH: &str = "target/release/docs";

#[derive(Envconfig, Debug)]
struct Config {
    #[envconfig(from = "BASICAUTH_USER")]
    basicauth_username: Option<String>,
    #[envconfig(from = "BASICAUTH_PASSWORD")]
    basicauth_password: Option<String>,
    #[envconfig(from = "PORT", default = "8088")]
    port: u16,
}

impl Config {
    fn basic_auth(&self) -> Option<(&str, &str)> {
        match (
            self.basicauth_username.as_ref(),
            self.basicauth_password.as_ref(),
        ) {
            (Some(u), Some(p)) => Some((u, p)),
            _ => None,
        }
    }
}

lazy_static! {
    static ref CONFIG: Config = Config::init().unwrap();
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let basicauth_enabled = CONFIG.basic_auth().is_some();
    if basicauth_enabled {
        info!("basic auth is enabled");
    }

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath)
            .wrap(Condition::new(
                basicauth_enabled,
                HttpAuthentication::basic(validator),
            ))
            .route("/{filename:.*}", web::get().to(index))
    })
    .bind((IP, CONFIG.port))?
    .run()
    .await
}

async fn validator(req: ServiceRequest, credentials: BasicAuth) -> Result<ServiceRequest> {
    let (username, password) = CONFIG.basic_auth().unwrap();

    if credentials.user_id().as_ref() == username
        && credentials
            .password()
            .map(|p| p.as_ref() == password)
            .unwrap_or(false)
    {
        return Ok(req);
    }

    Err(HttpResponse::Unauthorized().body("unauthorized").into())
}

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let rawpath = req.match_info().query("filename");

    let mut path: PathBuf = rawpath.parse().unwrap();
    if path.is_dir() || rawpath.is_empty() {
        path.push("index.html");
    }

    let docpath = Path::new(DOCPATH).join(path);
    Ok(NamedFile::open(docpath).map_err(|err| -> Error {
        match err.kind() {
            ErrorKind::NotFound => HttpResponse::NotFound().body("not found").into(),
            ErrorKind::PermissionDenied => HttpResponse::Forbidden().body("forbidden").into(),
            _ => err.into(),
        }
    })?)
}
