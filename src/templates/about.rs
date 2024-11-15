use super::layout;
use maud::{html, Markup};

/// Renders the about page (personal info)
pub fn about_page() -> Markup {
    layout(
        "About",
        None,
        html! {
            div class="prose dark:prose-invert lg:prose-lg mx-auto" {
                p { "Welcome to my blog! This is a simple blog built with Rust, Rocket, and Maud." }
                p {
                    "The blog posts are written in Markdown and stored in the content directory. "
                    "The site is styled using Tailwind CSS and supports both light and dark modes."
                }
            }
        },
    )
}
