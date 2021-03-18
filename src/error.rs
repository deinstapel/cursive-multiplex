use crate::Id;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AddViewError {
    #[error("some error occured")]
    GenericError {},
}

#[derive(Debug, Error)]
pub enum RemoveViewError {
    #[error("invalid id given, cannot be removed: {}", id)]
    InvalidId { id: Id },

    #[error("id has no parent, cannot be removed: {}", id)]
    NoParent { id: Id },

    #[error("something broke, oh no ")]
    Generic {},
}

#[derive(Debug, Error)]
pub enum SwitchError {
    #[error("node {} has no parent to be switched to from {}", from, to)]
    NoParent { from: Id, to: Id },

    #[error("error while switching, figuring out...")]
    Failed {},
}

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("encountered arithmetic error")]
    Arithmetic {},
}

impl std::convert::From<indextree::NodeError> for SwitchError {
    fn from(_error: indextree::NodeError) -> Self {
        SwitchError::Failed {}
    }
}
