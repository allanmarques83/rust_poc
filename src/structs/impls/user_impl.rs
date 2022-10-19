use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand_core::OsRng;
use rocket_db_pools::Connection;
use sqlx::Acquire;
use std::error::Error;
use uuid::Uuid;
use rocket_db_pools::sqlx::{PgConnection};

use crate::enums::user_status::UserStatus;
use crate::util::pagination::{DEFAULT_LIMIT};
use crate::config::postgres::DBConnection;

use crate::structs::user::{User, NewUser};
use crate::util::util::clean_html;

impl User {
    pub async fn find(connection: &mut PgConnection, uuid: &str) -> Result<Self, Box<dyn Error>> {
        let parsed_uuid = Uuid::parse_str(uuid)?;
        let query_str = "SELECT * FROM users WHERE uuid = $1";

        Ok(sqlx::query_as::<_, Self>(query_str)
            .bind(parsed_uuid)
            .fetch_one(connection)
            .await?)
    }

    pub fn to_html_string(&self) -> String {
        format!(
            r#"<div><span class="label">UUID:</span>{uuid}</div>
            <div><span class="label">Username: </span>{username}</div>
            <div><span class="label">Email: </span>{email}</div>
            <div><span class="label">Description: </span>{description}</div>
            <div><span class="label">Status: </span>{status}</div>
            <div><span class="label">Created At: </span>{created_at}</div>
            <div><span class="label">Updated At: </span>{updated_at}</div>"#,
            uuid = self.uuid,
            username = self.username,
            email = self.email,
            description = self.description.as_ref().unwrap_or(&String::from("")),
            status = self.status.to_string(),
            created_at = self.created_at.0.to_rfc3339(),
            updated_at = self.updated_at.0.to_rfc3339(),
        )
    }

    pub async fn find_all(
        db: &mut Connection<DBConnection>,
        pagination: i64,
    ) -> Result<(Vec<Self>, i64), Box<dyn Error>> {
        let query_str = "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2";
        
        let connection = db.acquire().await?;

        let users = sqlx::query_as::<_, Self>(query_str)
        .bind(DEFAULT_LIMIT as i32)
        .bind(pagination)
        .fetch_all(connection)
        .await?;

        let new_pagination = Self::get_new_pagination(db, &pagination).await?;

        Ok((users, new_pagination))
    }

    async fn get_new_pagination(db: &mut Connection<DBConnection>, current_pagination: &i64) -> Result<i64, Box<dyn Error>> {
        let connection = db.acquire().await?;

        let q = sqlx::query!("SELECT COUNT(*) FROM users")
            .fetch_one(connection)
            .await?;

        let count: i64 = q.count.unwrap_or(0);

        if current_pagination.is_positive(){
            return Ok(current_pagination.clone());
        }
        
        let division = count/current_pagination > 0;

        if division {
            return Ok(current_pagination+1);
        }
        Ok(current_pagination.clone())
    }

    pub async fn create<'r>(
        connection: &mut PgConnection,new_user: &'r NewUser<'r>,
    ) -> Result<Self, Box<dyn Error>> {
        let uuid = Uuid::new_v4();
        let username = &(clean_html(new_user.username));
        let description = &(new_user.description.map(|desc| clean_html(desc)));
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(new_user.password.as_bytes(), &salt);

        if password_hash.is_err() {
            return Err("cannot create password hash".into());
        }

        let query_str = r#"INSERT INTO users (uuid,username,email,password_hash, description,status) VALUES($1, $2, $3, $4, $5, $6) RETURNING *"#;
        
        Ok(sqlx::query_as::<_, Self>(query_str)
            .bind(uuid)
            .bind(username)
            .bind(new_user.email)
            .bind(password_hash.unwrap().to_string())
            .bind(description)
            .bind(UserStatus::Inactive)
            .fetch_one(connection)
            .await?
        )
    }
}