use std::path::PathBuf;
use rocket::fs::FileServer;
use std::sync::Mutex;
use rocket::launch;

mod models;
mod templates;
mod router;

use crate::models::post::Post;
use crate::router::routes;

#[launch]
fn rocket() -> _ {
    let posts = match load_posts() {
        Ok(mut posts) => {
            // Sort posts by date (newest first)
            posts.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));
            Mutex::new(posts)
        },
        Err(e) => {
            eprintln!("Error loading posts: {}", e);
            Mutex::new(Vec::new())
        }
    };

    rocket::build()
        .mount("/", rocket::routes![
            routes::index,
            routes::post,
            routes::about
        ])
        .mount("/static", FileServer::from("static"))
        .manage(posts)
}

fn load_posts() -> std::io::Result<Vec<Post>> {
    let content_dir = PathBuf::from("content/posts");
    let mut posts = Vec::new();
    
    println!("Attempting to load posts from: {:?}", content_dir);

    if !content_dir.exists() {
        eprintln!("Content directory does not exist!");
        return Ok(Vec::new());
    }
    
    for entry in std::fs::read_dir(content_dir)? {
        let entry = entry?;
        println!("Found entry: {:?}", entry.path());
        
        if entry.file_type()?.is_dir() {
            match Post::load(entry.path()) {
                Ok(post) => {
                    println!("Successfully loaded post: {}", post.meta.title);
                    posts.push(post);
                },
                Err(e) => {
                    eprintln!("Error loading post at {:?}: {}", entry.path(), e);
                }
            }
        }
    }
    
    println!("Total posts loaded: {}", posts.len());
    Ok(posts)
}