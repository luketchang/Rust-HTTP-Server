use std::fmt::{ Display, Debug, Formatter, Result as FmtResult };

/* Enum: StatusCode
 * ________________
 *  - enum representing status codes
 */
#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404
}

/* Implementation: StatusCode
 * __________________________
 *  - Function: message
 *      - takes in reference to self and matches type with status message
 */
impl StatusCode {
    pub fn message(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found"
        }
    }
}

/* Implementation: Display for StatusCode
 * ______________________________________
 *  - Function: fmt
 *      - formats StatusCode object as u16 to be used for print and debug
 */
impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}