//! Module to handle all spine errors

use serialize::hex::FromHexError;
use serialize::json::ParserError;
use from_json::FromJsonError;
use std::fmt;
use std::error::Error;

/// Error that can happen while calculating an animation.
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

impl fmt::Debug for SkeletonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SkeletonError::BoneNotFound(ref name) => write!(f, "Cannot find bone '{}'", name),
            SkeletonError::SlotNotFound(ref name) => write!(f, "Cannot find slot '{}'", name),
            SkeletonError::SkinNotFound(ref name) => write!(f, "Cannot find skin '{}'", name),
            SkeletonError::AnimationNotFound(ref name) => write!(f, "Cannot find animation '{}'", name),
            SkeletonError::InvalidColor(ref e)  => write!(f, "Cannot convert color to hexadecimal: {:?}", e),
            SkeletonError::FromJsonError(ref e) => write!(f, "Cannot deserialize from json: {:?}", e),
            SkeletonError::ParserError(ref e)   => write!(f, "Cannot deserialize from json: {:?}", e),
        }
    }
}

impl fmt::Display for SkeletonError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self, formatter)
    }
}

impl Error for SkeletonError {
    fn description(&self) -> &str {
        match *self {
            SkeletonError::BoneNotFound(_) => "bone cannot be found in skeleton bones",
            SkeletonError::SlotNotFound(_) => "slot cannot be found in skeleton slots",
            SkeletonError::SkinNotFound(_) => "skin cannot be found in skeleton skins",
            SkeletonError::InvalidColor(_) => "color cannot be parsed",
            SkeletonError::AnimationNotFound(_) => "animation cannot be found in skeleton animations",
            SkeletonError::FromJsonError(_) => "error while parsing json skeleton",
            SkeletonError::ParserError(_) => "error while parsing json skeleton",
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
