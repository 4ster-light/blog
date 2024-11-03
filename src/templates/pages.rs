use crate::models::post::{Post, PostMeta};
use maud::{html, Markup, DOCTYPE};

const TITLE: &str = "✰λster✰";

pub fn layout(title: &str, meta: Option<&PostMeta>, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                @if let Some(meta) = meta {
                    meta name="description" content=(meta.description);
                    meta property="og:title" content=(meta.title);
                    meta property="og:description" content=(meta.description);
                    @if let Some(image) = &meta.image {
                        meta property="og:image" content=(image);
                    }
                }
                title { (title) " | " (TITLE) }
                link rel="stylesheet" href="/static/css/tailwind.css";
            }
            body class="min-h-screen flex flex-col bg-gruvbox-bg dark:bg-gruvbox-bg-dark text-gruvbox-fg dark:text-gruvbox-fg-dark transition-colors duration-200" {
                header class="border-b border-gruvbox-border-light dark:border-gruvbox-border-dark bg-gruvbox-bg-s dark:bg-gruvbox-bg-h-dark shadow-sm" {
                    nav class="max-w-3xl mx-auto px-6 py-6" {
                        div class="flex justify-between items-center" {
                            a href="/" class="text-2xl text-gruvbox-fg dark:text-gruvbox-fg-dark hover:text-gruvbox-green dark:hover:text-gruvbox-green-bright transform transition duration-200 hover:-translate-y-0.5" {
                                (TITLE)
                            }
                            div class="space-x-8" {
                                a href="/" class="text-gruvbox-gray dark:text-gruvbox-gray-l-dark hover:text-gruvbox-green dark:hover:text-gruvbox-green-bright relative after:absolute after:bottom-[-2px] after:left-0 after:h-0.5 after:w-0 after:bg-gruvbox-green-bright after:transition-all after:duration-300 hover:after:w-full" { "Home" }
                                a href="/about" class="text-gruvbox-gray dark:text-gruvbox-gray-l-dark hover:text-gruvbox-green dark:hover:text-gruvbox-green-bright relative after:absolute after:bottom-[-2px] after:left-0 after:h-0.5 after:w-0 after:bg-gruvbox-green-bright after:transition-all after:duration-300 hover:after:w-full" { "About" }
                                a href="/contact" class="text-gruvbox-gray dark:text-gruvbox-gray-l-dark hover:text-gruvbox-green dark:hover:text-gruvbox-green-bright relative after:absolute after:bottom-[-2px] after:left-0 after:h-0.5 after:w-0 after:bg-gruvbox-green-bright after:transition-all after:duration-300 hover:after:w-full" { "Contact" }
                            }
                        }
                    }
                }
                main class="flex-grow max-w-3xl mx-auto px-6 py-12 w-full" {
                    (content)
                }
                footer class="mt-auto border-t border-gruvbox-border-light dark:border-gruvbox-border-dark bg-gruvbox-bg-s dark:bg-gruvbox-bg-h-dark shadow-inner" {
                    div class="max-w-3xl mx-auto px-6 py-6 text-center text-gruvbox-gray dark:text-gruvbox-gray-l-dark" {
                        p { "© 2024 " (TITLE) }
                    }
                }
            }
        }
    }
}

pub fn index_page(posts: &[Post]) -> Markup {
    layout(
        "Home",
        None,
        html! {
            div class="space-y-12" {
                @for post in posts {
                    article class="bg-gruvbox-bg-s dark:bg-gruvbox-bg-s-dark rounded-lg p-8 shadow-md hover:shadow-lg transform transition duration-200 hover:-translate-y-0.5" {
                        h2 class="text-2xl mb-3" {
                            a href={"/posts/" (post.slug)} class="text-gruvbox-fg dark:text-gruvbox-fg-dark hover:text-gruvbox-green dark:hover:text-gruvbox-green-bright transition-colors duration-200" {
                                (post.meta.title)
                            }
                        }
                        div class="flex items-center space-x-4 mb-4" {
                            time class="text-sm text-gruvbox-gray dark:text-gruvbox-gray-l-dark" datetime=(post.meta.date.format("%Y-%m-%d")) {
                                (post.meta.date.format("%B %d, %Y"))
                            }
                            div class="flex flex-wrap gap-2" {
                                @for tag in &post.meta.tags {
                                    span class="inline-block bg-gruvbox-bg dark:bg-gruvbox-fg rounded-full px-3 py-1 text-sm text-gruvbox-fg dark:text-gruvbox-bg-dark hover:bg-gruvbox-green hover:text-gruvbox-bg dark:hover:bg-gruvbox-green-bright dark:hover:text-gruvbox-bg-dark transition-all duration-300" {
                                        (tag)
                                    }
                                }
                            }
                        }
                        p class="text-gruvbox-fg dark:text-gruvbox-border-light leading-relaxed" {
                            (post.meta.description)
                        }
                    }
                }
            }
        },
    )
}

pub fn post_page(post: &Post) -> Markup {
    layout(
        &post.meta.title,
        Some(&post.meta),
        html! {
            article {
                div class="mb-12 pb-8 border-b border-gruvbox-border-light dark:border-gruvbox-border-dark" {
                    h1 class="text-4xl mb-6 text-gruvbox-fg dark:text-gruvbox-fg-dark" {
                        (post.meta.title)
                    }
                    div class="flex flex-col space-y-4 sm:flex-row sm:items-center sm:justify-between sm:space-y-0" {
                        time class="text-lg text-gruvbox-gray dark:text-gruvbox-gray-l-dark" datetime=(post.meta.date.format("%Y-%m-%d")) {
                            (post.meta.date.format("%B %d, %Y"))
                        }
                        div class="flex flex-wrap gap-2" {
                            @for tag in &post.meta.tags {
                                span class="inline-block bg-gruvbox-bg dark:bg-gruvbox-fg rounded-full px-3 py-1.5 text-sm text-gruvbox-fg dark:text-gruvbox-bg-dark hover:bg-gruvbox-green hover:text-gruvbox-bg dark:hover:bg-gruvbox-green-bright dark:hover:text-gruvbox-bg-dark transition-all duration-300 shadow-sm" {
                                    (tag)
                                }
                            }
                        }
                    }
                }
                div class="prose dark:prose-invert prose-lg max-w-none" {
                    (maud::PreEscaped(post.render_content()))
                }
            }
        },
    )
}
