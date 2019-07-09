use crate::Path;

#[derive(Debug, Fail)]
pub enum AddViewError {
    #[fail(display = "invalid path given: {:?}", path)]
    InvalidPath {
        path: Path,
    },
    #[fail(display = "some error occured")]
    GenericError {},
}

impl std::convert::From<failure::Error> for AddViewError {
    fn from(_error: failure::Error) -> Self {
        AddViewError::GenericError{}
    }
}
