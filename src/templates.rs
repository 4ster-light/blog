mod about;
mod blog;
mod contact;
mod index;
mod layout;

pub use about::about_page;
pub use blog::post_page;
pub use contact::contact_page;
pub use index::index_page;
pub use layout::layout;

/// The title of the page
pub const TITLE: &str = "✰λster✰";
