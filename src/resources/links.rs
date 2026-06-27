//! Official documentation URLs for each quest.
//!
//! Primary links use doc.rust-lang.org (stable). YouTube links may change over time.

#[derive(Debug, Clone, Copy)]
pub struct ResourceLinks {
    pub book: &'static str,
    pub rust_by_example: &'static str,
    pub std_docs: Option<&'static str>,
    pub reference: Option<&'static str>,
    pub youtube: &'static [&'static str],
}

pub fn open_url(url: &str) {
    match opener::open(url) {
        Ok(()) => println!("Opened: {url}"),
        Err(_) => {
            println!("Could not open browser. Copy this URL:");
            println!("{url}");
        }
    }
}
