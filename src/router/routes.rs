use rocket::response::content::RawHtml;
use std::sync::Mutex;
use rocket::get;

use crate::models::post::Post;
use crate::templates::pages;

#[get("/")]
pub fn index(posts: &rocket::State<Mutex<Vec<Post>>>) -> RawHtml<String> {
    let posts = posts.lock().unwrap();
    RawHtml(pages::index_page(&posts).into_string())
}

#[get("/posts/<slug>")]
pub fn post(slug: &str, posts: &rocket::State<Mutex<Vec<Post>>>) -> Option<RawHtml<String>> {
    let posts = posts.lock().unwrap();
    posts
        .iter()
        .find(|p| p.slug == slug)
        .map(|post| RawHtml(pages::post_page(post).into_string()))
}

#[get("/about")]
pub fn about() -> RawHtml<String> {
    RawHtml(pages::layout(
        "About",
        None,
        maud::html! {
            div class="prose dark:prose-invert lg:prose-lg mx-auto" {
                h1 { "About" }
                p { "Welcome to my blog! This is a simple blog built with Rust, Rocket, and Maud." }
                p { 
                    "The blog posts are written in Markdown and stored in the content directory. "
                    "The site is styled using Tailwind CSS and supports both light and dark modes."
                }
            }
        },
    ).into_string())
}
