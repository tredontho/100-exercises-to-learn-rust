// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug)]
pub enum TicketStatusError {
    TicketStatusInvalid,
}

impl std::fmt::Display for TicketStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "whatever")
    }
}

impl TryFrom<String> for Status {
    type Error = TicketStatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case("todo") {
            Ok(Status::ToDo)
        } else if value.eq_ignore_ascii_case("inprogress") {
            Ok(Status::InProgress)
        } else if value.eq_ignore_ascii_case("done") {
            Ok(Status::Done)
        } else {
            Err(TicketStatusError::TicketStatusInvalid)
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = TicketStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(String::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
