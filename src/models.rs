use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostMeta {
    pub title: String,
    pub description: String,
    pub date: DateTime<Utc>,
    pub tags: Vec<String>,
    pub image: Option<String>,
}

pub struct Post {
    pub slug: String,
    pub content: String,
    pub meta: PostMeta,
}

impl Post {
    pub fn load(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path.join("index.md"))?;
        let meta_str = std::fs::read_to_string(path.join("meta.toml"))?;
        let meta: PostMeta = toml::from_str(&meta_str)?;
        
        Ok(Post {
            slug: path.file_name().unwrap().to_string_lossy().into_owned(),
            content,
            meta,
        })
    }

    pub fn render_content(&self) -> String {
        let mut options = pulldown_cmark::Options::empty();
        options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
        options.insert(pulldown_cmark::Options::ENABLE_TABLES);
        
        let parser = pulldown_cmark::Parser::new_ext(&self.content, options);
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);
        html
    }
}