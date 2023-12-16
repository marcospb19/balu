use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Method {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Trace,
}

#[derive(Debug)]
pub struct MethodParseError;

impl FromStr for Method {
    type Err = MethodParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(match text {
            "GET" => Self::Get,
            "POST" => Self::Post,
            "DELETE" => Self::Delete,
            "PUT" => Self::Put,
            "OPTIONS" => Self::Options,
            "CONNECT" => Self::Connect,
            "HEAD" => Self::Head,
            "PATCH" => Self::Patch,
            "TRACE" => Self::Trace,
            _ => return Err(MethodParseError),
        })
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method_name = match self {
            Self::Connect => "CONNECT",
            Self::Delete => "DELETE",
            Self::Get => "GET",
            Self::Head => "HEAD",
            Self::Options => "OPTIONS",
            Self::Patch => "PATCH",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Trace => "TRACE",
        };

        write!(f, "{method_name}")
    }
}
