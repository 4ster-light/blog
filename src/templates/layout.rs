use super::TITLE;
use crate::models::PostMeta;
use maud::{html, Markup, DOCTYPE};

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
                    } else {
                        meta property="og:image" content="/static/images/placeholder.webp";
                    }
                }
                title { (title) " | " (TITLE) }
                link rel="stylesheet" href="/static/css/tailwind.css";

                // Prism CSS
                link rel="stylesheet" href="/static/css/code.css";
                // Copy button
                link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/toolbar/prism-toolbar.min.css";
            }

            body class="min-h-screen flex flex-col bg-gruvbox-bg dark:bg-gruvbox-bg-dark text-gruvbox-fg dark:text-gruvbox-fg-dark transition-colors duration-300" {
                header class="border-b border-gruvbox-border-light dark:border-gruvbox-border-dark bg-gruvbox-bg-s dark:bg-gruvbox-bg-h-dark shadow-sm" {
                    nav class="max-w-3xl mx-auto px-6 py-4" {
                        div class="flex justify-between items-center" {
                            a href="/" class="text-2xl font-bold text-gruvbox-yellow dark:text-gruvbox-orange-bright hover:text-gruvbox-orange dark:hover:text-gruvbox-yellow-bright transform transition duration-300" {
                                (TITLE)
                            }
                            div class="space-x-8" {
                                a href="/" class="text-gruvbox-fg dark:text-gruvbox-fg-dark hover:text-gruvbox-green dark:hover:text-gruvbox-green-bright relative after:absolute after:bottom-[-2px] after:left-0 after:h-0.5 after:w-0 after:bg-gruvbox-green-bright after:transition-all after:duration-300 hover:after:w-full" { "Home" }
                                a href="/about" class="text-gruvbox-fg dark:text-gruvbox-fg-dark hover:text-gruvbox-yellow dark:hover:text-gruvbox-yellow-bright relative after:absolute after:bottom-[-2px] after:left-0 after:h-0.5 after:w-0 after:bg-gruvbox-yellow-bright after:transition-all after:duration-300 hover:after:w-full" { "About" }
                                a href="/contact" class="text-gruvbox-fg dark:text-gruvbox-fg-dark hover:text-gruvbox-orange dark:hover:text-gruvbox-orange-bright relative after:absolute after:bottom-[-2px] after:left-0 after:h-0.5 after:w-0 after:bg-gruvbox-orange-bright after:transition-all after:duration-300 hover:after:w-full" { "Contact" }
                            }
                        }
                    }
                }

                main class="flex-grow max-w-3xl mx-auto px-6 py-8 w-full" { (content) }

                footer class="mt-auto border-t border-gruvbox-border-light dark:border-gruvbox-border-dark bg-gruvbox-bg-s dark:bg-gruvbox-bg-h-dark shadow-sm" {
                    div class="max-w-3xl mx-auto px-6 py-4 text-center text-gruvbox-fg dark:text-gruvbox-fg-dark" {
                        p { "© 2024 " (TITLE) }
                    }
                }

                // Prism JS
                script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-core.min.js" {}
                script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/autoloader/prism-autoloader.min.js" {}
                // Copy button
                script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/toolbar/prism-toolbar.min.js" {}
                script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/copy-to-clipboard/prism-copy-to-clipboard.min.js" {}
            }
        }
    }
}
