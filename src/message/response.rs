pub use tokenizer::Tokenizer;

use crate::{
    common::{
        status_code::{self, StatusCode},
        version::{self, Version},
    },
    headers::{header, Headers},
    Error, SipMessage,
};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Response {
    pub status_code: StatusCode,
    pub version: Version,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Response {
    pub fn status_code(&self) -> &StatusCode {
        &self.status_code
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }

    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}\r\n{}\r\n{}",
            self.version,
            self.status_code,
            self.headers,
            String::from_utf8_lossy(&self.body)
        )
    }
}

impl TryFrom<SipMessage> for Response {
    type Error = &'static str;

    fn try_from(sip_message: crate::SipMessage) -> Result<Self, Self::Error> {
        match sip_message {
            crate::SipMessage::Request(_) => {
                Err("Can't convert a models::SipMessage::Response into Request !")
            }
            crate::SipMessage::Response(response) => Ok(response),
        }
    }
}

impl TryFrom<&[u8]> for Response {
    type Error = Error;

    fn try_from(from: &[u8]) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(from)?.1.try_into()
    }
}

impl TryFrom<Vec<u8>> for Response {
    type Error = Error;

    fn try_from(from: Vec<u8>) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(&from)?.1.try_into()
    }
}

impl TryFrom<&str> for Response {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(from.as_bytes())?.1.try_into()
    }
}

impl TryFrom<String> for Response {
    type Error = Error;

    fn try_from(from: String) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(&from.as_bytes())?.1.try_into()
    }
}

impl TryFrom<bytes::Bytes> for Response {
    type Error = Error;

    fn try_from(from: bytes::Bytes) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(&from)?.1.try_into()
    }
}

impl Into<String> for Response {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Into<Vec<u8>> for Response {
    fn into(self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl Into<bytes::Bytes> for Response {
    fn into(self) -> bytes::Bytes {
        bytes::Bytes::from(self.to_string())
    }
}

pub mod tokenizer {
    use super::{header, status_code, version, Response};
    use crate::{Error, NomError};
    use std::convert::TryInto;

    impl<'a> TryInto<Response> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<Response, Error> {
            Ok(Response {
                version: self.version.try_into()?,
                status_code: self.status_code.try_into()?,
                headers: self
                    .headers
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, Error>>()?
                    .into(),
                body: self.body.into(),
            })
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Tokenizer<'a> {
        pub version: version::Tokenizer<'a>,
        pub status_code: status_code::Tokenizer<'a>,
        pub headers: Vec<header::Tokenizer<'a>>,
        pub body: &'a [u8],
    }

    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
            use nom::{bytes::complete::tag, multi::many0, sequence::tuple};

            let (rem, (version, status_code)) = tuple((
                version::Tokenizer::tokenize,
                status_code::Tokenizer::tokenize,
            ))(part)?;
            let (rem, headers) = many0(header::Tokenizer::tokenize)(rem)?;
            let (body, _) = tag("\r\n")(rem)?;

            Ok((
                &[],
                Self {
                    version,
                    status_code,
                    headers,
                    body,
                },
            ))
        }
    }
}
