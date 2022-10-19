use rocket_db_pools::{sqlx::{PgPool}, Database};

#[derive(Database)]
#[database("postgres_connection")]
pub struct DBConnection(PgPool);