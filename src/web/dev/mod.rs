use std::path::Path;

use anyhow::Error;
use rocket::http::ContentType;
use rocket::response::Debug;
use rocket::{
    figment::{providers::Env, Figment, Profile},
    fs::FileServer,
    Build, Rocket,
};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::{database, diesel};
use rsass::{compile_scss_path, output};

pub mod event_details;
pub mod events;
pub mod home;

#[database("entropy_db")]
pub struct EntropyDbConn(diesel::SqliteConnection);

pub type EntropyWebResult<T> = Result<T, Debug<anyhow::Error>>;

#[get("/<file>")]
async fn css(file: String) -> EntropyWebResult<(ContentType, String)> {
    let scss_dir = Path::new("src/web/scss");
    let path = scss_dir.join(Path::new(&file));
    let path = path.with_extension("scss");
    let path = path.as_path();
    let format = output::Format {
        style: output::Style::Introspection,
        ..Default::default()
    };

    let css = compile_scss_path(path, format).map_err(Error::from)?;
    let css = String::from_utf8(css).map_err(Error::from)?;

    Ok((ContentType::CSS, css))
}

pub fn app() -> Rocket<Build> {
    let figment = Figment::from(rocket::Config::default())
        .merge(("template_dir", "src/web/templates"))
        .merge(("databases.entropy_db.url", "entropy.sqlite3"))
        .merge(Env::prefixed("ENTROPY_").global())
        .select(Profile::from_env_or("ENTROPY_PROFILE", "default"));

    rocket::custom(figment)
        .mount("/", home::routes())
        .mount("/events", events::routes())
        .mount("/events", event_details::routes())
        .mount("/css", routes![css])
        .mount("/", FileServer::from("src/web/static"))
        .attach(Template::fairing())
        .attach(EntropyDbConn::fairing())
}

pub async fn run() -> () {
    app().launch().await.unwrap();
}