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
            CREATE TABLE IF NOT EXISTS user_global_reputation (
                user_id INTEGER PRIMARY KEY,
                global_reputation INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS user_message_reputation (
                user_id INTEGER NOT NULL,  -- Пользователь, получивший репутацию
                message_id INTEGER NOT NULL,  -- ID сообщения
                reputation INTEGER NOT NULL DEFAULT 0,  -- Репутация за сообщение
                giver_id INTEGER NOT NULL,  -- Кто выставил репутацию
                PRIMARY KEY (user_id, message_id)
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

    pub async fn set_user_global_reputation(
        &self,
        user_id: i64,
        reputation: i64,
    ) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO user_global_reputation (user_id, global_reputation)
            VALUES (?, ?)
            ON CONFLICT(user_id) DO UPDATE
            SET global_reputation = ?
            "#,
        )
        .bind(user_id)
        .bind(reputation)
        .bind(reputation)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_user_global_reputation(&self, user_id: i64) -> Result<Option<i64>, Error> {
        Ok(sqlx::query_scalar(
            r#"
            SELECT global_reputation
            FROM user_global_reputation
            WHERE user_id = ?;
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn set_message_reputation(
        &self,
        user_id: i64,
        message_id: i64,
        giver_id: i64,
        reputation: i64,
    ) -> Result<(), Error> {
        let existing_reputation: Option<i64> = sqlx::query_scalar(
            r#"
            SELECT reputation
            FROM user_message_reputation
            WHERE user_id = ? AND message_id = ? AND giver_id = ?;
            "#,
        )
        .bind(user_id)
        .bind(message_id)
        .bind(giver_id)
        .fetch_optional(&self.pool)
        .await?;

        if existing_reputation.is_some() {
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

        self.update_global_reputation(user_id, reputation).await?;

        Ok(())
    }

    pub async fn get_message_reputation(&self, message_id: i64) -> Result<Option<i64>, Error> {
        Ok(sqlx::query_scalar(
            r#"
            SELECT SUM(reputation)
            FROM user_message_reputation
            WHERE message_id = ?;
            "#,
        )
        .bind(message_id)
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn remove_message_reputation(
        &self,
        user_id: i64,
        message_id: i64,
        giver_id: i64,
    ) -> Result<(), Error> {
        let existing_reputation: Option<i64> = sqlx::query_scalar(
            r#"
            SELECT reputation
            FROM user_message_reputation
            WHERE user_id = ? AND message_id = ? AND giver_id = ?;
            "#,
        )
        .bind(user_id)
        .bind(message_id)
        .bind(giver_id)
        .fetch_optional(&self.pool)
        .await?;

        if existing_reputation.is_none() {
            return Err(Error::RowNotFound);
        }

        sqlx::query(
            r#"
            DELETE FROM user_message_reputation
            WHERE user_id = ? AND message_id = ? AND giver_id = ?;
            "#,
        )
        .bind(user_id)
        .bind(message_id)
        .bind(giver_id)
        .execute(&self.pool)
        .await?;

        self.update_global_reputation(user_id, -1).await?;

        Ok(())
    }

    async fn update_global_reputation(
        &self,
        user_id: i64,
        reputation_change: i64,
    ) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO user_global_reputation (user_id, global_reputation)
            VALUES (?, ?)
            ON CONFLICT(user_id) DO UPDATE
            SET global_reputation = global_reputation + ?
            "#,
        )
        .bind(user_id)
        .bind(reputation_change)
        .bind(reputation_change)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
