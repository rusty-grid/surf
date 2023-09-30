use nom::{
    bytes::complete::{tag, take_while},
    character::is_alphanumeric,
    combinator::map,
    multi::separated_list0,
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

fn is_letter(c: char) -> bool {
    is_alphanumeric(c as u8)
}

fn is_letter_or_dot(c: char) -> bool {
    is_alphanumeric(c as u8) || c == '.'
}

pub fn parse_surf(input: &str) -> IResult<&str, Surf<'_>> {
    let (input, _) = tag("grid!")(input)?;
    let (input, host) = take_while(is_letter_or_dot)(input)?;
    let (input, path) = map(
        separated_list0(tag("/"), take_while(is_letter)),
        |mut list| {
            list.remove(0);
            list
        },
    )(input)?;

    Ok((
        input,
        Surf {
            host,
            path,
            query: HashMap::default(),
            fragment: None,
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
}
