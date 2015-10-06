use skeleton;
use skeleton::error::SkeletonError;
use std::collections::HashSet;

pub struct AnimationIter<'a> {
    time: f32,
    skeleton: &'a skeleton::Skeleton,
    animation: Option<&'a skeleton::Animation>,
    skin: &'a skeleton::Skin,
    default_skin: &'a skeleton::Skin,
    world_srts: Vec<skeleton::SRT>
}

struct AnimationItem {
    srt: skeleton::SRT,
    slot: skeleton::Slot
}

impl<'a> AnimationIter<'a> {

    /// Iterator<Item=Vec<Slot>> where item are modified with timelines
    pub fn new(skeleton: &'a skeleton::Skeleton, skin: &str, animation: Option<&str>)
        -> Result<AnimationIter<'a>, SkeletonError>
    {
        // try getting skin and 'default' skin, Error out if not found
        let skin = try!(skeleton.skins.get(skin)
            .ok_or(SkeletonError::SkinNotFound(skin.into())));
        let default_skin = try!(skeleton.skins.get("default")
            .ok_or(SkeletonError::SkinNotFound("default".into())));

        // get all bones in animation
        let animation = if let Some(animation) = animation {
            Some(try!(skeleton.animations.get(animation)
                .ok_or(SkeletonError::AnimationNotFound(animation.into()))))
        } else {
            None
        };

        Ok(AnimationIter {
            time: 0f32,
            skeleton: skeleton,
            animation: animation,
            skin: skin,
            default_skin: default_skin,
            world_srts: Vec::new()
        })
    }

}
