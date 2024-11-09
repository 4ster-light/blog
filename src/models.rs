use chrono::{DateTime, Utc};
use pulldown_cmark::{Options, Parser};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub slug: String,
    pub content: String,
    pub meta: PostMeta,
}

impl Post {
    pub fn load(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(&path)?;
        let slug = extract_slug(&path)?;

        let (frontmatter, markdown_content) = split_content(&content)?;
        let mut meta: PostMeta = serde_yaml::from_str(frontmatter)?;
        parse_date(&mut meta)?;

        Ok(Post {
            slug,
            content: markdown_content.to_string(),
            meta,
        })
    }

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

impl PostMeta {
    pub fn date(&self) -> &DateTime<Utc> {
        &self.parsed_date
    }
}

fn extract_slug(path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    path.file_stem()
        .ok_or_else(|| "Invalid file path: no filename".into())
        .map(|s| s.to_string_lossy().into_owned())
}

fn split_content(content: &str) -> Result<(&str, &str), Box<dyn std::error::Error>> {
    let parts: Vec<&str> = content.split("---\n").collect();
    match parts.len() {
        len if len >= 3 => Ok((parts[1], parts[2])),
        _ => Err("Invalid markdown format: missing frontmatter".into()),
    }
}

fn parse_date(meta: &mut PostMeta) -> Result<(), Box<dyn std::error::Error>> {
    match chrono::NaiveDate::parse_from_str(&meta.date, "%Y-%m-%d") {
        Ok(date) => {
            // Convert the date to DateTime<Utc> by setting the time to midnight UTC
            let datetime = date
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| "Failed to create datetime")?;
            meta.parsed_date = DateTime::from_naive_utc_and_offset(datetime, Utc);
            Ok(())
        }
        Err(_) => Err("Invalid date format. Expected YYYY-MM-DD".into()),
    }
}
