/*!
# spine

Parses a Spine document and calculates what needs to be drawn.

## Step 1: loading the document

Call `SpineDocument::new` to parse the content of a document.

This function returns an `Err` if the document is not valid JSON or if something is not
 recognized in it.

```no_run
# use std::fs::File;
# use std::path::Path;
let document = spine::SpineDocument::new(File::open(&Path::new("skeleton.json")).unwrap())
    .unwrap();
```

## Step 2: preparing for drawing

You can retreive the list of animations and skins provided a document:

```no_run
# let document: spine::SpineDocument = unsafe { std::mem::uninitialized() };
let skins = document.get_skins_list();

let animations = document.get_animations_list();
let first_animation_duration = document.get_animation_duration(animations[0]).unwrap();
```

You can also get a list of the names of all the sprites that can possibly be drawn by this
 Spine animation.

```no_run
# let document: spine::SpineDocument = unsafe { std::mem::uninitialized() };
let sprites = document.get_possible_sprites();
```

Note that the names do not necessarly match file names. They are the same names that you have in
 the Spine editor. It is your job to turn these resource names into file names if necessary.

## Step 3: animating

At each frame, call `document.calculate()` in order to get the list of things that need to be
 drawn for the current animation.

This function takes the skin name, the animation name (or `None` for the default pose) and the
 time in the current animation's loop.

```no_run
# let document: spine::SpineDocument = unsafe { std::mem::uninitialized() };
let results = document.calculate("default", Some("walk"), 0.176).unwrap();
```

The results contain the list of sprites that need to be drawn, with their matrix. The matrix
 supposes that each sprite would cover the whole viewport (ie. drawn from `(-1, -1)` to
 `(1, 1)`). The matrix is pre-multiplying ; if you want to apply your own matrix `C` over
 the one returned, you need to call `C * M`.

```no_run
# use std::collections::HashMap;
# let results: spine::Calculation = unsafe { std::mem::uninitialized() };
# let textures_list: HashMap<&str, i32> = unsafe { std::mem::uninitialized() };
# fn draw<A,B,C>(_: A, _: B, _: C) {}
for (sprite_name, matrix, color) in results.sprites.into_iter() {
    let texture = textures_list.get(&sprite_name).unwrap();
    draw(texture, matrix, color);
}
```

*/
#![deny(missing_docs)]

extern crate color;
extern crate cgmath;
#[macro_use]
extern crate from_json;

use color::{Rgb, Rgba};
use cgmath::Matrix4;

use std::io::Read;

mod json;
mod skeleton;

pub use skeleton::Skeleton;

/// Spine document loaded in memory.
pub struct SpineDocument {
    source: json::Document,
}

impl SpineDocument {
    /// Loads a document from a reader.
    pub fn new<R: Read>(mut reader: R) -> Result<SpineDocument, String> {
        let document = try!(from_json::Json::from_reader(&mut reader).map_err(|e| format!("{:?}", e)));
        let document: json::Document = try!(from_json::FromJson::from_json(&document)
            .map_err(|e| format!("{:?}", e)));

        Ok(SpineDocument {
            source: document
        })
    }

    /// Returns the list of all animations in this document.
    pub fn get_animations_list(&self) -> Vec<&str> {
        if let Some(ref list) = self.source.animations {
            list.keys().map(|e| &e[..]).collect()
        } else {
            Vec::new()
        }
    }

    /// Returns the list of all skins in this document.
    pub fn get_skins_list(&self) -> Vec<&str> {
        if let Some(ref list) = self.source.skins {
            list.keys().map(|e| &e[..]).collect()
        } else {
            Vec::new()
        }
    }

    /// Returns true if an animation is in the document.
    pub fn has_animation(&self, name: &str) -> bool {
        if let Some(ref list) = self.source.animations {
            list.get(&name.to_string()).is_some()
        } else {
            false
        }
    }

    /// Returns true if a skin is in the document.
    pub fn has_skin(&self, name: &str) -> bool {
        if let Some(ref list) = self.source.skins {
            list.get(&name.to_string()).is_some()
        } else {
            false
        }
    }

