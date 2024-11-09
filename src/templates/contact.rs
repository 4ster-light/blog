use super::layout;
use maud::{html, Markup};

pub fn contact_page() -> Markup {
    layout(
        "Contact",
        None,
        html! {
            div class="prose dark:prose-invert lg:prose-lg mx-auto" {
                p { "If you have any questions or feedback, please don't hesitate to get in touch!" }
                p { "You can reach me on any of the following platforms:" }
                ul {
                    li { a href="https://twitter.com/4ster_light" { "Twitter / X" } }
                    li { a href="https://github.com/4ster-light" { "GitHub" } }
                    li { a href="https://bsky.app/profile/4ster-light.bsky.social" { "BlueSky" } }
                    li { a href="mailto:davidvivarbogonez@gmail.com" { "Email" } }
                }
                p { "I may take some time to answer mails, but my DMs are always open in all of my socials." }
            }
        },
    )
}
