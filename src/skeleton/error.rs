use serialize::hex::FromHexError;
use serialize::json::ParserError;
use std::fmt;
use std::error::Error;
use from_json::FromJsonError;

/// Error that can happen while calculating an animation.
#[derive(Debug)]
pub enum SkeletonError {

    /// Parser error
    ParserError(ParserError),

    /// Parser error
    FromJsonError(FromJsonError),

    /// The requested bone was not found.
    BoneNotFound(String),

    /// The requested slot was not found.
    SlotNotFound(String),

    /// The requested slot was not found.
    SkinNotFound(String),

    /// The requested slot was not found.
    InvalidColor(FromHexError),

    /// The requested animation was not found.
    AnimationNotFound(String),
}

impl fmt::Display for SkeletonError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(formatter)
    }
}

impl Error for SkeletonError {
    fn description(&self) -> &str {
        match self {
            &SkeletonError::BoneNotFound(_) => "Bone name cannot be found in skeleton bones",
            &SkeletonError::SlotNotFound(_) => "Slot name cannot be found in skeleton slots",
            &SkeletonError::SkinNotFound(_) => "Skin name cannot be found in skeleton skins",
            &SkeletonError::InvalidColor(_) => "Color cannot be parsed",
            &SkeletonError::AnimationNotFound(_) => "Animation name cannot be found in skeleton animations",
            &SkeletonError::FromJsonError(_) => "Error while parsing json skeleton",
            &SkeletonError::ParserError(_) => "Error while parsing json skeleton",
        }
    }
}

impl From<FromHexError> for SkeletonError {
    fn from(error: FromHexError) -> SkeletonError {
        SkeletonError::InvalidColor(error)
    }
}

impl From<ParserError> for SkeletonError {
    fn from(error: ParserError) -> SkeletonError {
        SkeletonError::ParserError(error)
    }
}

impl From<FromJsonError> for SkeletonError {
    fn from(error: FromJsonError) -> SkeletonError {
        SkeletonError::FromJsonError(error)
    }
}
