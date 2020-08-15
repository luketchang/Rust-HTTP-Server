use std::collections::HashMap;
use std::convert::From;

/* Struct: QueryString
 * ___________________
 *  - stored as a hashmap to account for different pieces of query string
 *  - mapping from buf string to Value type (either single buf string or array of buf strings)
 *  - example: a=1&b=2&c&d=&e&d=4...
 */
#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

/* Enum: Value
 * ___________
 *  - enum that of type Single for singular query string or Multiple for query string piece
 *    with multiple components
 */
#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}

/* Implementation: QueryString
 * ___________________________
 *  - Function: get
 *      - takes in reference to self and key string, returning hashmap value given key
 */
impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) {
        self.data.get(key);
    }
}

/* Implementation: From<&'buf str> for QueryString<'buf>
 * ___________________________
 *  - handles conversion from heap string to QueryString hashmap type
 *  - Function: from
 *      - takes in heap string and returns QueryString instance
 *      - creates new HashMap
 *      - loops through param string split by & and divides those substrings into key value pairs
 *      - enters key value pairs into HashMap, appending to value vector if key's corresponding value is of
 *         type Multiple and creating a new Multiple type QueryString value if originally of type Single
 */
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split("&") {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find("=") {
                key = &sub_str[..i];
                val = &sub_str[i+1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        *existing = Value::Multiple(vec![prev_val, val]);
                    }
                    Value::Multiple(vec) => vec.push(val)
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}