//! Skeleton structs
//! Owns json::Animation

pub mod error;
mod timelines;
pub mod animation;

use json;
use from_json;
use std::collections::HashMap;
use std::io::Read;
use std::f32::consts::PI;
use serialize::hex::{FromHex, FromHexError};

// Reexport skeleton modules
use self::error::SkeletonError;
use self::timelines::{BoneTimeline, SlotTimeline};
use self::animation::SkinAnimation;

const TO_RADIAN: f32 = PI / 180f32;

fn bone_index(name: &str, bones: &[Bone]) -> Result<usize, SkeletonError> {
    bones.iter().position(|b| b.name == *name).ok_or_else(|| SkeletonError::BoneNotFound(name.to_owned()))
}

fn slot_index(name: &str, slots: &[Slot]) -> Result<usize, SkeletonError> {
    slots.iter().position(|b| b.name == *name).ok_or_else(|| SkeletonError::SlotNotFound(name.to_owned()))
}

/// Skeleton data converted from json and loaded into memory
pub struct Skeleton {
    /// bones for the skeleton, hierarchically ordered
    bones: Vec<Bone>,
    /// slots
    slots: Vec<Slot>,
    /// skins : key: skin name, value: slots attachments
    skins: HashMap<String, Skin>,
    /// all the animations
    animations: HashMap<String, Animation>
}

impl Skeleton {

    /// Consumes reader (with json data) and returns a skeleton wrapping
    pub fn from_reader<R: Read>(mut reader: R) -> Result<Skeleton, SkeletonError> {

        // read and convert as json
        let document = try!(from_json::Json::from_reader(&mut reader));
        let document: json::Document = try!(from_json::FromJson::from_json(&document));

        // convert to skeleton (consumes document)
        Skeleton::from_json(document)
    }

