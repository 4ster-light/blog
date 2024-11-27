use super::layout;
use crate::{models::Post, templates::TITLE};
use maud::{html, Markup};

/// Renders the index page (home)
pub fn index_page(posts: &[Post]) -> Markup {
    layout(
        "Home",
        None,
        html! {
            (presentation_card())

            h1 class="text-4xl font-bold text-center text-gruvbox-gray dark:text-gruvbox-gray-dark mt-20" { "Blog Posts" }
            hr class="border-t border-gruvbox-border-light dark:border-gruvbox-border-dark mt-3 mb-5 font-mono" {}

            div class="space-y-8" {
                @for post in posts {
                    (post_card(post))
                }
            }
        },
    )
}

/// Renders the presentation card
fn presentation_card() -> Markup {
    html! {
        div class="bg-gruvbox-bg-s dark:bg-gruvbox-bg-s-dark rounded-lg p-6 shadow-md" {
            div class="flex flex-col sm:flex-row items-center sm:space-x-6 space-y-4 sm:space-y-0" {
                img src="/static/images/pfp.jpg" alt="profile picture" class="h-20 w-20 rounded-full sm:h-24 sm:w-24" {}
                div class="text-center sm:text-left" {
                    h1 class="text-2xl font-bold text-gruvbox-yellow dark:text-gruvbox-orange-bright hover:text-gruvbox-orange dark:hover:text-gruvbox-yellow-bright transform transition duration-300" {
                        a href="/about" { "David Vivar Bogónez" }
                    }
                    p class="text-gruvbox-fg dark:text-gruvbox-fg-dark" { "aka: " (TITLE) }
                }
            }
        }
    }
}

/// Renders a single post card
fn post_card(post: &Post) -> Markup {
    html! {
        a href={"/posts/" (post.slug)} class="block" {
            article class="bg-gruvbox-bg-s dark:bg-gruvbox-bg-s-dark rounded-lg p-4 sm:p-6 shadow-md hover:shadow-lg transform transition duration-300 hover:-translate-y-1 border border-transparent hover:border-gruvbox-border-light dark:hover:border-gruvbox-border-dark" {
                h2 class="text-lg sm:text-xl font-bold mb-2 text-gruvbox-yellow dark:text-gruvbox-orange-bright hover:text-gruvbox-orange dark:hover:text-gruvbox-yellow-bright transition-colors duration-300" {
                    (post.meta.title)
                }
                div class="flex flex-col sm:flex-row sm:items-center sm:space-x-3 mb-3 space-y-2 sm:space-y-0" {
                    time class="text-sm font-medium text-gruvbox-fg dark:text-gruvbox-fg-dark" datetime=(post.meta.date().format("%Y-%m-%d")) {
                        (post.meta.date().format("%B %d, %Y"))
                    }
                    div class="flex flex-wrap gap-1.5" {
                        @for tag in &post.meta.tags {
                            span class="inline-block bg-gruvbox-green/10 dark:bg-gruvbox-green-bright/10 text-gruvbox-green dark:text-gruvbox-green-bright rounded-full px-2 py-0.5 text-xs font-medium border border-gruvbox-green/20 dark:border-gruvbox-green-bright/20 hover:bg-gruvbox-green hover:text-gruvbox-bg dark:hover:bg-gruvbox-green-bright dark:hover:text-gruvbox-bg-dark transition-all duration-300 shadow-sm hover:shadow-md" {
                                (tag)
                            }
                        }
                    }
                }
                p class="text-gruvbox-fg dark:text-gruvbox-fg-dark leading-relaxed text-sm" {
                    (post.meta.description)
                }
            }
        }
    }
}