    /// Returns the duration of an animation.
    ///
    /// Returns `None` if the animation doesn't exist.
    ///
    /// TODO: check events and draworder?
    pub fn get_animation_duration(&self, animation: &str) -> Option<f32> {
        // getting a reference to the `json::Animation`
        let animation: &json::Animation =
            if let Some(anim) = self.source.animations.as_ref() {
                match anim.get(animation) {
                    Some(a) => a,
                    None => return None
                }
            } else {
                return None;
            };

        // this contains the final result
        let mut result = 0.0f32;

        // checking the bones
        if let Some(ref bones) = animation.bones {
            for timelines in bones.values() {
                if let Some(ref translate) = timelines.translate.as_ref() {
                    for elem in translate.iter() {
                        if elem.time > result { result = elem.time }
                    }
                }
                if let Some(ref rotate) = timelines.rotate.as_ref() {
                    for elem in rotate.iter() {
                        if elem.time > result { result = elem.time }
                    }
                }
                if let Some(ref scale) = timelines.scale.as_ref() {
                    for elem in scale.iter() {
                        if elem.time > result { result = elem.time }
                    }
                }
            }
        }

        // checking the slots
        if let Some(ref slots) = animation.slots {
            for timelines in slots.values() {
                if let Some(ref attachment) = timelines.attachment.as_ref() {
                    for elem in attachment.iter() {
                        if elem.time > result { result = elem.time }
                    }
                }
                if let Some(ref color) = timelines.color.as_ref() {
                    for elem in color.iter() {
                        if elem.time > result { result = elem.time }
                    }
                }
            }
        }

        // returning
        Some(result)
    }

    /// Returns a list of all possible sprites when drawing.
    ///
    /// The purpose of this function is to allow you to preload what you need.
    pub fn get_possible_sprites(&self) -> Vec<&str> {
        if let Some(ref list) = self.source.skins {
            let mut result = list.iter().flat_map(|(_, skin)| skin.iter())
                                 .flat_map(|(_, slot)| slot.iter())
                                 .map(|(name, vals)| {
                                     if let Some(ref name) = vals.name {
                                         &name[..]
                                     } else {
                                         &name[..]
                                     }
                                 })
                                 .collect::<Vec<_>>();

            result.sort();
            result.dedup();
            result

        } else {
            Vec::new()
        }
    }

