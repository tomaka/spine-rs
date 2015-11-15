//! Module to interpolate animated sprites

use skeleton;
use skeleton::error::SkeletonError;
use std::collections::HashMap;
use std::slice::Iter;

/// Wrapper on attachment depending whether slot attachment is animated or not
enum AttachmentWrapper<'a> {
    Static(Option<&'a skeleton::Attachment>),
    Dynamic(Option<&'a skeleton::Attachment>, HashMap<&'a str, Option<&'a skeleton::Attachment>>),
}

/// Struct to handle animated skin and calculate sprites
pub struct SkinAnimation<'a> {
    anim_bones: Vec<(&'a skeleton::Bone, Option<&'a skeleton::timelines::BoneTimeline>)>,
    anim_slots: Vec<(&'a skeleton::Slot, AttachmentWrapper<'a>, Option<&'a skeleton::timelines::SlotTimeline>)>,
    duration: f32
}

/// Interpolated slot with attachment and color
#[derive(Debug)]
pub struct Sprite<'a> {
    /// attachment name
    pub attachment: &'a str,
    /// color
    pub color: [u8; 4],
    /// srt
    pub srt: skeleton::SRT
}

impl<'a> SkinAnimation<'a> {

    /// Iterator<Item=Vec<CalculatedSlot>> where item are modified with timelines
    pub fn new(skeleton: &'a skeleton::Skeleton, skin: &str, animation: Option<&str>)
        -> Result<SkinAnimation<'a>, SkeletonError>
    {
        // search all attachments defined by the skin name (use 'default' skin if not found)
        let skin = try!(skeleton.get_skin(skin));
        let default_skin = try!(skeleton.get_skin("default"));

        // get animation
        let (animation, duration) = if let Some(animation) = animation {
            let anim = try!(skeleton.animations.get(animation)
                .ok_or_else(|| SkeletonError::AnimationNotFound(animation.to_owned())));
            (Some(anim), anim.duration)
        } else {
            (None, 0f32)
        };

        // get bone related data
        let anim_bones = skeleton.bones.iter().enumerate().map(|(i, b)|
            (b, animation.and_then(|anim| anim.bones.iter()
                .find(|&&(idx, _)| idx == i).map(|&(_, ref a)| a)))).collect();

        let find_attach = |i: usize, name: &str| skin.find(i, name).or_else(|| default_skin.find(i, name));

        // get slot related data
        let anim_slots = skeleton.slots.iter().enumerate().map(|(i, s)| {

            let anim = animation.and_then(|anim|
                anim.slots.iter().find(|&&(idx, _)| idx == i ).map(|&(_, ref anim)| anim));

            let slot_attach = s.attachment.as_ref().and_then(|name| find_attach(i, &name));
            let attach = match anim.map(|anim| anim.get_attachment_names()) {
                Some(names) => {
                    if names.is_empty() {
                         AttachmentWrapper::Static(slot_attach)
                    } else {
                        let attachments = names.iter().map(|&name|(name, find_attach(i, name))).collect();
                        AttachmentWrapper::Dynamic(slot_attach, attachments)
                    }
                },
                None => AttachmentWrapper::Static(slot_attach)
            };
            (s, attach, anim)
        }).collect();

        Ok(SkinAnimation {
            duration: duration,
            anim_bones: anim_bones,
            anim_slots: anim_slots,
        })
    }

    /// Gets duration of the longest timeline in the animation
    pub fn get_duration(&self) -> f32 {
        self.duration
    }

    /// gets all bones srts at given time
    fn get_bones_srts(&self, time: f32) -> Vec<skeleton::SRT> {

        let mut srts: Vec<skeleton::SRT> = Vec::with_capacity(self.anim_bones.len());
        for &(b, anim) in &self.anim_bones {

            // starts with setup pose
            let mut srt = b.srt.clone();
            let mut rotation = 0.0;

            // add animation srt
            if let Some(anim_srt) = anim.map(|anim| anim.srt(time)) {
                srt.position[0] += anim_srt.position[0];
                srt.position[1] += anim_srt.position[1];
                rotation += anim_srt.rotation;
                srt.scale[0] *= anim_srt.scale[0];
                srt.scale[1] *= anim_srt.scale[1];
            }

            // inherit world from parent srt
            if let Some(ref parent_srt) = b.parent_index.and_then(|p| srts.get(p)) {
                srt.position = parent_srt.transform(srt.position);
                if b.inherit_rotation {
                    rotation += parent_srt.rotation;
                }
                if b.inherit_scale {
                    srt.scale[0] *= parent_srt.scale[0];
                    srt.scale[1] *= parent_srt.scale[1];
                }
            }

            // re-calculate sin/cos only if rotation has changed
            if rotation != 0.0 {
                srt.rotation += rotation;
                srt.cos = srt.rotation.cos();
                srt.sin = srt.rotation.sin();
            }
            srts.push(srt)
        }
        srts
    }

    /// Interpolates animated slots at given time
    pub fn interpolate<'b: 'a>(&'b self, time: f32) -> Option<Sprites<'b>> {

        if time > self.duration {
            return None;
        }

        let srts = self.get_bones_srts(time);
        let iter = self.anim_slots.iter();
        Some(Sprites {
            iter: iter,
            srts: srts,
            time: time
        })
    }

    /// Creates an iterator which iterates sprites at delta seconds interval
    pub fn run<'b: 'a>(&'b self, delta: f32) -> AnimationIter<'b> {
        AnimationIter {
            skin_animation: &self,
            time: 0f32,
            delta: delta
        }
    }
}

/// Iterator over all sprites interpolated at a given time
pub struct Sprites<'a> {
    iter: Iter<'a, (&'a skeleton::Slot, AttachmentWrapper<'a>, Option<&'a skeleton::timelines::SlotTimeline>)>,
    srts: Vec<skeleton::SRT>,
    time: f32
}

impl<'a> Iterator for Sprites<'a> {
    type Item = Sprite<'a>;
    fn next<'b>(&'b mut self) -> Option<Sprite<'a>> {

        while let Some(&(slot, ref skin_attach, anim)) = self.iter.next() {

            // search animated attachment
            let (name, skin_attach) = match *skin_attach {
                AttachmentWrapper::Static(ref attach) => (None, attach),
                AttachmentWrapper::Dynamic(ref attach, ref names) => {
                    match anim.unwrap().interpolate_attachment(self.time) {
                        Some(Some(name)) => {
                            let attach = names.get(&*name).unwrap();
                            (Some(name), attach)
                        },
                        Some(None) => (None, attach),
                        None => (None, attach),
                    }
                }
            };

            // nothing to show if there is no attachment
            if let Some(ref skin_attach) = *skin_attach {

                // color
                let color = anim.map(|anim| anim.interpolate_color(self.time))
                            .unwrap_or(slot.color.clone());

                // attachment name
                let attach_name = name.or(skin_attach.name.as_ref()
                                      .or(slot.attachment.as_ref()).map(|n| &**n))
                                  .expect("no attachment name provided");

                return Some(Sprite {
                    attachment: attach_name,
                    srt: self.srts[slot.bone_index].clone(),
                    color: color
                })
            }
        }

        // end of iter
        None
    }
}

/// Iterator over a constant period
#[derive(Clone)]
pub struct AnimationIter<'a> {
    skin_animation: &'a SkinAnimation<'a>,
    time: f32,
    delta: f32
}

impl<'a> Iterator for AnimationIter<'a> {
    type Item = Sprites<'a>;
    fn next(&mut self) -> Option<Sprites<'a>> {
        let result = self.skin_animation.interpolate(self.time);
        self.time += self.delta;
        result
    }
}
