use chrono::{DateTime, Utc};
use pulldown_cmark::{Options, Parser};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;

/// Represents the metadata of a post
#[derive(Serialize, Deserialize, Debug)]
pub struct PostMeta {
    pub title: String,
    pub description: String,
    pub date: String,
    #[serde(skip)]
    parsed_date: DateTime<Utc>,
    pub tags: Vec<String>,
    pub image: Option<String>,
}

impl PostMeta {
    /// Returns the post's date
    pub fn date(&self) -> &DateTime<Utc> {
        &self.parsed_date
    }
}

/// Represents a post
#[derive(Serialize, Deserialize)]
pub struct Post {
    pub slug: String,
    pub content: String,
    pub meta: PostMeta,
}

impl Post {
    /// Loads a post from a file path
    pub fn load(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        use std::fs::read_to_string;

        let content = read_to_string(&path)?;
        let slug = extract_slug(&path)?;

        let (frontmatter, markdown_content) = split_content(&content)?;
        let mut meta: PostMeta = serde_yaml::from_str(frontmatter.as_str())?;
        parse_date(&mut meta)?;

        Ok(Post {
            slug,
            content: markdown_content.to_string(),
            meta,
        })
    }

    /// Renders the post's content as HTML
    pub fn render_content(&self) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);

        let parser = Parser::new_ext(&self.content, options);
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);

        html
    }
}

/// Extracts the slug from a file path by using its filename (without extension)
fn extract_slug(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    path.file_stem()
        .ok_or_else(|| "Invalid file path: no filename".into())
        .map(|s| s.to_string_lossy().into_owned())
}

/// Splits the content into frontmatter and markdown content, handling multiple separator sections
fn split_content(content: &str) -> Result<(String, String), Box<dyn Error>> {
    let parts: Vec<&str> = content.split("---\n").collect();
    match parts.len() {
        len if len >= 3 => Ok((parts[1].to_string(), parts[2..].join("---\n"))),
        _ => Err("Invalid markdown format: missing frontmatter".into()),
    }
}

/// Parses the date string into a UTC DateTime, setting time to midnight
fn parse_date(meta: &mut PostMeta) -> Result<(), Box<dyn Error>> {
    match chrono::NaiveDate::parse_from_str(&meta.date, "%Y-%m-%d") {
        Ok(date) => {
            let datetime = date
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| "Failed to create datetime")?;
            meta.parsed_date = DateTime::from_naive_utc_and_offset(datetime, Utc);
            Ok(())
        }
        Err(_) => Err("Invalid date format. Expected YYYY-MM-DD".into()),
    }
}
