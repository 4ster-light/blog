use rocket::response::content::RawHtml;
use std::sync::Mutex;
use rocket::get;

use crate::models::Post;
use crate::templates::{index_page, layout, post_page};

#[get("/")]
pub fn index(posts: &rocket::State<Mutex<Vec<Post>>>) -> RawHtml<String> {
    let posts = posts.lock().unwrap();
    RawHtml(index_page(&posts).into_string())
}

#[get("/about")]
pub fn about() -> RawHtml<String> {
    RawHtml(layout(
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

#[get("/contact")]
pub fn contact() -> RawHtml<String> {
    RawHtml(layout(
        "Contact",
        None,
        maud::html! {
            div class="prose dark:prose-invert lg:prose-lg mx-auto" {
                h1 { "Contact" }
                p { "If you have any questions or feedback, please don't hesitate to get in touch!" }
                p { 
                    "You can reach me on any of the following platforms:"
                }
                ul {
                    li { a href="https://github.com/4ster-light" { "GitHub" } }
                    li { a href="https://twitter.com/4ster_light" { "Twitter" } }
                    li { a href="mailto:davidvivarbogonez@gmail.com" { "Email" } }
                }
            }
        },
    ).into_string())
}

#[get("/posts/<slug>")]
pub fn posts(slug: &str, posts: &rocket::State<Mutex<Vec<Post>>>) -> Option<RawHtml<String>> {
    let posts = posts.lock().unwrap();
    posts
        .iter()
        .find(|p| p.slug == slug)
        .map(|post| RawHtml(post_page(post).into_string()))
}
