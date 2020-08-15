use std::str::FromStr;

/* Enum: Method
 * ______________
 *  - enum for each method type
 */
#[derive(Debug)]
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

/* Implementation: FromStr for Method
 * ______________
 *  - conversion from string to method type
 *  - returns Ok(...) because the conversion returns a result 
 */
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