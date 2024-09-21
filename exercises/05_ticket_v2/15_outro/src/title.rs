// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 characters.
//   Implement the traits required to make the tests pass too.

#[derive(Debug, Clone, PartialEq)]
pub struct TicketTitle(String);

impl TicketTitle {
    const MAX_LENGTH: usize = 50;
}
#[derive(Debug, thiserror::Error)]
#[error("{value}")]
pub struct ParseTitleError {
    value: String,
}

impl TryFrom<String> for TicketTitle {
    type Error = ParseTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = ParseTitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "" => Err(ParseTitleError {
                value: "The title cannot be empty".to_string(),
            }),
            v if v.len() > TicketTitle::MAX_LENGTH => Err(ParseTitleError {
                value: format!(
                    "The title cannot be longer than {} characters",
                    TicketTitle::MAX_LENGTH
                ),
            }),
            _ => Ok(TicketTitle(String::from(value))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(
            err.to_string(),
            "The title cannot be longer than 50 characters"
        );
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
