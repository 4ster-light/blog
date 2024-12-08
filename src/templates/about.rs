use super::layout;
use super::TITLE;
use maud::{html, Markup};

/// Renders the about page (personal info)
pub fn about_page() -> Markup {
    layout(
        "About",
        None,
        html! {
            div class="prose dark:prose-invert lg:prose-lg mx-auto" {
                h2 { "Who am I?" }
                p { "Hi I'm David Vivar Bogónez, most known at my socials as " (TITLE) ". I'm just a Spanish 17 year old who loves to code on his free time." }
                h3 { "My tech" }
                p { "I'm someone who always loves to try new technologies but I'm most comfortable with the following:" }
                ul {
                    li {
                        a href="https://rust-lang.org/" { "Rust" } ", "
                        a href="https://python.org/" { "Python" } ", "
                        a href="https://golang.org/" { "Go" } " and "
                        a href="https://typescriptlang.org/" { "TypeScript" }
                    }
                    li { a href="https://tailwindcss.com/" { "Tailwind CSS" } }
                    li { a href="https://htmx.org/" { "HTMX" } " and " a href="https://alpinejs.dev/" { "Alpine JS" } }
                }
                p { "I'm also a big fan and currently exploring the following:" }
                ul {
                    li { a href="https://ocaml.org/" { "OCaml" } }
                    li { a href="https://racket-lang.org/" { "Racket" } }
                    li { a href="https://www.haskell.org/" { "Haskell" } }
                    li { a href="https://nixos.org/" { "Nix and NixOS" } }
                    li { a href="https://webassembly.org/" { "WASM" } }
                }
                h3 { "About myself" }
                p { "I have some hobbies outside of programming too! I like to read books, watch movies, play video games and listen to music. I generally listen to artists like Bruno Mars and Dua Lipa but I like other genres like Brazilian Phonk too. My favourite saga of books and movies are Dune by Frank Herbert and my favourite video game is Super Smash Bros Ultimate. I also like to travel and explore new places :) ." }
                h3 { "This blog" }
                p { "This blog is a place where I write about my experiences and thoughts about programming, technology and life in general. I hope you enjoy it :) ." }
                p { "It is also quite special on the way it's built because it shows my taste for the tech stacks I use which in this case is:"}
                ul {
                    li { a href="https://www.rust-lang.org/" { "Rust" } " with the " a href="https://github.com/tokio-rs/axum" { "Axum" } " framework and the " a href="https://maud.lambda.xyz/" { "Maud" } " crate for templating" }
                    li { a href="https://tailwindcss.com/" { "Tailwind CSS" } " for styling" }
                    li { a href="https://alpinejs.dev/" { "Alpine JS" } " for UI interactions" }
                }
            }
        },
    )
}
