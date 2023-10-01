use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::{is_alphanumeric, complete::alphanumeric0},
    combinator::{map, opt},
    multi::{separated_list0, many0},
    sequence::{preceded, separated_pair},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Surf<'a> {
    host: &'a str,
    path: Vec<&'a str>,
    query: HashMap<&'a str, &'a str>,
    fragment: Option<&'a str>,
}

fn is_letter_or_dot(c: char) -> bool {
    is_alphanumeric(c as u8) || c == '.'
}

pub fn parse_surf(input: &str) -> IResult<&str, Surf<'_>> {
    let protocol_parser = alt((tag("grid!"), tag("grid://")));
    let (input, _) = opt(protocol_parser)(input)?;
    let (input, host) = take_while(is_letter_or_dot)(input)?;

    let path_parser = many0(preceded(tag("/"), alphanumeric0));
    let (input, path) = map(opt(path_parser), Option::unwrap_or_default)(input)?;

    let key_value = separated_pair(alphanumeric0, tag("="), alphanumeric0);
    let key_value_list = separated_list0(tag("&"), key_value);
    let query_parser = preceded(tag("?"), key_value_list);
    let query_hash = map(query_parser, |q| q.into_iter().collect());
    let (input, query) = map(opt(query_hash), Option::unwrap_or_default)(input)?;

    let fragment_parser = preceded(tag("#"), alphanumeric0);
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

#[cfg(test)]
mod tests {
    use super::{parse_surf, Surf};
    use std::collections::HashMap;

    #[test]
    fn simple_surf() {
        let (_, surf) = parse_surf("grid!example.com").unwrap();

        assert_eq!(
            surf,
            Surf {
                host: "example.com",
                path: Vec::default(),
                query: HashMap::default(),
                fragment: None,
            }
        );
    }

    #[test]
    fn simple_surf_with_double_slash() {
        let (_, surf) = parse_surf("grid://example.com").unwrap();

        assert_eq!(
            surf,
            Surf {
                host: "example.com",
                path: Vec::default(),
                query: HashMap::default(),
                fragment: None,
            }
        );
    }

    #[test]
    fn parse_path() {
        let (_, surf) = parse_surf("grid!example.com/with/a/path").unwrap();

        assert_eq!(
            surf,
            Surf {
                host: "example.com",
                path: ["with", "a", "path"].into(),
                query: HashMap::default(),
                fragment: None,
            }
        );
    }

    #[test]
    fn parse_query_params() {
        let (_, surf) = parse_surf("grid!example.com/with/a/path?key1=val1&key2=val2").unwrap();

        assert_eq!(
            surf,
            Surf {
                host: "example.com",
                path: ["with", "a", "path"].into(),
                query: [("key1", "val1"), ("key2", "val2")].into(),
                fragment: None,
            }
        );
    }

    #[test]
    fn parse_fragments() {
        let (_, surf) =
            parse_surf("grid!example.com/with/a/path?key1=val1&key2=val2#fragment").unwrap();

        assert_eq!(
            surf,
            Surf {
                host: "example.com",
                path: ["with", "a", "path"].into(),
                query: [("key1", "val1"), ("key2", "val2")].into(),
                fragment: Some("fragment")
            }
        );
    }

    #[test]
    fn parse_with_no_protocol() {
        let (_, surf) = parse_surf("example.com/with/a/path?key1=val1&key2=val2#fragment").unwrap();

        assert_eq!(
            surf,
            Surf {
                host: "example.com",
                path: ["with", "a", "path"].into(),
                query: [("key1", "val1"), ("key2", "val2")].into(),
                fragment: Some("fragment")
            }
        );
    }
}
