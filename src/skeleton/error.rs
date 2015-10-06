use serialize::hex::FromHexError;

/// Error that can happen while calculating an animation.
pub enum SkeletonError {
    /// The requested bone was not found.
    BoneNotFound(String),

    /// The requested slot was not found.
    SlotNotFound(String),

    /// The requested slot was not found.
    SkinNotFound(String),

    /// The requested slot was not found.
    InvalidColor(String),

    /// The requested animation was not found.
    AnimationNotFound(String),
}

impl From<SkeletonError> for String {
    fn from(error: SkeletonError) -> String {
        match error {
            SkeletonError::BoneNotFound(b) => format!("Cannot find bone '{}'", &b),
            SkeletonError::SlotNotFound(s) => format!("Cannot find slot '{}'", &s),
            SkeletonError::SkinNotFound(s) => format!("Cannot find skin '{}'", &s),
            SkeletonError::InvalidColor(s) => format!("Cannot convert '{}' into a color", &s),
            SkeletonError::AnimationNotFound(s) => format!("Cannot find animation '{}'", &s),
        }
    }
}

impl From<FromHexError> for SkeletonError {
    fn from(error: FromHexError) -> SkeletonError {
        SkeletonError::InvalidColor(format!("{:?}", error))
    }
}
