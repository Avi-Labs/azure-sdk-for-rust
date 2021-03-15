use http::request::Request;

#[derive(Debug)]
pub struct BatchOperation {
    pub(crate) request: Request<String>,
}

impl BatchOperation {
    pub(crate) fn new(request: Request<String>) -> Self {
        Self { request }
    }
}
