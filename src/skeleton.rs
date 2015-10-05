/// Skeleton structs
/// Owns json::Animation

use json;
use from_json;
use std::collections::HashMap;
use std::io::Read;
use std::f32::consts::PI;
use serialize::hex::{FromHex, FromHexError};

// Reexport all timelines
use timelines::*;

const TO_RADIAN: f32 = PI / 180f32;

fn bone_index(name: &str, bones: &[Bone]) -> Result<usize, SkeletonError> {
    bones.iter().position(|b| b.name == *name).ok_or(SkeletonError::BoneNotFound(name.into()))
}

fn slot_index(name: &str, slots: &[Slot]) -> Result<usize, SkeletonError> {
    slots.iter().position(|b| b.name == *name).ok_or(SkeletonError::SlotNotFound(name.into()))
}

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
}

impl From<SkeletonError> for String {
    fn from(error: SkeletonError) -> String {
        match error {
            SkeletonError::BoneNotFound(b) => format!("Cannot find bone '{}'", &b),
            SkeletonError::SlotNotFound(s) => format!("Cannot find slot '{}'", &s),
            SkeletonError::SkinNotFound(s) => format!("Cannot find skin '{}'", &s),
            SkeletonError::InvalidColor(s) => format!("Cannot convert '{}' into a color", &s),
        }
    }
}

impl From<FromHexError> for SkeletonError {
    fn from(error: FromHexError) -> SkeletonError {
        SkeletonError::InvalidColor(format!("{:?}", error))
    }
}

/// Skeleton data converted from json and loaded into memory
pub struct Skeleton {
    /// bones for the skeleton, hierarchically ordered
    pub bones: Vec<Bone>,
    /// slots
    pub slots: Vec<Slot>,
    /// skins : key: skin name, value: slots attachments
    pub skins: HashMap<String, Skin>,
    /// all the animations
    pub animations: HashMap<String, Animation>
}

impl Skeleton {

    /// Consumes reader (with json data) and returns a skeleton wrapping
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
                let slot = try!(Slot::from_json(s, &bones, &slots));
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
        for jskin in doc.skins.into_iter() {
            for (name, jslots) in jskin.into_iter() {
                let mut skin = Vec::new();
                for (name, attachments) in jslots.into_iter() {
                    let slot_index = try!(slot_index(&name, &slots));
                    let mut attachments = attachments.into_iter().map(|(name, mut attachment)| {
                        if attachment.name.is_none() { attachment.name = Some(name); }
                        Attachment::from_json(attachment)
                     }).collect();
                    skin.push((slot_index, attachments));
                }
                skins.insert(name, Skin {
                    slots: skin
                });
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

pub struct Skin {
    slots: Vec<(usize, Vec<Attachment>)>
}

/// Animation with precomputed data
pub struct Animation {
    bones: Vec<(usize, BoneTimeline)>,
    slots: Vec<(usize, SlotTimeline)>,
    events: Vec<json::EventKeyframe>,
    draworder: Vec<json::DrawOrderTimeline>,
    duration: f32
}

impl Animation {

    /// Creates a from_json Animation
    fn from_json(animation: json::Animation, bones: &[Bone], slots: &[Slot])
        -> Result<Animation, SkeletonError>
    {
        let duration = Animation::duration(&animation);

        let mut abones = Vec::new();
        for jbones in animation.bones.into_iter() {
            for (name, timelines) in jbones.into_iter() {
                let index = try!(bone_index(&name, bones));
                abones.push((index, BoneTimeline::from_json(timelines)));
            }
        }

        let mut aslots = Vec::new();
        for jslots in animation.slots.into_iter() {
            for (name, timelines) in jslots.into_iter() {
                let index = try!(slot_index(&name, slots));
                aslots.push((index, SlotTimeline::from_json(timelines)));
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

/// Scale, Rotate, Translate struct
#[derive(Debug, Clone)]
pub struct SRT {
    scale: (f32, f32),
    rotation: f32,
    translation: (f32, f32),
}

pub struct Bone {
    name: String,
    parent_index: Option<usize>,
    length: f32,
    srt: SRT
}

impl Bone {
    fn from_json(bone: json::Bone, bones: &[Bone]) -> Result<Bone, SkeletonError> {
        let index = match bone.parent {
            Some(ref name) => Some(try!(bone_index(name, bones))),
            None => None
        };
        Ok(Bone {
            name: bone.name,
            parent_index: index,
            length: bone.length.unwrap_or(0f32),
            srt: SRT {
                scale: (bone.scaleX.unwrap_or(1f32), bone.scaleY.unwrap_or(1f32)),
                rotation: bone.rotation.unwrap_or(0f32) * TO_RADIAN,
                translation: (bone.x.unwrap_or(0f32), bone.y.unwrap_or(0f32)),
            }
        })
    }
}

pub struct Slot {
    name: String,
    bone_index: usize,
    color: Vec<u8>,
    attachment: Option<usize>
}

impl Slot {
    fn from_json(slot: json::Slot, bones: &[Bone], slots: &[Slot]) -> Result<Slot, SkeletonError> {
        let bone_index = try!(bone_index(&slot.bone, &bones));
        let color = try!(slot.color.unwrap_or("FFFFFFFF".into()).from_hex());
        let attachment = match slot.attachment {
            Some(jslot) => Some(try!(slot_index(&jslot, &slots))),
            None => None
        };
        Ok(Slot {
            name: slot.name,
            bone_index: bone_index,
            color: color,
            attachment: attachment
        })
    }
}

pub struct Attachment {
    name: Option<String>,
    type_: json::AttachmentType,
    srt: SRT,
    size: (f32, f32),
    fps: Option<f32>,
    mode: Option<String>,
    //vertices: Option<Vec<??>>     // TODO: ?
}

impl Attachment {
    fn from_json(attachment: json::Attachment) -> Attachment {
        Attachment {
            name: attachment.name,
            type_: attachment.type_.unwrap_or(json::AttachmentType::Region),
            srt: SRT {
                scale: (attachment.scaleX.unwrap_or(1f32), attachment.scaleY.unwrap_or(1f32)),
                rotation: attachment.rotation.unwrap_or(0f32) * TO_RADIAN,
                translation: (attachment.x.unwrap_or(0f32), attachment.y.unwrap_or(0f32)),
            },
            size: (attachment.width.unwrap_or(0f32), attachment.height.unwrap_or(0f32)),
            fps: attachment.fps,
            mode: attachment.mode
        }
    }
}
