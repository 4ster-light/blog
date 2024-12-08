use axum::{routing::get, Router};
use std::io::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;

mod models;
mod routes;
mod templates;

#[tokio::main]
async fn main() -> Result<()> {
    let posts = match load_posts() {
        Ok(mut posts) => {
            // Sort posts by date (newest first)
            posts.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));
            Arc::new(Mutex::new(posts))
        }
        Err(e) => {
            eprintln!("Error loading posts: {}", e);
            Arc::new(Mutex::new(Vec::new()))
        }
    };

    let app = Router::new()
        .route("/", get(routes::index))
        .route("/about", get(routes::about))
        .route("/contact", get(routes::contact))
        .route("/posts/:slug", get(routes::posts))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(posts);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9112").await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

/// Loads all posts from the content directory at server startup
fn load_posts() -> Result<Vec<models::Post>> {
    use std::fs::read_dir;
    use std::path::PathBuf;

    let content_dir = PathBuf::from("content/posts");

    println!("\nAttempting to load posts from: {:?}\n", content_dir);

    if !content_dir.exists() {
        eprintln!("Content directory does not exist!");
        return Ok(Vec::new());
    }

    let posts: Vec<models::Post> = read_dir(&content_dir)?
        .filter_map(|entry| {
            let path = match entry {
                Ok(e) => e.path(),
                Err(e) => {
                    eprintln!("Error reading directory entry: {}", e);
                    return None;
                }
            };

            if path.extension().map_or(false, |ext| ext == "md") {
                println!("Found markdown file at: {:?}", path);
                match models::Post::load(path.clone()) {
                    Ok(post) => {
                        println!("Successfully loaded post: {}", post.meta.title);
                        Some(post)
                    }
                    Err(e) => {
                        eprintln!("Error loading post at {:?}: {}", path, e);
                        None
                    }
                }
            } else {
                None
            }
        })
        .collect();

    println!("\nTotal posts loaded: {}\n", posts.len());

    Ok(posts)
}
