use maud::{html, Markup};
use crate::models::Post;
use super::layout;

pub fn index_page(posts: &[Post]) -> Markup {
    layout(
        "Home",
        None,
        html! {
            div class="space-y-8" {
                @for post in posts {
                    article class="bg-gruvbox-bg-s dark:bg-gruvbox-bg-s-dark rounded-lg p-6 shadow-md hover:shadow-lg transform transition duration-300 hover:-translate-y-1 border border-transparent hover:border-gruvbox-border-light dark:hover:border-gruvbox-border-dark" {
                        h2 class="text-xl font-bold mb-2" {
                            a href={"/posts/" (post.slug)} class="text-gruvbox-yellow dark:text-gruvbox-orange-bright hover:text-gruvbox-orange dark:hover:text-gruvbox-yellow-bright transition-colors duration-300" {
                                (post.meta.title)
                            }
                        }
                        div class="flex items-center space-x-3 mb-3" {
                            time class="text-sm font-medium text-gruvbox-fg dark:text-gruvbox-fg-dark" datetime=(post.meta.date.format("%Y-%m-%d")) {
                                (post.meta.date.format("%B %d, %Y"))
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
        },
    )
}
