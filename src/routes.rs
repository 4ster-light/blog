use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Response},
};
use std::sync::Arc;

use crate::models::Post;
use crate::templates::{about_page, contact_page, index_page, post_page};

pub async fn index(State(posts): State<Arc<Vec<Post>>>) -> Html<String> {
    Html(index_page(&posts).into_string())
}

pub async fn about() -> Html<String> {
    Html(about_page().into_string())
}

pub async fn contact() -> Html<String> {
    Html(contact_page().into_string())
}

pub async fn posts(State(posts): State<Arc<Vec<Post>>>, Path(slug): Path<String>) -> Response {
    posts
        .iter()
        .find(|p| p.slug == slug)
        .map(|post| Html(post_page(post).into_string()).into_response())
        .unwrap_or_else(|| (axum::http::StatusCode::NOT_FOUND, "Post not found").into_response())
}
