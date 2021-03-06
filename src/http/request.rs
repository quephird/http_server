use std::convert::TryFrom;
use std::str;

use super::method::Method;
use super::parse_error::ParseError;
use super::query_parameters::QueryParameters;

#[derive(Debug)]
pub struct Request<'qp> {
    pub method: Method,
    pub path: &'qp str,
    pub query_params: Option<QueryParameters<'qp>>,
    headers: Vec<String>,
    body: Option<String>,
}        

fn parse_path_and_query_string(second_token: &str) -> (&str, Option<&str>) {
    let tokens: Vec<&str> = second_token.split("?").collect();
    return if tokens.len() < 2 {
        (tokens[0], None)
    } else {
        (tokens[0], Some(tokens[1]))
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

fn parse_first_line<'qp>(first_request_line: &'qp str) -> Result<(Method, &str, Option<QueryParameters<'qp>>, f32), ParseError> {
    let tokens: Vec<&str> = first_request_line.split_ascii_whitespace().collect();

    // The first line should _always_ have three components like:
    //
    //    GET /foo?bar=baz HTTP/1.1
    if tokens.len() < 3 {
        return Err(ParseError::InvalidRequest);
    }

    // This takes advantage of the FromStr trait for Method
    let method = tokens[0].parse()?;

    let (path, maybe_query_string) = parse_path_and_query_string(tokens[1]);

    let mut query_params: Option<QueryParameters<'qp>> = None;
    if maybe_query_string.is_some() {
        // This takes advantage of the From trait for QueryParameters
        query_params = Some(QueryParameters::from(maybe_query_string.unwrap()));
    }

    let http_version = parse_http_version(tokens[2]).unwrap();

    Ok((method, path, query_params, http_version))
}

impl<'qp> TryFrom<&'qp [u8]> for Request<'qp> {
    type Error = ParseError;

    fn try_from(buffer: &'qp [u8]) -> Result<Self, Self::Error> {
        let result = str::from_utf8(buffer);
        match result {
            Ok(request_str) => {
                let request_lines: Vec<&str> = request_str.split("\r\n").collect();
                let maybe_first_line = parse_first_line(request_lines[0]);
                if maybe_first_line.is_err() {
                    return Err(maybe_first_line.unwrap_err());
                }

                let (method, path, query_params, _http_version) = maybe_first_line.unwrap();
                Ok(
                    Self {
                        method: method,
                        path: path,
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
