use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str;

use super::method::Method;
use super::query_parameters::QueryParameters;

#[derive(Debug)]
pub struct Request<'qp> {
    method: Method,
    path: String,
    query_params: Option<QueryParameters<'qp>>,
    headers: Vec<String>,
    body: Option<String>,
}        

impl<'qp> Request<'qp> {
    fn parse_path_and_query_string(second_token: &str) -> (String, Option<&str>) {
        let tokens: Vec<&str> = second_token.split("?").collect();
        return if tokens.len() < 2 {
            (tokens[0].to_string(), None)
        } else {
            (tokens[0].to_string(), Some(tokens[1]))
        };
    }

    fn parse_http_version(third_token: &str) -> Result<f32, ParseError> {
        if !third_token.starts_with("HTTP/") || third_token.len() < 6 {
            return Err(ParseError::InvalidRequest);
        }

        let (_, maybe_http_version) = third_token.split_at(5);
        match maybe_http_version.parse::<f32>() {
            Err(_) => Err(ParseError::InvalidHTTPVersion),
            Ok(http_version) =>
                if http_version < 1.1 {
                    Err(ParseError::InvalidHTTPVersion)
                } else {
                    Ok(http_version)
                }
        }
    }

    fn parse_first_line(first_request_line: &'qp str) -> Result<(Method, String, Option<QueryParameters<'qp>>, f32), ParseError> {
        let tokens: Vec<&str> = first_request_line.split_ascii_whitespace().collect();

        // The first line should _always_ have three components like:
        //
        //    GET /foo?bar=baz HTTP/1.1
        if tokens.len() < 3 {
            return Err(ParseError::InvalidRequest);
        }

        // This takes advantage of the FromStr trait for Method
        let method = tokens[0].parse()?;

        let (path, maybe_query_string) = Self::parse_path_and_query_string(tokens[1]);

        let mut query_params: Option<QueryParameters<'qp>> = None;
        if maybe_query_string.is_some() {
            // This takes advantage of the From trait for QueryParameters
            query_params = Some(QueryParameters::from(maybe_query_string.unwrap()));
        }

        let http_version = Self::parse_http_version(tokens[2]).unwrap();

        Ok((method, path.to_string(), query_params, http_version))
    }
}

impl<'qp> TryFrom<&'qp [u8]> for Request<'qp> {
    type Error = ParseError;

    fn try_from(buffer: &'qp [u8]) -> Result<Self, Self::Error> {
        let result = str::from_utf8(buffer);
        match result {
            Ok(request_str) => {
                let request_lines: Vec<&str> = request_str.split("\r\n").collect();
                let maybe_first_line = Self::parse_first_line(request_lines[0]);
                if maybe_first_line.is_err() {
                    return Err(maybe_first_line.unwrap_err());
                }

                let (method, path, query_params, _http_version) = maybe_first_line.unwrap();
                Ok(
                    Self {
                        method: method,
                        path: path.to_string(),
                        query_params: query_params,
                        headers: vec![],
                        body: None,
                    }
                )
            },
            Err(_) => Err(ParseError::InvalidUTF8Encoding),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidUTF8Encoding,
    InvalidHTTPVersion,
    InvalidHTTPMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid request!",
            Self::InvalidUTF8Encoding => "Invalid UTF8 encoding!",
            Self::InvalidHTTPVersion => "Invalid HTTP version sent!",
            Self::InvalidHTTPMethod => "Invalid HTTP method requested!",
        }
    }
}

// This is required to satisfy the Error trait below
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.message())
    }
}

// Empty implementation
impl Error for ParseError {}