use super::method::Method;

pub struct Request {
    method: Method,
    path: String,
    query_string: Option<String>,
    headers: Vec<String>,
    body: String,
}        
