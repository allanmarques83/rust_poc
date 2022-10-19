use rocket::{get,post,put,patch,delete, http::Status};
use rocket_db_pools::{sqlx::Acquire, Connection};
use rocket::form::{Form, Contextual};
use rocket::request::FlashMessage;

use rocket::response::{content::RawHtml, Flash, Redirect};

use crate::{
    config::postgres::DBConnection,
    util::{html_response::HtmlResponse},
    structs::user::{User, NewUser}
};

const USER_HTML_PREFIX: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8" />
<title>Our Application User</title>
</head>
<body>
"#;

const USER_HTML_SUFFIX: &str = r#"</body></html>"#;

#[get("/users/<uuid>")]
pub async fn get_user(
    mut db: Connection<DBConnection>,
    uuid: &str,
    flash: Option<FlashMessage<'_>>,
) -> HtmlResponse {
    let connection = db.acquire().await.map_err(|_| Status::InternalServerError)?;
    let user = User::find(connection, uuid).await.map_err(|_| Status::NotFound)?;
    
    let mut html_string = String::from(USER_HTML_PREFIX);

    if flash.is_some() {
        html_string.push_str(flash.unwrap().message());
    }

    html_string.push_str(&user.to_html_string());
    html_string.push_str(format!(r#"<a href="/users/edit/{}">Edit User</a>"#,user.uuid).as_ref());
    html_string.push_str(r#"<a href="/users">UserList</a>"#);
    html_string.push_str(USER_HTML_SUFFIX);

    Ok(RawHtml(html_string))
}

#[get("/users?<pagination>", format = "text/html")]
pub async fn get_users(
    mut db: Connection<DBConnection>, pagination: Option<i64>
) -> HtmlResponse {
    let (users, new_pagination) = User::find_all(
        &mut db, pagination.unwrap_or(0)
    ).await.map_err(|_| Status::NotFound)?;

    let mut html_string = String::from(USER_HTML_PREFIX);

    for user in users.iter() {
        html_string.push_str(&user.to_html_string());
        html_string.push_str(format!(r#"<a href="/users/{}">SeeUser</a><br/>"#, user.uuid).as_ref());
        html_string.push_str(
            format!(r#"<a href="/users/edit/{}">EditUser</a><br/>"#, user.uuid).as_ref(),
        );
    }

    html_string.push_str(
    format!(
            r#"<a href="/users?pagination={}">Next</a><br/>"#,
            &new_pagination,
        ).as_ref()
    );

    html_string.push_str(r#"<a href="/users/new">New user</a>"#);
    html_string.push_str(USER_HTML_SUFFIX);
    Ok(RawHtml(html_string))
}

#[get("/users/new", format = "text/html")]
pub async fn new_user(mut _db: Connection<DBConnection>) -> HtmlResponse {
    todo!("will implement later");
}

#[post("/users", format = "application/x-www-form-urlencoded", data = "<user_context>")]
pub async fn create_user<'r>(
    mut db: Connection<DBConnection>, user_context: Form<Contextual<'r, NewUser<'r>>>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    if user_context.value.is_none() {
        return Err(Flash::error(Redirect::to("/users/new"), "error_message".to_string()));
    }

    let new_user = user_context.value.as_ref().unwrap();

    let connection = db.acquire().await.map_err(|_| {
        Flash::error(
            Redirect::to("/users/new"),
            "<div>Something went wrong when creating user</div>",
        )
    })?;

    let user = User::create(connection, new_user).await.map_err(|_| {
        Flash::error(
            Redirect::to("/users/new"),
            "<div>Something went wrong when creating user</div>",
        )
    })?;

    Ok(Flash::success(
    Redirect::to(format!("/users/{}", user.uuid)),
    "<div>Successfully created user</div>",
    ))
}

#[get("/users/edit/<_uuid>", format = "text/html")]
pub async fn edit_user(mut _db: Connection<DBConnection>, _uuid:&str) -> HtmlResponse {
    todo!("will implement later")
}

#[put("/users/<_uuid>", format = "text/html", data = "<_user>")]
pub async fn put_user(
    mut _db: Connection<DBConnection>, _uuid: &str, _user: Form<User>
) -> HtmlResponse {
    todo!("will implement later")
}

#[patch("/users/<_uuid>", format = "text/html", data = "<_user>")]
pub async fn patch_user(
    mut _db: Connection<DBConnection>,
    _uuid: &str,
    _user: Form<User>,
) -> HtmlResponse {
    todo!("will implement later")
}

#[delete("/users/<_uuid>", format = "text/html")]
pub async fn delete_user(mut _db: Connection<DBConnection>, _uuid: &str) -> HtmlResponse {
    todo!("will implement later")
}