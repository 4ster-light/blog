use super::layout;
use crate::models::Post;
use maud::{html, Markup};

pub fn post_page(post: &Post) -> Markup {
    layout(
        &post.meta.title,
        Some(&post.meta),
        html! {
            article {
                div class="mb-8 pb-6 border-b border-gruvbox-border-light dark:border-gruvbox-border-dark" {
                    img class="w-full rounded-lg shadow-lg mb-10" src=(
                        if let Some(img) = &post.meta.image {
                            img
                        } else {
                            "/static/images/placeholder.png"
                        }
                    ) alt="Post image" {}

                    h1 class="text-4xl font-bold mb-4 text-gruvbox-yellow dark:text-gruvbox-orange-bright" {
                        (post.meta.title)
                    }

                    div class="flex flex-col space-y-3 sm:flex-row sm:items-center sm:justify-between sm:space-y-0" {
                        time class="text-base font-medium text-gruvbox-fg dark:text-gruvbox-fg-dark" datetime=(post.meta.date().format("%Y-%m-%d")) {
                            (post.meta.date().format("%B %d, %Y"))
                        }

                        div class="flex flex-wrap gap-1.5" {
                            @for tag in &post.meta.tags {
                                span class="inline-block bg-gruvbox-yellow/5 dark:bg-gruvbox-yellow-bright/5 text-gruvbox-yellow dark:text-gruvbox-yellow-bright rounded-full px-2.5 py-1 text-xs font-medium border border-gruvbox-yellow/10 dark:border-gruvbox-yellow-bright/10 hover:bg-gruvbox-yellow hover:text-gruvbox-bg dark:hover:bg-gruvbox-yellow-bright dark:hover:text-gruvbox-bg-dark transition-all duration-300 shadow-sm hover:shadow-md" {
                                    (tag)
                                }
                            }
                        }
                    }
                }

                div class="prose dark:prose-invert prose-base max-w-none" {
                    (maud::PreEscaped(post.render_content()))
                }
            }
        },
    )
}
