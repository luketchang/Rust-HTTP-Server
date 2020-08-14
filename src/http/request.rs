
use super::method:: { Method, MethodError };
use std::convert::TryFrom;
use std::error::Error;
use std::str;
use std::str::Utf8Error;
use std::fmt::{ Display, Debug, Formatter, Result as FmtResult };

/* Struct: Request
 * ______________________
 *  - request object which contains request path, query string, and method type
 *  - example: GET /search?user='me' HTTP/1.1
 */
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method
}

/* Implementation: Request 
 * _______________________
 *  
 */
impl Request {
    
}

/* Implementation: TryFrom<&[u8]> for Request
 * __________________________________________
 *  - conversion method from buffer (&[u8]) to Request object
 *  - returns custom ParseErrors
 *  - Function: try_from
 *      - converts buffer into string
 *      - calls get_next_word function three times, each time setting the specific needed 
 *        value and updating request with the truncated string
 *      - if protocol isn't HTTP/1.1, return error
 *      - convert method string into Method enum type using parse
 *      - convert path component into the url path and query string pieces
 */
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find("?") {
            query_string = Some(&path[i+1..]);
            path = &path[..i];
        }

        unimplemented!()
    }
}
/* Function: get_next_word
 * _______________________
 *  - loops through the characters of the current request form, returning tuple
 *    of (result, updated request) if space or return character hit
 */
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..]));
        }
    }
    None
}

/* Implementation: From<MethodError> for ParseError
 * ________________________________________________
 *  - implements conversion from MethodError (which can be returned when parse is called)
 *    to InvalidMethod ParseError
 */
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

/* Enum: ParseError
 * ________________
 *  - defines different possible errors when parsing received request
 */
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

/* Implementation: ParseError
 * __________________________
 *  - Function: message
 *      - takes reference to self as parameter and returns heap allocated string literal
 *      - matches ParseError type with error string
 */
impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

/* Implementation: From<Utf8Error> for ParseError
 * ______________________________________________
 *  - implements the conversion from Utf8Error to custom ParseError
 */
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

/* Implementation: Display for ParseError
 * ______________________________________
 *  - implements the formatting method fmt for displaying a ParseError in println
 */
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

/* Implementation: Debug for ParseError
 * ____________________________________
 *  - implements the formatting method fmt for displaying a ParseError in debug print
 */
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}