    /// Calculates the list of sprites that must be displayed and their matrix.
    ///
    /// If `elapsed` is longer than the duration of the animation, it will be modulo'd.
    // TODO: implement draw order timeline
    // TODO: implement events
    // TODO: implement other attachment types
    pub fn calculate(&self, skin: &str, animation: Option<&str>, mut elapsed: f32)
        -> Result<Calculation, CalculationError>
    {
        // adapting elapsed
        if let Some(animation) = animation {
            if let Some(duration) = self.get_animation_duration(animation) {
                elapsed = elapsed % duration;
            }
        }
        let elapsed = elapsed;

        // getting a reference to the `json::Skin`
        let skin = try!(self.source.skins.as_ref().and_then(|l| l.get(skin))
            .ok_or(CalculationError::SkinNotFound));

        // getting a reference to "default" skin
        let default_skin = try!(self.source.skins.as_ref().and_then(|l| l.get("default"))
            .ok_or(CalculationError::SkinNotFound));

        // getting a reference to the `json::Animation`
        let animation: Option<&json::Animation> = match animation {
            Some(animation) => Some(try!(self.source.animations.as_ref()
                .and_then(|l| l.get(animation)).ok_or(CalculationError::AnimationNotFound))),
            None => None
        };

        // calculating the default pose of all bones
        let mut bones: Vec<(&json::Bone, BoneData)> = self.source.bones.as_ref().map(|bones| {
            bones.iter().map(|bone| (bone, get_bone_default_local_setup(bone))).collect()
        }).unwrap_or_else(|| Vec::new());

        // if we are animating, adding to the default pose the calculations from the animation
        if let Some(animation) = animation {
            if let Some(anim_bones) = animation.bones.as_ref() {
                for (bone_name, timelines) in anim_bones.iter() {
                    // calculating the variation from the animation
                    let anim_data = try!(timelines_to_bonedata(timelines, elapsed));

                    // adding this to the `bones` vec above
                    match bones.iter_mut().find(|&&mut (b, _)| b.name == *bone_name) {
                        Some(&mut (_, ref mut data)) => { *data = data.clone() + anim_data; },
                        None => ()
                    };
                }
            }
        };

        // now we have our list of bones with their relative positions
        // adding the position of the parent to each bone
        let bones: Vec<(&str, Matrix4<f32>)> = bones.iter().map(|&(ref bone, ref relative_data)| {
            let mut current_matrix = relative_data.to_matrix();
            let mut current_parent = bone.parent.as_ref();

            loop {
                if let Some(parent_name) = current_parent {
                    assert!(parent_name != &bone.name);     // prevent infinite loop

                    match bones.iter().find(|&&(b, _)| b.name == *parent_name) {
                        Some(ref p) => {
                            current_parent = p.0.parent.as_ref();
                            current_matrix = p.1.to_matrix() * current_matrix;
                        },
                        None => {
                            current_parent = None;  // TODO: return BoneNotFound(parent_name);
                        }
                    }

                } else {
                    break
                }
            }

            (&bone.name[..], current_matrix.clone())

        }).collect();

        // now taking each slot in the document and matching its bone
        // `slots` contains the slot name, bone data, color, and attachment
        let mut slots: Vec<(&str, Matrix4<f32>, Option<&str>, Option<&str>)> =
            if let Some(slots) = self.source.slots.as_ref() {
                let mut result = Vec::new();

                for slot in slots.iter() {
                    let bone = try!(bones.iter().find(|&&(name, _)| name == slot.bone)
                        .ok_or(CalculationError::BoneNotFound(&slot.bone)));
                    result.push((&slot.name[..], bone.1, slot.color.as_ref()
                        .map(|s| &s[..]), slot.attachment.as_ref().map(|s| &s[..])))
                }

                result

            } else {
                Vec::new()
            };

        // if we are animating, replacing the values by the ones overridden by the animation
        if let Some(animation) = animation {
            if let Some(anim_slots) = animation.slots.as_ref() {
                for (slot_name, timelines) in anim_slots.iter() {
                    // calculating the variation from the animation
                    let (anim_color, anim_attach) =
                        try!(timelines_to_slotdata(timelines, elapsed));

                    // adding this to the `slots` vec above
                    match slots.iter_mut().find(|&&mut (s, _, _, _)| s == slot_name) {
                        Some(&mut (_, _, ref mut color, ref mut attachment)) => {
                            if let Some(c) = anim_color { *color = Some(c) };
                            if let Some(a) = anim_attach { *attachment = Some(a) };
                        },
                        None => ()
                    };
                }
            }
        };

        // now finding the attachment of each slot
        let slots = {
            let mut results = Vec::new();

            for (slot_name, bone_data, _color, attachment) in slots.into_iter() {
                if let Some(attachment) = attachment {
                    let attachments = match skin.iter().chain(default_skin.iter())
                                                .find(|&(slot, _)| slot == slot_name)
                    {
                        Some(a) => a,
                        None => continue
                    };

                    let attachment = try!(attachments.1.iter()
                        .find(|&(a, _)| a == attachment)
                        .ok_or(CalculationError::AttachmentNotFound(attachment)));

                    let attachment_transform = get_attachment_transformation(attachment.1);
                    let bone_data = bone_data * attachment_transform;

                    let attachment = if let Some(ref name) = attachment.1.name {
                        &name[..]
                    } else {
                        &attachment.0[..]
                    };

                    results.push((
                        attachment,
                        bone_data,
                        Rgba { a: 255, c: Rgb::new(255, 255, 255) }
                    ));
                }
            }

            results
        };

        // final result
        Ok(Calculation {
            sprites: slots
        })
    }
}

/// Result of an animation state calculation.
#[derive(Debug)]
pub struct Calculation<'a> {
    /// The list of sprites that should be drawn.
    ///
    /// The elements are sorted from bottom to top, ie. each element can cover the previous one.
    ///
    /// The matrix assumes that the sprite is displayed from (-1, -1) to (1, 1), ie. would cover
    ///  the whole screen.
    pub sprites: Vec<(&'a str, Matrix4<f32>, Rgba<u8>)>,
}

