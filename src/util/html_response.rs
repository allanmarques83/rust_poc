use rocket::{response::content::RawHtml, http::Status};

pub type HtmlResponse = Result<RawHtml<String>, Status>;