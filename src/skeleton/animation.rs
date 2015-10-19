use skeleton;
use skeleton::error::SkeletonError;

pub struct SkinAnimation<'a> {
    skeleton: &'a skeleton::Skeleton,
    animation: Option<&'a skeleton::Animation>,
    skin: &'a skeleton::Skin,
    default_skin: &'a skeleton::Skin,
    duration: f32
}

pub struct CalculatedSlot {
    pub attachment: String,
    pub srt: skeleton::SRT,
    pub color: Vec<u8>
}

impl<'a> SkinAnimation<'a> {

    /// Iterator<Item=Vec<CalculatedSlot>> where item are modified with timelines
    pub fn new(skeleton: &'a skeleton::Skeleton, skin: &str, animation: Option<&str>)
        -> Result<SkinAnimation<'a>, SkeletonError>
    {
        // try getting skins
        let skin = try!(skeleton.skins.get(skin)
            .ok_or(SkeletonError::SkinNotFound(skin.into())));
        let default_skin = try!(skeleton.skins.get("default")
            .ok_or(SkeletonError::SkinNotFound("default".into())));

        // get animation
        let (animation, duration) = if let Some(animation) = animation {
            let anim = try!(skeleton.animations.get(animation)
                .ok_or(SkeletonError::AnimationNotFound(animation.into())));
            (Some(anim), anim.duration)
        } else {
            (None, 0f32)
        };

        Ok(SkinAnimation {
            skeleton: skeleton,
            animation: animation,
            skin: skin,
            default_skin: default_skin,
            duration: duration,
        })
    }

    /// Interpolates animated slots at given time
    pub fn interpolate(&self, time: f32) -> Option<Vec<CalculatedSlot>> {

        if time > self.duration {
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
                .map(|&(_, ref anim)| anim.srt(time)) {
                srt.add_assign(&anim_srt);
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
                    .or_else(|| self.default_skin.find(i, &slot_attach))) {

                let mut srt = srts[slot.bone_index].clone();
                srt.add_assign(&skin_attach.srt);

                // color
                let color = self.animation
                    .and_then(|anim| anim.slots.iter().find(|&&(idx, _)| idx == i ))
                    .map(|&(_, ref anim)| (*anim).interpolate_color(time))
                    .unwrap_or(vec![255, 255, 255, 255]);

                let attach_name = skin_attach.name.clone().or_else(|| slot.attachment.clone())
                    .expect("no attachment name provided");

                result.push(CalculatedSlot {
                    attachment: attach_name,
                    srt: srt,
                    color: color
                });

                // TODO: change attachment if animating
            }
        }

        Some(result)
    }

    /// Creates an iterator which iterates slots at delta seconds interval
    pub fn iter(&'a self, delta: f32) -> AnimationIter<'a> {
        AnimationIter {
            skin_animation: &self,
            time: 0f32,
            delta: delta
        }
    }
}

pub struct AnimationIter<'a> {
    skin_animation: &'a SkinAnimation<'a>,
    time: f32,
    delta: f32
}

impl<'a> Iterator for AnimationIter<'a> {
    type Item = Vec<CalculatedSlot>;
    fn next(&mut self) -> Option<Vec<CalculatedSlot>> {
        let result = self.skin_animation.interpolate(self.time);
        self.time += self.delta;
        result
    }
}
