use macros::UntypedHeader;

pub use tokenizer::Tokenizer;

#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct From(String);

pub mod tokenizer {
    use crate::common::uri;
    use crate::headers::header::Tokenize;

    #[derive(Eq, PartialEq, Clone, Debug)]
    pub struct Tokenizer<'a> {
        pub display_name: Option<&'a str>,
        pub uri: uri::Tokenizer<'a>,
        pub params: Vec<uri::param::Tokenizer<'a>>,
    }

    impl<'a> Tokenize<'a> for Tokenizer<'a> {
        fn tokenize(part: &'a str) -> Result<Self, crate::Error> {
            use nom::{
                bytes::complete::{tag, take_until},
                combinator::rest,
                error::VerboseError,
                multi::many0,
                sequence::tuple,
            };

            if part.contains('<') {
                let (_, (display_name, _, uri, _, params)) = tuple::<_, _, VerboseError<&str>, _>(
                    (take_until("<"), tag("<"), take_until(">"), tag(">"), rest),
                )(part)?;

                Ok(Self {
                    display_name: crate::utils::opt_trim(display_name),
                    uri: uri::Tokenizer::tokenize(uri.as_bytes())?.1,
                    params: many0(uri::param::Tokenizer::tokenize)(params.as_bytes())?.1,
                })
            } else {
                let (_, (uri, params)) = tuple((
                    uri::Tokenizer::tokenize_without_params,
                    many0(uri::param::Tokenizer::tokenize),
                ))(part.as_bytes())?;

                Ok(Self {
                    display_name: None,
                    uri,
                    params,
                })
            }
        }
    }
}

pub mod typed {
    use super::Tokenizer;
    use crate::common::uri::param::Tag;
    use crate::common::{uri::Param, Uri};
    use macros::{TypedHeader, UriAndParamsHelpers};
    use std::convert::{TryFrom, TryInto};

    #[derive(TypedHeader, UriAndParamsHelpers, Eq, PartialEq, Clone, Debug)]
    pub struct From {
        pub display_name: Option<String>,
        pub uri: Uri,
        pub params: Vec<Param>,
    }

    impl<'a> TryFrom<Tokenizer<'a>> for From {
        type Error = crate::Error;

        fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
            Ok(From {
                display_name: tokenizer.display_name.map(Into::into),
                uri: tokenizer.uri.try_into()?,
                params: tokenizer
                    .params
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()?,
            })
        }
    }

    impl From {
        pub fn tag(&self) -> Option<&Tag> {
            self.params.iter().find_map(|param| match param {
                Param::Tag(tag) => Some(tag),
                _ => None,
            })
        }
    }

    impl std::fmt::Display for From {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self.display_name {
                Some(display_name) => write!(
                    f,
                    "{} <{}>{}",
                    display_name,
                    self.uri,
                    self.params
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join("")
                ),
                None => write!(
                    f,
                    "{}{}",
                    self.uri,
                    self.params
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join("")
                ),
            }
        }
    }

    impl std::convert::From<crate::common::Uri> for From {
        fn from(uri: crate::common::Uri) -> Self {
            Self {
                display_name: None,
                uri,
                params: Default::default(),
            }
        }
    }
}
