use super::method::Method;

#[allow(unused)]
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}
