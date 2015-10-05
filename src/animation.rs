use skeleton;
use std::collections::HashSet;

pub struct AnimationIter<'a> {
    time: f32,
    animation: Option<&'a skeleton::Animation>
}

impl<'a> AnimationIter<'a> {

    /// Create an iterator over all the slots to be animated
    /// Todo: move children ?
    pub fn new(skeleton: &'a skeleton::Skeleton, skin: &str, animation: &str)
        -> Result<AnimationIter<'a>, skeleton::SkeletonError> {

        // try getting skin or 'default' skin, Error out if not found
        let skin = try!(skeleton.skins.get(skin).or_else(|| skeleton.skins.get("default"))
            .ok_or(skeleton::SkeletonError::SkinNotFound(skin.into())));

        // get all bones in animation
        let animation = skeleton.animations.get(animation);
        let animated_bone_indices = animation.map_or(Vec::new(), |animation|,
            animation.bones.iter().map(|&(bone_index, _)| bone_index).collect());

        let (mut static_srt, mut animated_srt) = (Vec::new(), Vec::new());
        for &(slot_index, attachments) in skin.iter() {
            let slot = skeleton.slots[slot_index];
            if let Some(attachment) = slot.attachment {
                let bone = skeleton.bones[slot.bone_index];
                let bone_srt = bone.srt.clone();
                if animated_bone_indices.contains(slot.bone_index) {
                    for animation in slot.animations
                    animated_srt.push((bone_index, skeleton.bones[bone_index].srt.clone());
                } else {
                    static_srt.push((bone_index, skeleton.bones[bone_index].srt.clone());
                }
            }
        })

// get all the attachments

    }
}
