use skeleton;
use skeleton::error::SkeletonError;
use std::collections::HashSet;

pub struct AnimationIter<'a> {
    time: f32,
    skeleton: &'a skeleton::Skeleton,
    animation: Option<&'a skeleton::Animation>,
    skin: &'a skeleton::Skin,
    default_skin: &'a skeleton::Skin
}

impl<'a> AnimationIter<'a> {

    /// Iterator<Item=Vec<Slot>> where item are modified with timelines
    pub fn new(skeleton: &'a skeleton::Skeleton, skin: &str, animation: Option<&str>)
        -> Result<AnimationIter<'a>, SkeletonError>
    {
        // try getting skins
        let skin = try!(skeleton.skins.get(skin)
            .ok_or(SkeletonError::SkinNotFound(skin.into())));
        let default_skin = try!(skeleton.skins.get("default")
            .ok_or(SkeletonError::SkinNotFound("default".into())));

        // get animation
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
            default_skin: default_skin
        })
    }

}

pub struct AnimationItem {
    srt: skeleton::SRT,
    slot: skeleton::Slot
}

impl<'a> Iterator for AnimationIter<'a> {
    type Item = Vec<(String, skeleton::SRT)>;

    fn next(&mut self) -> Option<Vec<(String, skeleton::SRT)>> {

        // escape if exceeds animation time
        if self.time > self.animation.map(|anim| anim.duration).unwrap_or(0f32) {
            return None;
        }

        // get all bones srt
        let mut srts: Vec<skeleton::SRT> = Vec::with_capacity(self.skeleton.bones.len());
        for (i, b) in self.skeleton.bones.iter().enumerate() {

            // starts with default bone srt
            let mut srt = b.srt.clone();

            // parent srt: translate bone (do not inherit scale and rotation yet)
            if let Some(ref parent_srt) = b.parent_index.and_then(|p| srts.get(p)) {
                srt.position.0 += parent_srt.position.0;
                srt.position.1 += parent_srt.position.1;
            }

            // animation srt
            if let Some(anim_srt) = self.animation
                .and_then(|anim| anim.bones.iter().find(|&&(idx, _)| idx == i ))
                .map(|&(_, ref anim)| anim.srt(self.time)) {
                srt.position.0 += anim_srt.position.0;
                srt.position.1 += anim_srt.position.1;
                srt.rotation += anim_srt.rotation;
                srt.scale.0 += anim_srt.scale.0;
                srt.scale.1 += anim_srt.scale.1;
            }

            srts.push(srt)
        }

        // loop all slots and animate them
        let mut result = Vec::new();
        for (i, slot) in self.skeleton.slots.iter().enumerate() {

            // nothing to show if there is no Attachment
            if let Some(ref skin_attach) = slot.attachment.as_ref()
                .and_then(|slot_attach|
                    // TODO: find a better way to store skins
                    self.skin.find(i, &slot_attach)
                    .or_else(|| self.skin.find(i, &slot_attach))) {

                let mut srt = srts[slot.bone_index].clone();
                srt.position.0 += skin_attach.srt.position.0;
                srt.position.1 += skin_attach.srt.position.1;
                srt.rotation += skin_attach.srt.rotation;
                srt.scale.0 += skin_attach.srt.scale.0;
                srt.scale.1 += skin_attach.srt.scale.1;

                let attach_name = skin_attach.name.clone().unwrap_or(slot.attachment.clone().unwrap());
                result.push((attach_name, srt));

                // TODO: change color and attachment if animating
            }

        }

        // increase time
        self.time += 0.1f32;

        Some(result)
    }

}
