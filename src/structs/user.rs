use rocket::form::FromForm;
use rocket_db_pools::sqlx::{FromRow};
use uuid::Uuid;

use crate::enums::user_status::UserStatus;
use crate::util::date_custom::DateCustom;


#[derive(Debug, FromRow, FromForm)]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub description: Option<String>,
    pub status: UserStatus,
    pub created_at: DateCustom,
    pub updated_at: DateCustom,
}

#[derive(Debug, FromForm)]
pub struct NewUser<'r> {
    #[field(validate = len(5..20).or_else(msg!("namecannot be empty")))]
    pub username: &'r str,
    pub email: &'r str,
    pub password: &'r str,
    #[field(validate = eq(self.password).or_else(msg!("password confirmation mismatch")))]
    pub password_confirmation: &'r str,
    #[field(default = "")]
    pub description: Option<&'r str>,
}