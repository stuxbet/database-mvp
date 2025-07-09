use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use crate::utils::User;
use crate::utils::Repo;
use crate::utils::Crud;
mod utils;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {

    let cred_db = Surreal::new::<File>(("cred.db",)).await?;
    cred_db.use_ns("app").use_db("secrets").await?;

    let cipher_pw = encrypt(b"rootpw")?;                // your AES-GCM helper
    cred_db
        .update(("remote_cred","prod"))
        .content(RemoteCred{
            id:"prod".into(),
            host:"wss://db.mycorp.com:8000".into(),
            user:"root".into(),
            pass:secrecy::SecretVec::new(cipher_pw),
        })
        .await?;


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
        .create("ada".into(), User {
            name: "Ada".into(),
            email: "ada@lovelace.io".into(),
        })
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
