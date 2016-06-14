use std::io::{Read, Write};

use hyper::uri:: RequestUri;
use hyper::header::Headers;
use hyper::client::Client;
use hyper::server::{Server, Request, Response};

extern crate hyper;

fn hello(req: Request, mut res: Response) {
    if let RequestUri::AbsoluteUri(url) = req.uri {
        let client = Client::new();
        let mut proxied_res = client.request(req.method, url).headers(req.headers).send().unwrap();
        let mut buffer = Vec::new();
        let _ = proxied_res.read_to_end(&mut buffer);
        {
            let h: &mut Headers = res.headers_mut();
            *h = proxied_res.headers.clone();
        }
        let mut res = res.start().unwrap();
        res.write_all(&buffer).unwrap();
    }
}

fn main() {
    Server::http("127.0.0.1:8080").unwrap().handle(hello).unwrap();
}
