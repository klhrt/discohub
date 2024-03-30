#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
#[path = "./router.rs"]
mod router;
use router::*;
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Wss;
use surrealdb::*;

#[derive(Debug, Serialize)]
struct Movie<'a> {
    title: &'a str,
    year: i32,
}

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

fn main() {
    initDatabase();
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

async fn initDatabase() -> surrealdb::Result<()> {
    DB.connect::<Wss>("localhost:8000").await?;

    DB.use_ns("test").use_db("test").await?;

    // DB.update(("movie", "Little Miss Sunshine"))
    //     .content(Movie {
    //         title: "Tobie",
    //         year: 2006,
    //     })
    //     .await?;
    Ok(())
}
