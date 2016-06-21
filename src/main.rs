extern crate hyper;
extern crate kuchiki;
extern crate liquid;

mod fixdom;
mod reddit;

use std::io::{Read, Write};
use std::str;
use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, SyncSender, Receiver, channel, sync_channel};

use hyper::Url;
use hyper::uri:: RequestUri;
use hyper::header::{Headers, AcceptEncoding, CookiePair, SetCookie, Cookie};
use hyper::client::Client;
use hyper::server::{Server, Request, Response, Handler};

use fixdom::fixdom;


fn get_url_in_query(url: &Url) -> Option<Url> {
    let find_res = url.query_pairs().find(|&(ref k, _)| k == "url");
    if let Some((_, qurl)) = find_res {
        let url_str = qurl.as_ref();
        Some(Url::parse(url_str).unwrap())
    } else {
        None
    }
}

struct WebypassHandler {
    cookie_sender: Mutex<Sender<(Url, Vec<CookiePair>)>>,
    cookie_requester: Mutex<SyncSender<Url>>,
    cookie_receiver: Mutex<Receiver<Option<Vec<CookiePair>>>>,
}

impl Handler for WebypassHandler {

    fn handle(&self, req: Request, mut res: Response) {
        let uri = req.uri.clone();
        match uri {
            RequestUri::AbsoluteUri(url) => {
                println!("URI: {}", url);
                if let Some(target_url) = get_url_in_query(&url) {
                    self.proxy(req, res, &target_url);
                    return;
                }
            },
            RequestUri::AbsolutePath(path) => {
                println!("Path {}", path);
                let url = Url::parse("http://localhost").unwrap().join(&path).unwrap();
                if let Some(target_url) = get_url_in_query(&url) {
                    self.proxy(req, res, &target_url);
                    return;
                }
            },
            _ => {
                println!("Unsupported URI type");
            }
        }
        *res.status_mut() = hyper::NotFound;
        res.send(b"Not Found").unwrap();
    }

}

impl WebypassHandler {
    fn proxy(&self, req: Request, mut res: Response, target_url: &Url) {
        let client = Client::new();
        self.cookie_requester.lock().unwrap().send(target_url.clone()).unwrap();
        let mut req_headers = req.headers.clone();
        req_headers.remove::<AcceptEncoding>();
        if let Some(cookies) = self.cookie_receiver.lock().unwrap().recv().unwrap() {
            req_headers.set(Cookie(cookies));
        }
        println!("debug {:?}", &req_headers);
        let mut proxied_res = client.request(req.method, target_url.clone()).headers(req_headers).send().unwrap();
        let mut buffer = Vec::new();
        let _ = proxied_res.read_to_end(&mut buffer);
        if let Some(&SetCookie(ref cookie_pairs)) = proxied_res.headers.get::<SetCookie>() {
            self.cookie_sender.lock().unwrap().send((target_url.clone(), cookie_pairs.clone())).unwrap();
        }
        {
            let h: &mut Headers = res.headers_mut();
            *h = proxied_res.headers.clone();
            println!("debug {:?}", &h);
            h.remove::<SetCookie>();
        }
        let mut res = res.start().unwrap();
        println!("debug {}", buffer.len());
        match fixdom(target_url, str::from_utf8(&buffer).unwrap()) {
            Some(fixed) => res.write_all(&fixed.as_bytes()).unwrap(),
            None => res.write_all(&buffer).unwrap(),
        };
    }
}

fn main() {
    let (cookie_sender_tx, cookie_sender_rx) = channel();
    let (cookie_requester_tx, cookie_requester_rx) = sync_channel(0);
    let (cookie_receiver_tx, cookie_receiver_rx) = sync_channel(0);
    let handler = WebypassHandler {
        cookie_sender: Mutex::new(cookie_sender_tx),
        cookie_requester: Mutex::new(cookie_requester_tx),
        cookie_receiver: Mutex::new(cookie_receiver_rx),
    };

    // {domain: {cookie_name: cookie}}
    let cookie_jar: HashMap<String, HashMap<String, CookiePair>> = HashMap::new();
    let cookie_jar_rc = Arc::new(Mutex::new(cookie_jar));

    {
        let jar_ref = cookie_jar_rc.clone();
        thread::spawn(move || {
            while let Ok((target_url, cookiepairs)) = cookie_sender_rx.recv() {
                let mut jar_handle = jar_ref.lock().unwrap();
                println!("Recv! {:?} {:?}", &target_url.domain(), &cookiepairs);
                let cookies = jar_handle.entry(target_url.domain().unwrap().to_owned()).or_insert_with(|| { HashMap::new() });
                for cookiepair in cookiepairs {
                    cookies.insert(cookiepair.name.clone(), cookiepair);
                }
            }
        });
    }

    {
        let jar_ref = cookie_jar_rc.clone();
        thread::spawn(move || {
            while let Ok(target_url) = cookie_requester_rx.recv() {
                println!("Request! {:?}", &target_url);
                let jar_handle = jar_ref.lock().unwrap();
                let maybe_cookies = jar_handle.get(target_url.domain().unwrap());
                let result = maybe_cookies.map(|cookies| cookies.values().map(|pair| pair.clone()).collect());
                cookie_receiver_tx.send(result).unwrap();
            }
        });
    }

    Server::http("127.0.0.1:8080").unwrap().handle(handler).unwrap();
}

