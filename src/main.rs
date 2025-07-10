use crate::utils::Crud;
use crate::utils::Repo;
use crate::utils::User;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::RecordId;
use surrealdb::Surreal;
mod utils;
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::RocksDb;

// #[derive(Debug, Serialize)]
// struct Name<'a> {
//     first: &'a str,
//     last: &'a str,
// }

// #[derive(Debug, Serialize)]
// struct Person<'a> {
//     title: &'a str,
//     name: Name<'a>,
//     marketing: bool,
// }

// #[derive(Debug, Serialize)]
// struct Responsibility {
//     marketing: bool,
// }

// #[derive

#[derive(Debug, Serialize)]
struct Person<'a> {
    name: &'a str,
    marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Row {
    id: RecordId,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // start embedded DB in RAM
    let db = Surreal::new::<RocksDb>("./data").await?;
    db.use_ns("app").use_db("app").await?;
    let _: Option<Row> = db
        .create("person")
        .content(Person {
            name: "Tobie",
            marketing: true,
        })
        .await?;

    // query
    let rows: Vec<Row> = db.select("person").await?;
    println!("{rows:#?}");

    // Connect over WebSocket
    let db = Surreal::new::<Ws>("10.77.7.97:8000").await?;

    // Authenticate as root
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select namespace and database
    db.use_ns("test").use_db("test").await?;
    println!("Connected to SurrealDB!");

    let users = Repo::<User>::new(&db, "user");

    // create

    users
        .create(
            "ada".into(),
            User {
                name: "Ada".into(),
                email: "ada@lovelace.io".into(),
            },
        )
        .await?;

    // read
    let fetched = users.read("ada".into()).await?;

    // update
    let updated = users
        .update(
            "ada".into(),
            User {
                name: "Ada L.".into(),
                email: fetched.unwrap().email,
            },
        )
        .await?;

    // delete
    users.delete("ada".into()).await?;

    Ok(())
}
