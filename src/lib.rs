#![feature(if_let)]
#![feature(phase)]
#![feature(tuple_indexing)]

#[phase(plugin)]
extern crate from_json_macros;

extern crate color;
extern crate cgmath;
extern crate from_json;
extern crate serialize;

use color::{Rgb, Rgba};
use cgmath::Matrix4;
use serialize::json;

mod format;

/// Spine document loaded in memory.
pub struct SpineDocument {
    source: format::Document,
}

impl SpineDocument {
    /// Loads a document from a reader.
    pub fn new<R: Reader>(reader: &mut R) -> Result<SpineDocument, String> {
        let document = try!(json::from_reader(reader).map_err(|e| e.to_string()));
        let document: format::Document = from_json::FromJson::from_json(&document).unwrap();

        Ok(SpineDocument {
            source: document
        })
    }

    /// Returns the list of all animations in this document.
    pub fn get_animations_list(&self) -> Vec<&str> {
        if let Some(ref list) = self.source.animations {
            list.keys().map(|e| e.as_slice()).collect()
        } else {
            Vec::new()
        }
    }

    /// Returns the list of all skins in this document.
    pub fn get_skins_list(&self) -> Vec<&str> {
        if let Some(ref list) = self.source.skins {
            list.keys().map(|e| e.as_slice()).collect()
        } else {
            Vec::new()
        }
    }

    /// Returns true if an animation is in the document.
    pub fn has_animation(&self, name: &str) -> bool {
        if let Some(ref list) = self.source.animations {
            list.find(&name.to_string()).is_some()
        } else {
            false
        }
    }

    /// Returns true if a skin is in the document.
    pub fn has_skin(&self, name: &str) -> bool {
        if let Some(ref list) = self.source.skins {
            list.find(&name.to_string()).is_some()
        } else {
            false
        }
    }

    /// Returns the duration of an animation.
    ///
    /// Returns `None` if the animation doesn't exist.
    /// 
    /// TODO: implement
    pub fn get_animation_duration(&self, animation: &str) -> Option<f32> {
        Some(1.0)
    }

    /// Returns a list of all possible sprites when drawing.
    ///
    /// The purpose of this function is to allow you to preload what you need.
    pub fn get_possible_sprites(&self) -> Vec<&str> {
        if let Some(ref list) = self.source.skins {
            let mut result = list.iter().flat_map(|(_, skin)| skin.iter())
                .flat_map(|(_, slot)| slot.keys()).map(|e| e.as_slice()).collect::<Vec<_>>();

            result.sort();
            result.dedup();
            result

        } else {
            Vec::new()
        }
    }

