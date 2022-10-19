use rocket::form::FromForm;

use crate::util::date_custom::DateCustom;

pub const DEFAULT_LIMIT: usize = 10;

#[derive(FromForm)]
pub struct Pagination {
    pub next: DateCustom,
    pub limit: usize,
}