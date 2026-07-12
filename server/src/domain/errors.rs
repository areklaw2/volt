use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum DomainError {
    #[error("direct conversation cannot have same user on both sides")]
    DirectWithSelf,
    #[error("group conversation requires a non-empty title")]
    GroupNeedsTitle,
    #[error("cannot add participant to a direct conversation")]
    CannotAddToDirect,
    #[error("user is already a participant")]
    AlreadyParticipant,
    #[error("message content cannot be empty")]
    EmptyMessage,
    #[error("image message content must be a URL")]
    ImageNeedsUrl,
    #[error("only the sender can edit this message")]
    NotYourMessage,
    #[error("conversation not found")]
    ConversationNotFound,
    #[error("user is not a participant of this conversation")]
    NotAParticipant,
    #[error("username cannot be empty")]
    EmptyUsername,
    #[error("display name cannot be empty")]
    EmptyDisplayName,
    #[error("internal error: {0}")]
    Internal(String),
}