    /// Creates a from_json skeleton
    /// Consumes json::Document
    fn from_json(doc: json::Document) -> Result<Skeleton, SkeletonError> {

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
        for jskin in doc.skins.into_iter() {
            for (name, jslots) in jskin.into_iter() {
                let mut skin = Vec::new();
                for (name, attachments) in jslots.into_iter() {
                    let slot_index = try!(slot_index(&name, &slots));
                    let attachments = attachments.into_iter().map(|(name, attachment)| {
                        (name, Attachment::from_json(attachment))
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

    /// get skin
    pub fn get_skin<'a>(&'a self, name: &str) -> Result<&'a Skin, SkeletonError> {
        self.skins.get(name).ok_or_else(|| SkeletonError::SkinNotFound(name.to_owned()))
    }

    /// Gets a SkinAnimation which can interpolate slots at a given time
    pub fn get_animated_skin<'a>(&'a self, skin: &str, animation: Option<&str>)
        -> Result<SkinAnimation<'a>, SkeletonError>
    {
        SkinAnimation::new(self, skin, animation)
    }

    /// Returns the list of all skins names in this document.
    pub fn get_skins_names(&self) -> Vec<&str> {
        self.skins.keys().map(|k| &**k).collect()
    }

    /// Returns the list of all animations names in this document.
    pub fn get_animations_names(&self) -> Vec<&str> {
        self.animations.keys().map(|k| &**k).collect()
    }

    /// Returns the list of all attachment names in all skins in this document.
    ///
    /// The purpose of this function is to allow you to preload what you need.
    pub fn get_attachments_names(&self) -> Vec<&str> {
        let mut names: Vec<_> = self.skins.values()
            .flat_map(|skin| skin.slots.iter()
                .flat_map(|&(_, ref attach)| attach.iter()
                    .map(|(k, v)| v.name.as_ref().map(|n| &**n).unwrap_or(&*k))))
            .collect();

        names.sort();
        names.dedup();
        names
    }
}

/// Skin
/// defines a set of slot with custom attachments
/// slots: Vec<(slot_index, HashMap<custom_attachment_name, Attachment>)>
/// TODO: simpler architecture
pub struct Skin {
    /// all slots modified by the skin, the default skin contains all skeleton bones
    slots: Vec<(usize, HashMap<String, Attachment>)>
}

impl Skin {
    /// find attachment in a skin
    fn find(&self, slot_index: usize, attach_name: &str) -> Option<&Attachment> {
        self.slots.iter().filter_map(|&(i, ref attachs)|
            if i == slot_index {
                attachs.get(attach_name)
            } else {
                None
            }).next()
    }

    /// get all attachments and their positions to setup the skeleton's skin
    pub fn attachment_positions(&self) -> Vec<(&str, &[[f32; 2]; 4])> {
        self.slots.iter().flat_map(|&(_, ref attachs)|
            attachs.iter().map(|(name, ref attach)| (&**name, &attach.positions))).collect()
    }
}

/// Animation with precomputed data
struct Animation {
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
                let timeline = try!(BoneTimeline::from_json(timelines));
                abones.push((index, timeline));
            }
        }

        let mut aslots = Vec::new();
        for jslots in animation.slots.into_iter() {
            for (name, timelines) in jslots.into_iter() {
                let index = try!(slot_index(&name, slots));
                let timeline = try!(SlotTimeline::from_json(timelines));
                aslots.push((index, timeline));
            }
        }

        Ok(Animation {
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
    /// scale
    pub scale: [f32; 2],
    /// rotation in radians
    pub rotation: f32,
    /// position or translation
    pub position: [f32; 2],
    /// cosinus
    pub cos: f32,
    /// sinus
    pub sin: f32
}

impl SRT {

    /// new srt
    pub fn new(scale_x: f32, scale_y: f32, rotation_deg: f32, x: f32, y: f32) -> SRT {
        let rotation = rotation_deg * TO_RADIAN;
        SRT {
            scale: [scale_x, scale_y],
            rotation: rotation,
            position: [x, y],
            cos: rotation.cos(),
            sin: rotation.sin()
        }
    }

    /// apply srt on a 2D point (consumes the point)
    pub fn transform(&self, v: [f32; 2]) -> [f32; 2] {
        [self.cos * v[0] * self.scale[0] - self.sin * v[1] * self.scale[1] + self.position[0],
         self.sin * v[0] * self.scale[0] + self.cos * v[1] * self.scale[1] + self.position[1]]
    }

    /// convert srt to a 3x3 transformation matrix (2D)
    pub fn to_matrix3(&self) -> [[f32; 3]; 3] {
        [
            [ self.cos * self.scale[0], self.sin, 0.0],
            [-self.sin, self.cos * self.scale[1], 0.0],
            [ self.position[0] , self.position[1], 1.0f32],
        ]
    }

    /// convert srt to a 4x4 transformation matrix (3D)
    pub fn to_matrix4(&self) -> [[f32; 4]; 4] {
        [
            [ self.cos * self.scale[0], self.sin, 0.0, 0.0],
            [-self.sin, self.cos * self.scale[1], 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [ self.position[0] , self.position[1], 0.0, 1.0f32],
        ]
    }

}

/// skeleton bone
struct Bone {
    name: String,
    parent_index: Option<usize>,
    // length: f32,
    srt: SRT,
    inherit_scale: bool,
    inherit_rotation: bool
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
            // length: bone.length.unwrap_or(0f32),
            srt: SRT::new(bone.scale_x.unwrap_or(1.0), bone.scale_y.unwrap_or(1.0),
                bone.rotation.unwrap_or(0.0), bone.x.unwrap_or(0.0), bone.y.unwrap_or(0.0)),
            inherit_scale: bone.inherit_scale.unwrap_or(true),
            inherit_rotation: bone.inherit_rotation.unwrap_or(true),
        })
    }
}

/// skeleton slot
struct Slot {
    name: String,
    bone_index: usize,
    color: [u8; 4],
    attachment: Option<String>
}

impl Slot {
    fn from_json(slot: json::Slot, bones: &[Bone]) -> Result<Slot, SkeletonError> {
        let bone_index = try!(bone_index(&slot.bone, &bones));
        let color = match slot.color {
            Some(c) => {
                let v = try!(c.from_hex());
                if v.len() != 4 {
                    return Err(SkeletonError::InvalidColor(FromHexError::InvalidHexLength));
                }
                [v[0], v[1], v[2], v[3]]
            },
            None => [255, 255, 255, 255]
        };

        Ok(Slot {
            name: slot.name,
            bone_index: bone_index,
            color: color,
            attachment: slot.attachment
        })
    }
}

/// skeletom animation
#[derive(Debug)]
struct Attachment {
    name: Option<String>,
    type_: json::AttachmentType,
    positions: [[f32; 2]; 4]
    // fps: Option<f32>,
    // mode: Option<String>,
    //vertices: Option<Vec<??>>     // TODO: ?
}

impl Attachment {
    /// converts json data into skeleton data
    fn from_json(attachment: json::Attachment) -> Attachment {
        let srt = SRT::new(attachment.scale_x.unwrap_or(1.0), attachment.scale_y.unwrap_or(1.0),
                           attachment.rotation.unwrap_or(0.0),
                           attachment.x.unwrap_or(0.0), attachment.y.unwrap_or(0.0));
        let (w2, h2) = (attachment.width.unwrap_or(0f32) / 2.0,
                        attachment.height.unwrap_or(0f32) / 2.0);
        Attachment {
            name: attachment.name,
            type_: attachment.type_.unwrap_or(json::AttachmentType::Region),
            positions: [srt.transform([-w2,  h2]),
                        srt.transform([w2,  h2]),
                        srt.transform([w2,  -h2]),
                        srt.transform([-w2,  -h2])]
        }
    }
}
