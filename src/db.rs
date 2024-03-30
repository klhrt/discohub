#![allow(non_snake_case)]

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Wss;
use surrealdb::sql::Thing;
use surrealdb::*;

#[derive(Debug, Serialize)]
struct Movie<'a> {
    title: &'a str,
    year: i32,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn initDatabase() -> surrealdb::Result<()> {
    DB.connect::<Wss>("localhost:8000").await?;

    DB.use_ns("test").use_db("test").await?;

    let _created: Vec<Record> = DB
        .create(("movie", "Little Miss Sunshine"))
        .content(Movie {
            title: "Little Miss Sunshine",
            year: 2006,
        })
        .await?
        .unwrap();
    Ok(())
}
