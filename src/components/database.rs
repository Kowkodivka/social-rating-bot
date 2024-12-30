use sqlx::{Error, Pool, Sqlite, SqlitePool};

pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(url: &str) -> Result<Self, Error> {
        let pool = SqlitePool::connect(url).await?;
        Ok(Self { pool })
    }

    pub async fn initialize(&self) -> Result<(), Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS user_experience (
                user_id INTEGER PRIMARY KEY,
                total_experience INTEGER NOT NULL DEFAULT 0
            );
            "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_user_experience(
        &self,
        user_id: i64,
        experience_gain: i64,
    ) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO user_experience (user_id, total_experience)
            VALUES (?, ?)
            ON CONFLICT(user_id) DO UPDATE
            SET total_experience = total_experience + ?
            "#,
        )
        .bind(user_id)
        .bind(experience_gain)
        .bind(experience_gain)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_user_experience(&self, user_id: i64) -> Result<Option<i64>, Error> {
        Ok(sqlx::query_scalar(
            r#"
            SELECT total_experience
            FROM user_experience
            WHERE user_id = ?;
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?)
    }
}
