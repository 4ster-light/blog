mod blog;
mod index;
mod layout;
mod about;
mod contact;

pub use blog::post_page;
pub use index::index_page;
pub use layout::layout;
pub use about::about_page;
pub use contact::contact_page;

/// The title of the page
pub const TITLE: &str = "✰λster✰";
