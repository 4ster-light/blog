use models::Post;
use rocket::fs::FileServer;
use rocket::launch;
use std::sync::Mutex;
use std::io::Result;

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
        .mount("/", rocket::routes![
            routes::index,
            routes::about,
            routes::contact,
            routes::posts
        ])
        .mount("/static", FileServer::from("static"))
        .manage(posts)
}

/// Loads all posts from the content directory at server startup
fn load_posts() -> Result<Vec<Post>> {
    use std::path::PathBuf;
    use std::fs::read_dir;

    let content_dir = PathBuf::from("content/posts");

    println!("\nAttempting to load posts from: {:?}\n", content_dir);

    if !content_dir.exists() {
        eprintln!("Content directory does not exist!");
        return Ok(Vec::new());
    }

    let posts: Vec<Post> = read_dir(&content_dir)?
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
                match Post::load(path.clone()) {
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
