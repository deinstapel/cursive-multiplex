use crate::{Path, Id};

#[derive(Debug, Fail)]
pub enum AddViewError {
    #[fail(display = "invalid path given: {:?}", path)]
    InvalidPath {
        path: Path,
    },
    #[fail(display = "some error occured")]
    GenericError {},
}

#[derive(Debug, Fail)]
pub enum SearchPathError {
    #[fail(display = "some error occured")]
    GenericError {},
}

#[derive(Debug, Fail)]
pub enum RemoveViewError {
    #[fail(display = "invalid id given, cannot be removed: {}", id)]
    InvalidId {
        id: Id,
    },

    #[fail(display = "id has no parent, cannot be removed: {}", id)]
    NoParent {
        id: Id
    },

    #[fail(display = "something broke, oh no ")]
    Generic {},
}

impl std::convert::From<failure::Error> for RemoveViewError {
    fn from(_error: failure::Error) -> Self {
        RemoveViewError::Generic{}
    }
}

impl std::convert::From<failure::Error> for AddViewError {
    fn from(_error: failure::Error) -> Self {
        AddViewError::GenericError{}
    }
}
