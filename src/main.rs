#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use rocket::{Build, Rocket};
use rocket_02::config::postgres::DBConnection;
use rocket_02::resource::user_resource;
use rocket_02::resource::http_default;
use rocket_db_pools::Database;


#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv().ok();

    rocket::build()
        .attach(DBConnection::init())
        .mount("/", routes![
            user_resource::get_user,
            user_resource::get_users,
            user_resource::new_user,
            user_resource::create_user,
            user_resource::edit_user,
            user_resource::put_user,
            user_resource::patch_user,
            user_resource::delete_user,
        ])
        .register("/", catchers![
            http_default::not_found
        ])
}
