use rocket::get;
use rocket::response::content::RawHtml;
use std::sync::Mutex;

use crate::models::Post;
use crate::templates::{about_page, contact_page, index_page, post_page};

#[get("/")]
pub fn index(posts: &rocket::State<Mutex<Vec<Post>>>) -> RawHtml<String> {
    let posts = posts.lock().unwrap();
    RawHtml(index_page(&posts).into_string())
}

#[get("/about")]
pub fn about() -> RawHtml<String> {
    RawHtml(about_page().into_string())
}

#[get("/contact")]
pub fn contact() -> RawHtml<String> {
    RawHtml(contact_page().into_string())
}

#[get("/posts/<slug>")]
pub fn posts(slug: &str, posts: &rocket::State<Mutex<Vec<Post>>>) -> Option<RawHtml<String>> {
    let posts = posts.lock().unwrap();
    posts
        .iter()
        .find(|p| p.slug == slug)
        .map(|post| RawHtml(post_page(post).into_string()))
}