    /// Calculates the list of sprites that must be displayed and their matrix.
    // TODO: implement draw order timeline
    // TODO: implement events
    // TODO: implement other attachment types
    pub fn calculate(&self, skin: &str, animation: Option<&str>, mut elapsed: f32) 
        -> Result<Calculation, ()>
    {
        // adapting elapsed
        if let Some(animation) = animation {
            if let Some(duration) = self.get_animation_duration(animation) {
                elapsed = elapsed % duration;
            }
        }
        let elapsed = elapsed;

        // getting a reference to the `format::Skin`
        let skin = try!(self.source.skins.as_ref().and_then(|l| l.find_equiv(&skin)).ok_or(()));

        // getting a reference to the `format::Animation`
        let animation: Option<&format::Animation> = match animation {
            Some(animation) => Some(try!(self.source.animations.as_ref()
                .and_then(|l| l.find_equiv(&animation)).ok_or(()))),
            None => None
        };

        // calculating the default pose of all bones
        let mut bones: Vec<(&format::Bone, Matrix4<f32>)> = self.source.bones.as_ref().map(|bones| {
            bones.iter().map(|bone| (bone, get_bone_default_local_setup(bone))).collect()
        }).unwrap_or_else(|| Vec::new());

        // if we are animating, adding to the default pose the calculations from the animation
        if let Some(animation) = animation {
            if let Some(anim_bones) = animation.bones.as_ref() {
                for (bone_name, timelines) in anim_bones.iter() {
                    // calculating the variation from the animation
                    let anim_data = try!(timelines_to_bonedata(timelines, elapsed));

                    // adding this to the `bones` vec above
                    match bones.iter_mut().find(|&&(b, _)| b.name == *bone_name) {
                        Some(&(_, ref mut data)) => { *data = *data * anim_data; },
                        None => ()
                    };
                }
            }
        };

        // now we have our list of bones with their relative positions
        // adding the position of the parent to each bone
        let bones: Vec<(&str, Matrix4<f32>)> = bones.iter().map(|&(ref bone, ref relative_data)| {
            let mut current_matrix = relative_data.clone();
            let mut current_parent = bone.parent.as_ref();

            loop {
                if let Some(parent_name) = current_parent {
                    assert!(parent_name != &bone.name);     // prevent infinite loop

                    match bones.iter().find(|&&(b, _)| b.name == *parent_name) {
                        Some(ref p) => {
                            current_parent = p.0.parent.as_ref();
                            current_matrix = p.1 * current_matrix;
                        },
                        None => {
                            current_parent = None       // TODO: return error instead
                        }
                    }

                } else {
                    break
                }
            }

            (bone.name.as_slice(), current_matrix.clone())

        }).collect();

        // now taking each slot in the document and matching its bone
        // `slots` contains the slot name, bone data, color, and attachment
        let mut slots: Vec<(&str, Matrix4<f32>, Option<&str>, Option<&str>)> =
            if let Some(slots) = self.source.slots.as_ref() {
                let mut result = Vec::new();

                for slot in slots.iter() {
                    let bone = try!(bones.iter().find(|&&(name, _)| name == slot.bone.as_slice())
                        .ok_or(()));
                    result.push((slot.name.as_slice(), bone.1, slot.color.as_ref()
                        .map(|s| s.as_slice()), slot.attachment.as_ref().map(|s| s.as_slice())))
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
                    match slots.iter_mut().find(|&&(s, _, _, _)| s == slot_name.as_slice()) {
                        Some(&(_, _, ref mut color, ref mut attachment)) => {
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

            for (slot_name, bone_data, color, attachment) in slots.into_iter() {
                let attachments = try!(skin.iter().find(|&(slot, _)| slot.as_slice() == slot_name)
                    .ok_or(()));
                let attachment = try!(attachments.1.iter()
                    .find(|&(a, _)| Some(a.as_slice()) == attachment).ok_or(()));

                let attachment_transform = get_attachment_transformation(attachment.1);
                let bone_data = (bone_data * attachment_transform);

                results.push((
                    attachment.0.as_slice(),
                    bone_data,
                    Rgba { a: 255, c: Rgb::new(255, 255, 255) }
                ));
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
#[deriving(Show)]
pub struct Calculation<'a> {
    /// The list of sprites that should be drawn.
    ///
    /// The elements are sorted from bottom to top, ie. each element can cover the previous one.
    ///
    /// The matrix assumes that the sprite is displayed from (-1, -1) to (1, 1), ie. would cover
    ///  the whole screen.
    pub sprites: Vec<(&'a str, Matrix4<f32>, Rgba<u8>)>,
}

/// Creates a matrix from a position, rotation, and scale.
///
/// The matrix is pre-multiplying.
fn to_matrix(position: (f32, f32), rotation: f32, scale: (f32, f32)) -> Matrix4<f32> {
    use cgmath::{Matrix2, Vector3, ToMatrix4, ToRad};
    use std::num::FloatMath;

    let scale_matrix = Matrix4::new(scale.0, 0.0, 0.0, 0.0, 0.0, scale.1, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);

    let rotation_matrix = Matrix2::from_angle(cgmath::deg(rotation).to_rad()).to_matrix4();

    let translation_matrix = Matrix4::from_translation(&Vector3::new(position.0, position.1, 0.0));

    translation_matrix * rotation_matrix * scale_matrix
}

/// Returns the setup pose of a bone relative to its parent.
fn get_bone_default_local_setup(bone: &format::Bone) -> Matrix4<f32> {
    to_matrix(
        (bone.x.unwrap_or(0.0) as f32, bone.y.unwrap_or(0.0) as f32),
        bone.rotation.unwrap_or(0.0) as f32,
        (bone.scaleX.unwrap_or(1.0) as f32, bone.scaleY.unwrap_or(1.0) as f32)
    )
}

/// Returns the `Matrix` of an attachment.
fn get_attachment_transformation(attachment: &format::Attachment) -> Matrix4<f32> {
    to_matrix(
        (attachment.x.unwrap_or(0.0) as f32, attachment.y.unwrap_or(0.0) as f32),
        attachment.rotation.unwrap_or(0.0) as f32,
        (
            attachment.scaleX.unwrap_or(1.0) as f32 * attachment.width.unwrap_or(1.0) as f32 / 2.0,
            attachment.scaleY.unwrap_or(1.0) as f32 * attachment.height.unwrap_or(1.0) as f32 / 2.0
        )
    )
}

/// Builds the `Matrix4` corresponding to a timeline.
fn timelines_to_bonedata(timeline: &format::BoneTimeline, elapsed: f32) -> Result<Matrix4<f32>, ()> {
    // calculating the current position
    let position = if let Some(timeline) = timeline.translate.as_ref() {
        // finding in which interval we are
        match timeline.iter().zip(timeline.iter().skip(1))
            .find(|&(before, after)| elapsed >= before.time as f32 && elapsed < after.time as f32)
        {
            Some((ref before, ref after)) => {
                // calculating the value using the curve function
                let position = (elapsed - (before.time as f32)) / ((after.time - before.time) as f32);

                (
                    try!(calculate_curve(&before.curve, before.x.unwrap_or(0.0) as f32,
                        after.x.unwrap_or(0.0) as f32, position)),
                    try!(calculate_curve(&before.curve, before.y.unwrap_or(0.0) as f32,
                        after.y.unwrap_or(0.0) as f32, position))
                )
            },
            None => {
                // we didn't find an interval, assuming we are past the end
                timeline.last().map(|t| (t.x.unwrap_or(0.0) as f32, t.y.unwrap_or(0.0) as f32))
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
            .find(|&(before, after)| elapsed >= before.time as f32 && elapsed < after.time as f32)
        {
            Some((ref before, ref after)) => {
                // calculating the value using the curve function
                let position = (elapsed - (before.time as f32)) / ((after.time - before.time) as f32);

                try!(calculate_curve(&before.curve, before.angle.unwrap_or(0.0) as f32,
                    after.angle.unwrap_or(0.0) as f32, position))
            },
            None => {
                // we didn't find an interval, assuming we are past the end
                timeline.last().map(|t| t.angle.unwrap_or(0.0) as f32)
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
            .find(|&(before, after)| elapsed >= before.time as f32 && elapsed < after.time as f32)
        {
            Some((ref before, ref after)) => {
                // calculating the value using the curve function
                let position = (elapsed - (before.time as f32)) / ((after.time - before.time) as f32);

                (
                    try!(calculate_curve(&before.curve, before.x.unwrap_or(1.0) as f32,
                        after.x.unwrap_or(1.0) as f32, position)),
                    try!(calculate_curve(&before.curve, before.y.unwrap_or(1.0) as f32,
                        after.y.unwrap_or(1.0) as f32, position))
                )
            },
            None => {
                // we didn't find an interval, assuming we are past the end
                timeline.last().map(|t| (t.x.unwrap_or(1.0) as f32, t.y.unwrap_or(1.0) as f32))
                    .unwrap_or((1.0, 1.0))
            }
        }

    } else {
        // we have no timeline
        (1.0, 1.0)
    };

    
    // returning
    Ok(to_matrix(position, rotation, scale))
}

///
/// Position must be between 0 and 1
// TODO: not implemented correctly
fn calculate_curve(formula: &Option<format::TimelineCurve>, from: f32, to: f32,
    position: f32) -> Result<f32, ()>
{
    let bezier = match formula {
        &None =>
            return Ok(from + position * (to - from)),
        &Some(format::CurvePredefined(ref a)) if a.as_slice() == "linear" =>
            return Ok(from + position * (to - from)),
        &Some(format::CurvePredefined(ref a)) if a.as_slice() == "stepped" =>
            return Ok(from),
        &Some(format::CurveBezier(ref a)) => a.as_slice(),
        _ => return Err(()),
    };
    
    let (cx1, cy1, cx2, cy2) = match (bezier.get(0), bezier.get(1),
                                      bezier.get(2), bezier.get(3))
    {
        (Some(cx1), Some(cy1), Some(cx2), Some(cy2)) => (cx1, cy1, cx2, cy2),
        _ => return Err(())
    };

    Ok(from + position * (to - from))
}

/// Builds the color and attachment corresponding to a slot timeline.
fn timelines_to_slotdata(timeline: &format::SlotTimeline, elapsed: f32)
    -> Result<(Option<&str>, Option<&str>), ()>
{
    // calculating the attachment
    let attachment = if let Some(timeline) = timeline.attachment.as_ref() {
        // finding in which interval we are
        match timeline.iter().zip(timeline.iter().skip(1))
            .find(|&(before, after)| elapsed >= before.time as f32 && elapsed < after.time as f32)
        {
            Some((ref before, ref after)) => {
                before.name.as_ref().map(|e| e.as_slice())
            },
            None => {
                // we didn't find an interval, assuming we are past the end
                timeline.last().and_then(|t| (t.name.as_ref().map(|e| e.as_slice())))
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
            .find(|&(before, after)| elapsed >= before.time as f32 && elapsed < after.time as f32)
        {
            Some((ref before, ref after)) => {
                before.color.as_ref().map(|e| e.as_slice())
            },
            None => {
                // we didn't find an interval, assuming we are past the end
                timeline.last().and_then(|t| (t.color.as_ref().map(|e| e.as_slice())))
            }
        }

    } else {
        // we have no timeline
        None
    };

    
    // returning
    Ok((color, attachment))
}

/// Parses a color from a string.
fn parse_color(input: &str) -> Result<Rgba<u8>, ()> {
    unimplemented!()
}
