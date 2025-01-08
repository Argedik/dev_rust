use sqlx::{SqlitePool, sqlite::SqliteQueryResult};
use crate::models::User;

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        // Tablomuz yoksa oluşturuyoruz:
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id   INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    // CREATE
    pub async fn create_user(&self, name: &str) -> Result<User, sqlx::Error> {
        let result: SqliteQueryResult = sqlx::query(
            r#"
            INSERT INTO users (name)
            VALUES (?)
            "#,
        )
        .bind(name)
        .execute(&self.pool)
        .await?;

        let last_id = result.last_insert_rowid();
        let user = User {
            id: last_id,
            name: name.to_string(),
        };
        Ok(user)
    }

    // READ
    pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name
            FROM users
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    // UPDATE
    pub async fn update_user(&self, id: i64, new_name: &str) -> Result<User, sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE users
            SET name = ?
            WHERE id = ?
            "#,
        )
        .bind(new_name)
        .bind(id)
        .execute(&self.pool)
        .await?;

        let updated_user = User {
            id,
            name: new_name.to_string(),
        };

        Ok(updated_user)
    }

    // DELETE
    pub async fn delete_user(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM users
            WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
