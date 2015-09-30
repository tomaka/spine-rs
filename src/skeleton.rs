/// Skeleton structs
/// Owns json::Animation

use json;
use from_json;
use std::collections::HashMap;
use std::io::Read;

fn bone_index(name: &str, bones: &[Bone]) -> Result<usize, SkeletonError> {
    bones.iter().position(|b| b.data.name == *name).ok_or(SkeletonError::BoneNotFound(name.into()))
}

fn slot_index(name: &str, slots: &[Slot]) -> Result<usize, SkeletonError> {
    slots.iter().position(|b| b.data.name == *name).ok_or(SkeletonError::SlotNotFound(name.into()))
}

/// Error that can happen while calculating an animation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SkeletonError {
    /// The requested bone was not found.
    BoneNotFound(String),

    /// The requested slot was not found.
    SlotNotFound(String),
}

impl From<SkeletonError> for String {
    fn from(error: SkeletonError) -> String {
        match error {
            SkeletonError::BoneNotFound(b) => format!("Cannot find bone '{}'", &b),
            SkeletonError::SlotNotFound(s) => format!("Cannot find slot '{}'", &s),
        }
    }
}

pub struct Skeleton {
    bones: Vec<Bone>,
    slots: Vec<Slot>,
    skins: HashMap<String, Vec<(usize, Vec<json::Attachment>)>>,
    animations: HashMap<String, Animation>
}

impl Skeleton {

    pub fn from_reader<R: Read>(mut reader: R) -> Result<Skeleton, String> {

        // read and convert as json
        let document = try!(from_json::Json::from_reader(&mut reader)
            .map_err(|e| format!("{:?}", e)));
        let document: json::Document = try!(from_json::FromJson::from_json(&document)
            .map_err(|e| format!("{:?}", e)));

        // convert to skeleton (consumes document)
        Skeleton::from_json(document)
    }

    /// Creates a from_json skeleton
    /// Consumes json::Document
    fn from_json(doc: json::Document) -> Result<Skeleton, String> {

        let mut bones = Vec::new();
        if let Some(jbones) = doc.bones {
            for b in jbones.into_iter() {
                let bone = try!(Bone::from_json(b, &bones));
                bones.push(bone);
            }
        }

        let mut slots = Vec::new();
        if let Some(jslots) = doc.slots {
            for s in jslots.into_iter() {
                let slot = try!(Slot::from_json(s, &bones));
                slots.push(slot);
            }
        }

        let mut animations = HashMap::new();
        for janimations in doc.animations.into_iter() {
            for (name, animation) in janimations.into_iter() {
                let animation = try!(Animation::from_json(animation, &bones, &slots));
                animations.insert(name, animation);
            }
        }

        let mut skins = HashMap::new();
        for jksins in doc.skins.into_iter() {
            for (name, jslots) in jksins.into_iter() {
                let mut skin = Vec::new();
                for (name, attachments) in jslots.into_iter() {
                    let slot = try!(slot_index(&name, &slots));
                    let mut attachments = attachments.into_iter().map(|(name, mut attachment)| {
                        if attachment.name.is_none() {
                            attachment.name = Some(name);
                        }
                        attachment
                     }).collect();
                    skin.push((slot, attachments));
                }
                skins.insert(name, skin);
            }
        }

        Ok(Skeleton {
            bones: bones,
            slots: slots,
            skins: skins,
            animations: animations
        })
    }

}


/// Animation with precomputed data
pub struct Animation {
    bones: Vec<(usize, json::BoneTimeline)>,
    slots: Vec<(usize, json::SlotTimeline)>,
    events: Vec<json::EventKeyframe>,
    draworder: Vec<json::DrawOrderTimeline>,
    duration: f32
}

impl Animation {

    /// Creates a from_json Animation
    pub fn from_json(animation: json::Animation, bones: &[Bone], slots: &[Slot])
        -> Result<Animation, SkeletonError>
    {
        let duration = Animation::duration(&animation);

        let mut abones = Vec::new();
        for jbones in animation.bones.into_iter() {
            for (name, timelines) in jbones.into_iter() {
                let index = try!(bone_index(&name, bones));
                abones.push((index, timelines));
            }
        }

        let mut aslots = Vec::new();
        for jslots in animation.slots.into_iter() {
            for (name, timelines) in jslots.into_iter() {
                let index = try!(slot_index(&name, slots));
                aslots.push((index, timelines));
            }
        }

        Ok(Animation {
            // data: animation,
            duration: duration,
            bones: abones,
            slots: aslots,
            events: animation.events.unwrap_or(Vec::new()),
            draworder: animation.draworder.unwrap_or(Vec::new()),
        })
    }

    fn duration(animation: &json::Animation) -> f32 {
        animation.bones.iter().flat_map(|bones| bones.values().flat_map(|timelines|{
            timelines.translate.iter().flat_map(|translate| translate.iter().map(|e| e.time))
            .chain(timelines.rotate.iter().flat_map(|rotate| rotate.iter().map(|e| e.time)))
            .chain(timelines.scale.iter().flat_map(|scale| scale.iter().map(|e| e.time)))
        }))
        .chain(animation.slots.iter().flat_map(|slots| slots.values().flat_map(|timelines|{
            timelines.attachment.iter().flat_map(|attachment| attachment.iter().map(|e| e.time))
            .chain(timelines.color.iter().flat_map(|color| color.iter().map(|e| e.time)))
        })))
        .fold(0.0f32, f32::max)
    }

}

pub struct Bone {
    data: json::Bone,
    parent_index: Option<usize>
}

impl Bone {
    pub fn from_json(bone: json::Bone, bones: &[Bone]) -> Result<Bone, SkeletonError> {
        let index = match bone.parent {
            Some(ref name) => Some(try!(bone_index(name, bones))),
            None => None
        };
        Ok(Bone {
            data: bone,
            parent_index: index
        })
    }
}

pub struct Slot {
    data: json::Slot,
    bone_index: usize
}

impl Slot {
    pub fn from_json(slot: json::Slot, bones: &[Bone]) -> Result<Slot, SkeletonError> {
        let bone_index = try!(bone_index(&slot.bone, &bones));
        Ok(Slot {
            data: slot,
            bone_index: bone_index
        })
    }
}
