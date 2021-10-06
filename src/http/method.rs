use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "PATCH" => Ok(Method::PATCH),
            "HEAD" => Ok(Method::HEAD),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
