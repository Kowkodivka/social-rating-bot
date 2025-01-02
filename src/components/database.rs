use sqlx::{Error, Pool, Sqlite, SqlitePool};

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, Error> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn initialize(&self) -> Result<(), Error> {
        let schema = r#"
            CREATE TABLE IF NOT EXISTS user_experience (
                user_id INTEGER PRIMARY KEY,
                total_experience INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS user_global_reputation (
                user_id INTEGER PRIMARY KEY,
                global_reputation INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS user_message_reputation (
                user_id INTEGER NOT NULL,
                message_id INTEGER NOT NULL,
                reputation INTEGER NOT NULL DEFAULT 0,
                giver_id INTEGER NOT NULL,
                PRIMARY KEY (user_id, message_id, giver_id)
            );
        "#;

        sqlx::query(schema).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn update_user_experience(&self, user_id: i64, experience: i64) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO user_experience (user_id, total_experience)
            VALUES (?, ?)
            ON CONFLICT(user_id) DO UPDATE
            SET total_experience = excluded.total_experience
            "#,
        )
        .bind(user_id)
        .bind(experience)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn fetch_user_experience(&self, user_id: i64) -> Result<Option<i64>, Error> {
        sqlx::query_scalar(
            r#"
            SELECT total_experience
            FROM user_experience
            WHERE user_id = ?
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn fetch_leaderboard(&self, limit: i64) -> Result<Vec<(i64, i64)>, Error> {
        let rows: Vec<(i64, i64)> = sqlx::query_as(
            r#"
            SELECT user_id, total_experience
            FROM user_experience
            ORDER BY total_experience DESC
            LIMIT ?;
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn update_user_global_reputation(
        &self,
        user_id: i64,
        reputation: i64,
    ) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO user_global_reputation (user_id, global_reputation)
            VALUES (?, ?)
            ON CONFLICT(user_id) DO UPDATE
            SET global_reputation = excluded.global_reputation
            "#,
        )
        .bind(user_id)
        .bind(reputation)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn fetch_user_global_reputation(&self, user_id: i64) -> Result<Option<i64>, Error> {
        sqlx::query_scalar(
            r#"
            SELECT global_reputation
            FROM user_global_reputation
            WHERE user_id = ?
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn add_message_reputation(
        &self,
        user_id: i64,
        message_id: i64,
        giver_id: i64,
        reputation: i64,
    ) -> Result<(), Error> {
        let existing = sqlx::query_scalar::<_, Option<i64>>(
            r#"
            SELECT 1
            FROM user_message_reputation
            WHERE user_id = ? AND message_id = ? AND giver_id = ?
            "#,
        )
        .bind(user_id)
        .bind(message_id)
        .bind(giver_id)
        .fetch_optional(&self.pool)
        .await?;

        if existing.is_some() {
            return Err(Error::RowNotFound);
        }

        sqlx::query(
            r#"
            INSERT INTO user_message_reputation (user_id, message_id, reputation, giver_id)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(user_id)
        .bind(message_id)
        .bind(reputation)
        .bind(giver_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn fetch_message_reputation(&self, message_id: i64) -> Result<Option<i64>, Error> {
        sqlx::query_scalar(
            r#"
            SELECT SUM(reputation)
            FROM user_message_reputation
            WHERE message_id = ?
            "#,
        )
        .bind(message_id)
        .fetch_optional(&self.pool)
        .await
    }
}
