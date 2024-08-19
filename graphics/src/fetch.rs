use std::future::Future;

use reqwest::{Response, Result};

pub struct Fetcher<'a> {
    href: &'a str,
}

impl<'a> Fetcher<'a> {
    pub fn new(href: &'a str) -> Self {
        Self { href }
    }

    pub fn fetch_as_bytes(&self, resource_path: &str) -> impl Future<Output = Result<Response>> {
        let url = format!("{}/{}", self.href, resource_path);
        reqwest::get(url)
    }
}
