use sqlx::{query, query_as, Pool, Sqlite};

use crate::proto;

type DbPool = Pool<Sqlite>;

pub struct Db {
    pool: DbPool,
}

impl Db {
    pub fn new(pool: DbPool) -> Self {
        Self { pool: pool }
    }

    pub async fn insert_event(&self, amount: f32, name: &str) -> anyhow::Result<()> {
        query("INSERT INTO event (amount, name) values ($1, $2)")
            .bind(amount)
            .bind(name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn fetch_all_events(&self) -> anyhow::Result<Vec<proto::Event>> {
        Ok(query_as::<_, proto::Event>("SELECT * FROM event")
            .fetch_all(&self.pool)
            .await?)
    }
}
