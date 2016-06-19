use hyper::Url;
use kuchiki;
use kuchiki::NodeRef;
use kuchiki::traits::*;

use reddit;

enum FixType {
    Reddit,
}

/// Returns fixed HTML if there's a proper filter for it.
///
/// If there's no available filter, returns None.
pub fn fixdom(url: &Url, html: &str) -> Option<String> {
    let doc: NodeRef = kuchiki::parse_html().one(html);
    let fixtype = match url.domain() {
        Some("www.reddit.com") => Some(FixType::Reddit),
        _ => None,
    };
    match fixtype {
        Some(FixType::Reddit) => Some(reddit::fix(&doc)),
        None => None,
    }
}
