use ammonia::Builder;
use std::collections::hash_set::HashSet;

pub fn clean_html(src: &str) -> String {
    Builder::default()
    .tags(HashSet::new())
    .clean(src)
    .to_string()
}