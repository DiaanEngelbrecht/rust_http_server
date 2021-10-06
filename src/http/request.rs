use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResut};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buffer> {
    path: &'buffer str,
    query_string: Option<QueryString<'buffer>>,
    method: Method,
}

impl<'buffer> TryFrom<&'buffer [u8]> for Request<'buffer> {
    type Error = ParseError;

    fn try_from(buf: &'buffer [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocolError);
        }

        let method: Method = method.parse()?;
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncodingError,
    InvalidProtocolError,
    InvalidMethodError,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncodingError => "InvalidEncodingError",
            Self::InvalidProtocolError => "InvalidProtocolError",
            Self::InvalidMethodError => "InvalidMethodError",
        }
    }
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResut {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResut {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncodingError
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethodError
    }
}
