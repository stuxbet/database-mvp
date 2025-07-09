use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

/// Blanket repository for any serde-able record that lives in one table.
pub struct Repo<'db, T> {
    db: &'db Surreal<Client>,
    table: &'static str,
    _p: std::marker::PhantomData<T>,
}

impl<'db, T> Repo<'db, T> {
    pub fn new(db: &'db Surreal<Client>, table: &'static str) -> Self {
        Self {
            db,
            table,
            _p: std::marker::PhantomData,
        }
    }
}

#[async_trait]
pub trait Crud<T, Id> {
    async fn create(&self, id: Id, value: T) -> surrealdb::Result<Option<T>>;
    async fn read(&self, id: Id) -> surrealdb::Result<Option<T>>;
    async fn update(&self, id: Id, patch: T) -> surrealdb::Result<Option<T>>;
    async fn delete(&self, id: Id) -> surrealdb::Result<Option<T>>;
}

#[async_trait]
impl<'db, T> Crud<T, String> for Repo<'db, T>
where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    async fn create(&self, id: String, value: T) -> surrealdb::Result<Option<T>> {
        self.db.create((self.table, id)).content(value).await
    }

    async fn read(&self, id: String) -> surrealdb::Result<Option<T>> {
        self.db.select((self.table, id)).await
    }

    async fn update(&self, id: String, patch: T) -> surrealdb::Result<Option<T>> {
        self.db.update((self.table, id)).content(patch).await
    }

    async fn delete(&self, id: String) -> surrealdb::Result<Option<T>> {
        self.db.delete((self.table, id)).await
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
}

// #[derive(Serialize, Deserialize)]
// pub struct RemoteCred {
//     id: String,   // primary key (e.g. "prod")
//     host: String, // "wss:db.mycorp.com:8000"
//     user: String,
//     pass: secrecy::SecretVec<u8>, // encrypted blob on disk
// }
