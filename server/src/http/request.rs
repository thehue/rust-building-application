use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

// lifetime specifier를 사용하기 위해서는 <>를 사용해서 Request를 제네릭하게 만들어야한다.
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<&'buf str>,
    method: Method,
}

// 'buf: Request가 buf라는 수명이 있다고 명시해주는 것

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    // TryFrom trait가 정의하는 기능을 구현하거나 추출하게 됨
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;

        // GET /search?name=abc&sort=1 HTTP/1.1\r\nHEADERS..
        match get_next_word(request) {
            Some((method, request)) => {}
            None => return Err(ParseError::InvalidRequest),
        }

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        // if문은 find()함수의 Result를 보고 제공한 패턴(some)에 매칭하려 하고, 변수 i에 있는 내용을 언래핑한다.
        if let Some(i) = path.find('?') {
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

// 첫번쨰는 공백전 첫번쨰 문자열, 두번째는 나머지 문자열
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (index, charactor) in request.chars().enumerate() {
        if charactor == ' ' || charactor == '\r' {
            return Some((&request[..index], &request[index + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvliadEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Requset",
            Self::InvliadEncoding => "Invliad Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvliadEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // self = ParseError
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // self = ParseError
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
