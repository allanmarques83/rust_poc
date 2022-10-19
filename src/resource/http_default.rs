use rocket::{Request, catch, response::content::RawHtml};

#[catch(404)]
pub fn not_found(_: &Request) -> RawHtml<String> {
    let html = String::from("<html>not found!</html>");
    RawHtml(html)
}