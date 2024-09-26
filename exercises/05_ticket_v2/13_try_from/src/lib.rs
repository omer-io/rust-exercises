// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, thiserror::Error)]
#[error("{invalid_title} is invalid")]
struct TitleError {
    invalid_title: String,
}

impl TryFrom<String> for Status {
    type Error = TitleError;
    fn try_from(value: String) -> Result<Self, Self::Error>{
        value.as_str().try_into()
    }
}
impl TryFrom<&str> for Status {
    type Error = TitleError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.to_lowercase().as_str() == "todo" {
            return Ok(Status::ToDo);
        }
        if value.to_lowercase().as_str() == "inprogress" {
            return Ok(Status::InProgress);
        }
        if value.to_lowercase().as_str() == "done" {
            return Ok(Status::Done);
        }
        Err(TitleError{
            invalid_title: value.to_string()
        })
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
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
