use std::fmt;

#[derive(Debug, PartialEq)]
pub struct GraphError {
    msg: String,
}

impl GraphError {
    pub fn new(msg: &str) -> Self {
        GraphError { msg: msg.to_string() }
    }
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.msg)
    }
}
