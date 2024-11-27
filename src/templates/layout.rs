use super::TITLE;
use crate::models::PostMeta;
use maud::{html, Markup, DOCTYPE};

/// Renders the base layout
pub fn layout(title: &str, meta: Option<&PostMeta>, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";

                title { (title) " | " (TITLE) }

                (meta_tags(meta))
                (impots())

                style { "@view-transition { navigation: auto; }" }
            }

            body class="min-h-screen flex flex-col bg-gruvbox-bg dark:bg-gruvbox-bg-dark text-gruvbox-fg dark:text-gruvbox-fg-dark transition-colors duration-300" {
                header class="border-b border-gruvbox-border-light dark:border-gruvbox-border-dark bg-gruvbox-bg-s dark:bg-gruvbox-bg-h-dark shadow-sm" {
                    nav class="max-w-3xl mx-auto px-6 py-4" x-data="{ open: false }" {
                        div class="flex justify-between items-center" {
                            a href="/" class="text-2xl font-bold text-gruvbox-yellow dark:text-gruvbox-orange-bright hover:text-gruvbox-orange dark:hover:text-gruvbox-yellow-bright transform transition duration-300" {
                                (TITLE)
                            }
                            (hamburger_menu())
                            (nav_links_desktop())
                        }
                        (mobile_menu())
                    }
                }

                main class="flex-grow max-w-3xl mx-auto px-6 py-8 w-full" { (content) }

                footer class="mt-auto border-t border-gruvbox-border-light dark:border-gruvbox-border-dark bg-gruvbox-bg-s dark:bg-gruvbox-bg-h-dark shadow-sm" {
                    div class="max-w-3xl mx-auto px-6 py-4 text-center text-gruvbox-fg dark:text-gruvbox-fg-dark" {
                        p { "© 2024 " (TITLE) }
                    }
                }
            }
        }
    }
}

fn meta_tags(meta: Option<&PostMeta>) -> Markup {
    html! {
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
    }
}

fn impots() -> Markup {
    html! {
        // * Tailwind CSS
        link rel="stylesheet" href="/static/css/tailwind.css";
        // * Code Highlighting
        link rel="stylesheet" href="/static/css/code.css";
        // * Copy button
        link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/toolbar/prism-toolbar.min.css";

        // * AlpineJS
        script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.14.3/dist/cdn.min.js" {}
        // * Prism JS: Syntax Highlighting
        script defer src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-core.min.js" {}
        script defer src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/autoloader/prism-autoloader.min.js" {}
        // * Copy button
        script defer src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/toolbar/prism-toolbar.min.js" {}
        script defer src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/copy-to-clipboard/prism-copy-to-clipboard.min.js" {}
    }
}

fn hamburger_menu() -> Markup {
    html! {
        button
            class="sm:hidden text-gruvbox-fg dark:text-gruvbox-fg-dark focus:outline-none relative flex flex-col justify-center items-center space-y-1"
            aria-label="Toggle Menu"
            x-on:click="open = !open" {
            div
                class="w-6 h-0.5 bg-current rounded-md transform transition-transform duration-700 ease-in-out"
                x-bind:class="open ? 'rotate-45 translate-y-1.5' : ''" {}
            div
                class="w-6 h-0.5 bg-current rounded-md transform transition-opacity duration-700 ease-in-out"
                x-bind:class="open ? 'opacity-0' : ''" {}
            div
                class="w-6 h-0.5 bg-current rounded-md transform transition-transform duration-700 ease-in-out"
                x-bind:class="open ? '-rotate-45 -translate-y-1.5' : ''" {}
        }
    }
}

fn nav_links_desktop() -> Markup {
    html! {
        div class="hidden sm:flex space-x-10" {
            a href="/" class="text-gruvbox-fg dark:text-gruvbox-fg-dark hover:text-gruvbox-green dark:hover:text-gruvbox-green-bright relative group" {
                "Home"
                span
                    class="absolute left-0 bottom-0 w-0 h-0.5 bg-gruvbox-green-bright transition-all duration-300 group-hover:w-full" {}
            }
            a href="/about" class="text-gruvbox-fg dark:text-gruvbox-fg-dark hover:text-gruvbox-yellow dark:hover:text-gruvbox-yellow-bright relative group" {
                "About"
                span
                    class="absolute left-0 bottom-0 w-0 h-0.5 bg-gruvbox-yellow-bright transition-all duration-300 group-hover:w-full" {}
            }
            a href="/contact" class="text-gruvbox-fg dark:text-gruvbox-fg-dark hover:text-gruvbox-orange dark:hover:text-gruvbox-orange-bright relative group" {
                "Contact"
                span
                    class="absolute left-0 bottom-0 w-0 h-0.5 bg-gruvbox-orange-bright transition-all duration-300 group-hover:w-full" {}
            }
        }
    }
}

fn mobile_menu() -> Markup {
    html! {
        div
            id="mobile-menu"
            x-bind:class="open ? 'max-h-screen opacity-100 mt-6' : 'max-h-0 opacity-0'"
            class="sm:hidden overflow-hidden flex flex-col space-y-3 transition-all duration-700 ease-in-out" {
            a href="/" class="text-gruvbox-fg dark:text-gruvbox-fg-dark hover:text-gruvbox-green dark:hover:text-gruvbox-green-bright relative group px-4 py-2" {
                "Home"
                span
                    class="absolute left-0 bottom-0 w-0 h-0.5 bg-gruvbox-green-bright transition-all duration-300 group-hover:w-full" {}
            }
            a href="/about" class="text-gruvbox-fg dark:text-gruvbox-fg-dark hover:text-gruvbox-yellow dark:hover:text-gruvbox-yellow-bright relative group px-4 py-2" {
                "About"
                span
                    class="absolute left-0 bottom-0 w-0 h-0.5 bg-gruvbox-yellow-bright transition-all duration-300 group-hover:w-full" {}
            }
            a href="/contact" class="text-gruvbox-fg dark:text-gruvbox-fg-dark hover:text-gruvbox-orange dark:hover:text-gruvbox-orange-bright relative group px-4 py-2" {
                "Contact"
                span
                    class="absolute left-0 bottom-0 w-0 h-0.5 bg-gruvbox-orange-bright transition-all duration-300 group-hover:w-full" {}
            }
        }
    }
}
