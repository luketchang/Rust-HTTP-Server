use std::str::FromStr;

pub enum Method {
    GET,
    DELETE,
    PUT,
    POST,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATH
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "PUT" => Ok(Self::PUT),
            "POST" => Ok(Self::POST),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATH" => Ok(Self::PATH),
            _ => Err(MethodError)
        }
    }
}

pub struct MethodError;