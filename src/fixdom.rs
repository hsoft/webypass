use hyper::Url;

use reddit;

/// Returns fixed HTML if there's a proper filter for it.
///
/// If there's no available filter, returns None.
pub fn fixdom(url: &Url, html: &str) -> Option<String> {
    match url.domain() {
        Some("www.reddit.com") => Some(reddit::fix(html)),
        _ => None,
    }
}
