use rocket::fs::FileServer;
use rocket::launch;
use std::path::PathBuf;
use std::sync::Mutex;

mod models;
mod routes;
mod templates;

#[launch]
fn rocket() -> _ {
    let posts = match load_posts() {
        Ok(mut posts) => {
            // Sort posts by date (newest first)
            posts.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));
            Mutex::new(posts)
        }
        Err(e) => {
            eprintln!("Error loading posts: {}", e);
            Mutex::new(Vec::new())
        }
    };

    rocket::build()
        .mount(
            "/",
            rocket::routes![routes::index, routes::about, routes::contact, routes::posts,],
        )
        .mount("/static", FileServer::from("static"))
        .manage(posts)
}

fn load_posts() -> std::io::Result<Vec<models::Post>> {
    let content_dir = PathBuf::from("content/posts");
    let mut posts = Vec::new();

    println!();
    println!("Attempting to load posts from: {:?}", content_dir);

    if !content_dir.exists() {
        eprintln!("Content directory does not exist!");
        return Ok(Vec::new());
    }

    for entry in std::fs::read_dir(content_dir)? {
        println!();

        let entry = entry?;
        let path = entry.path();

        // Check if it's a .md file
        if entry.file_type()?.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            println!("Found markdown file at: {:?}", path);

            match models::Post::load(path.clone()) {
                Ok(post) => {
                    println!("Successfully loaded post: {}", post.meta.title);
                    posts.push(post);
                }
                Err(e) => {
                    eprintln!("Error loading post at {:?}: {}", path, e);
                }
            }
        }
    }

    println!();
    println!("Total posts loaded: {}", posts.len());
    println!();

    Ok(posts)
}
