use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::{
        complete::{alphanumeric0, char},
        is_alphanumeric,
    },
    combinator::{map, opt},
    multi::{many0, separated_list0},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Surf<'a> {
    pub host: &'a str,
    pub path: Vec<&'a str>,
    pub query: HashMap<&'a str, &'a str>,
    pub fragment: Option<&'a str>,
}

fn is_letter_or_dot(c: char) -> bool {
    is_alphanumeric(c as u8) || c == '.'
}

fn parse_surf(input: &str) -> IResult<&str, Surf<'_>> {
    let protocol_parser = alt((tag("grid!"), tag("grid://")));
    let (input, _) = opt(protocol_parser)(input)?;
    let (input, host) = take_while(is_letter_or_dot)(input)?;

    let path_parser = many0(preceded(char('/'), alphanumeric0));
    let (input, path) = map(opt(path_parser), Option::unwrap_or_default)(input)?;

    let key_value = separated_pair(alphanumeric0, char('='), alphanumeric0);
    let key_value_list = separated_list0(char('&'), key_value);
    let query_parser = preceded(char('?'), key_value_list);
    let query_hash = map(query_parser, |q| q.into_iter().collect());
    let (input, query) = map(opt(query_hash), Option::unwrap_or_default)(input)?;

    let fragment_parser = preceded(char('#'), alphanumeric0);
    let (input, fragment) = opt(fragment_parser)(input)?;

    Ok((
        input,
        Surf {
            host,
            path,
            query,
            fragment,
        },
    ))
}

impl<'a> TryFrom<&'a str> for Surf<'a> {
    type Error = ();

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        use nom::error::Error;

        match parse_surf(s).finish() {
            Ok((_, surf)) => Ok(surf),
            Err(Error { input, code }) => {
                dbg!(input, code);
                Err(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Surf;
    use std::collections::HashMap;

    type SurfResult<'a> = Result<Surf<'a>, ()>;

    #[test]
    fn simple_surf() {
        let surf: SurfResult = "grid!example.com".try_into();

        assert_eq!(
            surf,
            Ok(Surf {
                host: "example.com",
                path: Vec::default(),
                query: HashMap::default(),
                fragment: None,
            })
        );
    }

    #[test]
    fn simple_surf_with_double_slash() {
        let surf: SurfResult = "grid://example.com".try_into();

        assert_eq!(
            surf,
            Ok(Surf {
                host: "example.com",
                path: Vec::default(),
                query: HashMap::default(),
                fragment: None,
            })
        );
    }

    #[test]
    fn parse_path() {
        let surf: SurfResult = "grid!example.com/with/a/path".try_into();

        assert_eq!(
            surf,
            Ok(Surf {
                host: "example.com",
                path: ["with", "a", "path"].into(),
                query: HashMap::default(),
                fragment: None,
            })
        );
    }

    #[test]
    fn parse_query_params() {
        let surf: SurfResult = "grid!example.com/with/a/path?key1=val1&key2=val2".try_into();

        assert_eq!(
            surf,
            Ok(Surf {
                host: "example.com",
                path: ["with", "a", "path"].into(),
                query: [("key1", "val1"), ("key2", "val2")].into(),
                fragment: None,
            })
        );
    }

    #[test]
    fn parse_fragments() {
        let surf: SurfResult =
            "grid!example.com/with/a/path?key1=val1&key2=val2#fragment".try_into();

        assert_eq!(
            surf,
            Ok(Surf {
                host: "example.com",
                path: ["with", "a", "path"].into(),
                query: [("key1", "val1"), ("key2", "val2")].into(),
                fragment: Some("fragment")
            })
        );
    }

    #[test]
    fn parse_with_no_protocol() {
        let surf: SurfResult = "example.com/with/a/path?key1=val1&key2=val2#fragment".try_into();

        assert_eq!(
            surf,
            Ok(Surf {
                host: "example.com",
                path: ["with", "a", "path"].into(),
                query: [("key1", "val1"), ("key2", "val2")].into(),
                fragment: Some("fragment")
            })
        );
    }
}