/// Error that can happen while calculating an animation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CalculationError<'a> {
    /// The requested skin was not found.
    SkinNotFound,

    /// The requested animation was not found.
    AnimationNotFound,

    /// The requested bone was not found in the list of bones.
    ///
    /// This probably means that the Spine document contains an error.
    BoneNotFound(&'a str),

    /// The requested slot was not found.
    ///
    /// This probably means that the Spine document contains an error.
    SlotNotFound(&'a str),

    /// The requested attachment was not found.
    ///
    /// This probably means that the Spine document contains an error.
    AttachmentNotFound(&'a str),

    /// The curve function was not recognized.
    UnknownCurveFunction(String),
}

/// Informations about a bone's position.
///
/// Can be absolute or relative to its parent.
#[derive(Debug, Clone)]
struct BoneData {
    position: (f32, f32),
    rotation: f32,
    scale: (f32, f32),
}

impl BoneData {
    fn to_matrix(&self) -> Matrix4<f32> {
        use cgmath::{Matrix2, Vector3};

        let scale_matrix = Matrix4::new(self.scale.0, 0.0, 0.0, 0.0, 0.0, self.scale.1, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let rotation_matrix = Matrix4::from(Matrix2::from_angle(cgmath::deg(self.rotation).into()));
        let translation_matrix = Matrix4::from_translation(&Vector3::new(self.position.0, self.position.1, 0.0));

        translation_matrix * rotation_matrix * scale_matrix
    }
}

impl std::ops::Add<BoneData> for BoneData {
    type Output = BoneData;

    fn add(self, rhs: BoneData) -> BoneData {
        BoneData {
            position: (self.position.0 + rhs.position.0, self.position.1 + rhs.position.1),
            rotation: self.rotation + rhs.rotation,
            scale: (self.scale.0 * rhs.scale.0, self.scale.1 * rhs.scale.1),
        }
    }
}

/// Returns the setup pose of a bone relative to its parent.
fn get_bone_default_local_setup(bone: &json::Bone) -> BoneData {
    BoneData {
        position: (bone.x.unwrap_or(0.0), bone.y.unwrap_or(0.0)),
        rotation: bone.rotation.unwrap_or(0.0),
        scale: (bone.scaleX.unwrap_or(1.0), bone.scaleY.unwrap_or(1.0)),
    }
}

/// Returns the `Matrix` of an attachment.
fn get_attachment_transformation(attachment: &json::Attachment) -> Matrix4<f32> {
    BoneData {
        position: (attachment.x.unwrap_or(0.0), attachment.y.unwrap_or(0.0)),
        rotation: attachment.rotation.unwrap_or(0.0),
        scale: (
            attachment.scaleX.unwrap_or(1.0) * attachment.width.unwrap_or(1.0) / 2.0,
            attachment.scaleY.unwrap_or(1.0) * attachment.height.unwrap_or(1.0) / 2.0
        ),
    }.to_matrix()
}

/// Builds the `Matrix4` corresponding to a timeline.
fn timelines_to_bonedata(timeline: &json::BoneTimeline, elapsed: f32) -> Result<BoneData, CalculationError> {
    // calculating the current position
    let position = if let Some(timeline) = timeline.translate.as_ref() {
        // finding in which interval we are
        match timeline.iter().zip(timeline.iter().skip(1))
            .find(|&(before, after)| elapsed >= before.time && elapsed < after.time)
        {
            Some((ref before, ref after)) => {
                // calculating the value using the curve function
                let position = (elapsed - (before.time)) / ((after.time - before.time));

                (
                    try!(calculate_curve(&before.curve, before.x.unwrap_or(0.0),
                        after.x.unwrap_or(0.0), position)),
                    try!(calculate_curve(&before.curve, before.y.unwrap_or(0.0),
                        after.y.unwrap_or(0.0), position))
                )
            },
            None => {
                // we didn't find an interval, assuming we are past the end
                timeline.last().map(|t| (t.x.unwrap_or(0.0), t.y.unwrap_or(0.0)))
                    .unwrap_or((0.0, 0.0))
            }
        }

    } else {
        // we have no timeline
        (0.0, 0.0)
    };


    // calculating the current rotation
    let rotation = if let Some(timeline) = timeline.rotate.as_ref() {
        // finding in which interval we are
        match timeline.iter().zip(timeline.iter().skip(1))
            .find(|&(before, after)| elapsed >= before.time && elapsed < after.time)
        {
            Some((ref before, ref after)) => {
                // calculating the value using the curve function
                let position = (elapsed - (before.time)) / ((after.time - before.time));

                try!(calculate_curve(&before.curve, before.angle.unwrap_or(0.0),
                    after.angle.unwrap_or(0.0), position))
            },
            None => {
                // we didn't find an interval, assuming we are past the end
                timeline.last().map(|t| t.angle.unwrap_or(0.0))
                    .unwrap_or(0.0)
            }
        }

    } else {
        // we have no timeline
        0.0
    };


    // calculating the current scale
    let scale = if let Some(timeline) = timeline.scale.as_ref() {
        // finding in which interval we are
        match timeline.iter().zip(timeline.iter().skip(1))
            .find(|&(before, after)| elapsed >= before.time && elapsed < after.time)
        {
            Some((ref before, ref after)) => {
                // calculating the value using the curve function
                let position = (elapsed - (before.time)) / ((after.time - before.time));

                (
                    try!(calculate_curve(&before.curve, before.x.unwrap_or(1.0),
                        after.x.unwrap_or(1.0), position)),
                    try!(calculate_curve(&before.curve, before.y.unwrap_or(1.0),
                        after.y.unwrap_or(1.0), position))
                )
            },
            None => {
                // we didn't find an interval, assuming we are past the end
                timeline.last().map(|t| (t.x.unwrap_or(1.0), t.y.unwrap_or(1.0)))
                    .unwrap_or((1.0, 1.0))
            }
        }

    } else {
        // we have no timeline
        (1.0, 1.0)
    };


    // returning
    Ok(BoneData {
        position: position,
        rotation: rotation,
        scale: scale,
    })
}

/// Calculates a curve using the value of a "curve" member.
///
/// Position must be between 0 and 1
fn calculate_curve(formula: &Option<json::TimelineCurve>, from: f32, to: f32,
    position: f32) -> Result<f32, CalculationError>
{
    assert!(position >= 0.0 && position <= 1.0);

    let bezier = match formula {
        &None |
        &Some(json::TimelineCurve::CurveLinear) =>
            return Ok(from + position * (to - from)),
        &Some(json::TimelineCurve::CurveStepped) =>
            return Ok(from),
        &Some(json::TimelineCurve::CurveBezier(ref a)) => &a[..]
    };

    let (cx1, cy1, cx2, cy2) = match (bezier.get(0), bezier.get(1),
                                      bezier.get(2), bezier.get(3))
    {
        (Some(&cx1), Some(&cy1), Some(&cx2), Some(&cy2)) =>
            (cx1, cy1, cx2, cy2),
        a =>
            return Err(CalculationError::UnknownCurveFunction(format!("{:?}", a)))
    };

    let factor = (0 ..).map(|v| v as f32 * 0.02)
        .take_while(|v| *v <= 1.0)
        .map(|t| {
            let x = 3.0 * cx1 * t * (1.0 - t) * (1.0 - t)
                + 3.0 * cx2 * t * t * (1.0 - t) + t * t * t;
            let y = 3.0 * cy1 * t * (1.0 - t) * (1.0 - t)
                + 3.0 * cy2 * t * t * (1.0 - t) + t * t * t;

            (x, y)
        })
        .scan((0.0, 0.0), |previous, current| {
            let result = Some((previous.clone(), current));
            *previous = current;
            result
        })
        .find(|&(previous, current)| {
            position >= previous.0 && position < current.0
        })
        .map(|((_, val), _)| val)
        .unwrap_or(1.0);

    Ok(from + factor * (to - from))
}

/// Builds the color and attachment corresponding to a slot timeline.
fn timelines_to_slotdata(timeline: &json::SlotTimeline, elapsed: f32)
    -> Result<(Option<&str>, Option<&str>), CalculationError>
{
    // calculating the attachment
    let attachment = if let Some(timeline) = timeline.attachment.as_ref() {
        // finding in which interval we are
        match timeline.iter().zip(timeline.iter().skip(1))
            .find(|&(before, after)| elapsed >= before.time && elapsed < after.time)
        {
            Some((ref before, _)) => {
                before.name.as_ref().map(|e| &e[..])
            },
            None => {
                // we didn't find an interval, assuming we are past the end
                timeline.last().and_then(|t| (t.name.as_ref().map(|e| &e[..])))
            }
        }

    } else {
        // we have no timeline
        None
    };


    // calculating the color
    let color = if let Some(timeline) = timeline.color.as_ref() {
        // finding in which interval we are
        match timeline.iter().zip(timeline.iter().skip(1))
            .find(|&(before, after)| elapsed >= before.time && elapsed < after.time)
        {
            Some((ref before, _)) => {
                before.color.as_ref().map(|e| &e[..])
            },
            None => {
                // we didn't find an interval, assuming we are past the end
                timeline.last().and_then(|t| (t.color.as_ref().map(|e| &e[..])))
            }
        }

    } else {
        // we have no timeline
        None
    };


    // returning
    Ok((color, attachment))
}
