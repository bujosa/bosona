use crate::prelude::*;

#[derive(Debug)]
pub struct Request {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

#[derive(Default, Clone)]
pub struct RequestBuilder {
    url: Option<String>,
    method: Option<String>,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder::default()
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url.insert(url.into());
        self
    }
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method.insert(method.into());
        self
    }
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body.insert(body.into());
        self
    }
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    pub fn build(self) -> Result<Request> {
        let Some(url) = self.url.as_ref() else {
			return Err(Error::Static("No URL"));
		};

        let method = self.method.clone().unwrap_or_else(|| "GET".to_string());

        Ok(Request {
            url: url.to_string(),
            method,
            headers: self.headers.clone(), // That is necessary because of the ownership.
            body: self.body.clone(),
        })
    }
}

// This example is for understanding the builder pattern.
