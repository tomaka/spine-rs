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

#![cfg_attr(feature = "serde_macros", feature(custom_derive, plugin, custom_attribute, type_macros))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate color;
extern crate cgmath;
extern crate serde;
extern crate serde_json;

mod format;

use color::{Rgb, Rgba};
use cgmath::Matrix4;
use std::io::Read;

/// Spine document loaded in memory.
pub struct SpineDocument {
    source: format::Document,
}

impl SpineDocument {
    /// Loads a document from a reader.
    pub fn new<R: Read>(reader: R) -> Result<SpineDocument, String> {

        let document: format::Document = try!(serde_json::from_reader(reader)
                                              .map_err(|e| format!("{:?}", e)));

        Ok(SpineDocument {
            source: document
        })
    }

    /// Returns the list of all animations in this document.
    pub fn get_animations_list(&self) -> Vec<&str> {
        self.source.animations.keys().map(|e| &e[..]).collect()
    }

    /// Returns the list of all skins in this document.
    pub fn get_skins_list(&self) -> Vec<&str> {
        self.source.skins.keys().map(|e| &e[..]).collect()
    }

    /// Returns true if an animation is in the document.
    pub fn has_animation(&self, name: &str) -> bool {
         self.source.animations.contains_key(name)
    }

    /// Returns true if a skin is in the document.
    pub fn has_skin(&self, name: &str) -> bool {
         self.source.skins.contains_key(name)
    }

    /// Returns the duration of an animation.
    ///
    /// Returns `None` if the animation doesn't exist.
    ///
    /// TODO: check events and draworder?
    pub fn get_animation_duration(&self, animation: &str) -> Option<f32> {
        // getting a reference to the `format::Animation`
        let animation: &format::Animation =
            match self.source.animations.get(animation) {
                Some(a) => a,
                None => return None
            };

        // this contains the final result
        let mut result = 0.0f32;

        // checking the bones
        for timelines in animation.bones.values() {
            for elem in timelines.translate.iter() {
                if elem.time > result { result = elem.time }
            }
            for elem in timelines.rotate.iter() {
                if elem.time > result { result = elem.time }
            }
            for elem in timelines.scale.iter() {
                if elem.time > result { result = elem.time }
            }
        }

        // checking the slots
        for timelines in animation.slots.values() {
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

        // returning
        Some(result as f32)
    }

    /// Returns a list of all possible sprites when drawing.
    ///
    /// The purpose of this function is to allow you to preload what you need.
    pub fn get_possible_sprites(&self) -> Vec<&str> {
        let mut result = self.source.skins.iter().flat_map(|(_, skin)| skin.iter())
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

        // getting a reference to the `format::Skin`
        let skin = try!(self.source.skins.get(skin).ok_or(CalculationError::SkinNotFound));

        // getting a reference to "default" skin
        let default_skin = try!(self.source.skins.get("default").ok_or(CalculationError::SkinNotFound));

        // getting a reference to the `format::Animation`
        let animation: Option<&format::Animation> = match animation {
            Some(animation) => Some(try!(self.source.animations.get(animation)
                                         .ok_or(CalculationError::AnimationNotFound))),
            None => None
        };

        // calculating the default pose of all bones
        let mut bones: Vec<(&format::Bone, BoneData)> = self.source.bones.iter()
            .map(|bone| (bone, get_bone_default_local_setup(bone))).collect();

        // if we are animating, adding to the default pose the calculations from the animation
        if let Some(animation) = animation {
            for (bone_name, timelines) in animation.bones.iter() {
                // calculating the variation from the animation
                let anim_data = try!(timelines_to_bonedata(timelines, elapsed));

                // adding this to the `bones` vec above
                match bones.iter_mut().find(|&&mut (b, _)| b.name == *bone_name) {
                    Some(&mut (_, ref mut data)) => { *data = data.clone() + anim_data; },
                    None => ()
                };
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
        let mut slots: Vec<(&str, Matrix4<f32>, Option<&str>, Option<&str>)> ={
            let mut result = Vec::new();

            for slot in self.source.slots.iter() {
                let bone = try!(bones.iter().find(|&&(name, _)| name == slot.bone)
                    .ok_or(CalculationError::BoneNotFound(&slot.bone)));
                result.push((&slot.name[..], bone.1, slot.color.as_ref()
                    .map(|s| &s[..]), slot.attachment.as_ref().map(|s| &s[..])))
            }

            result
        };

        // if we are animating, replacing the values by the ones overridden by the animation
        if let Some(animation) = animation {
            for (slot_name, timelines) in animation.slots.iter() {
                // calculating the variation from the animation
                let (anim_color, anim_attach) = timelines_to_slotdata(timelines, elapsed);

                // adding this to the `slots` vec above
                match slots.iter_mut().find(|&&mut (s, _, _, _)| s == slot_name) {
                    Some(&mut (_, _, ref mut color, ref mut attachment)) => {
                        if let Some(c) = anim_color { *color = Some(c) };
                        if let Some(a) = anim_attach { *attachment = Some(a) };
                    },
                    None => ()
                };
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
fn get_bone_default_local_setup(bone: &format::Bone) -> BoneData {
    BoneData {
        position: (bone.x , bone.y),
        rotation: bone.rotation,
        scale: (bone.scaleX.unwrap_or(1.0) as f32, bone.scaleY.unwrap_or(1.0) as f32),
    }
}

/// Returns the `Matrix` of an attachment.
fn get_attachment_transformation(attachment: &format::Attachment) -> Matrix4<f32> {
    BoneData {
        position: (attachment.x, attachment.y),
        rotation: attachment.rotation,
        scale: (
            attachment.scaleX.unwrap_or(1.0) as f32 * attachment.width / 2.0,
            attachment.scaleY.unwrap_or(1.0) as f32 * attachment.height / 2.0
        ),
    }.to_matrix()
}

/// Builds the `Matrix4` corresponding to a timeline.
fn timelines_to_bonedata(timeline: &format::BoneTimeline, elapsed: f32) -> Result<BoneData, CalculationError> {

    // calculating the current position
    // finding in which interval we are
    let position = match timeline.translate.iter().zip(timeline.translate.iter().skip(1))
        .find(|&(before, after)| elapsed >= before.time && elapsed < after.time)
    {
        Some((ref before, ref after)) => {
            // calculating the value using the curve function
            let position = (elapsed - (before.time)) / ((after.time - before.time) as f32);

            (
                try!(calculate_curve(&before.curve, before.x,
                    after.x, position)),
                try!(calculate_curve(&before.curve, before.y,
                    after.y, position))
            )
        },
        None => {
            // we didn't find an interval, assuming we are past the end
            timeline.translate.last().map(|t| (t.x, t.y))
                .unwrap_or((0.0, 0.0))
        }
    };


    // calculating the current rotation
    // finding in which interval we are
    let rotation = match timeline.rotate.iter().zip(timeline.rotate.iter().skip(1))
        .find(|&(before, after)| elapsed >= before.time && elapsed < after.time)
    {
        Some((ref before, ref after)) => {
            // calculating the value using the curve function
            let position = (elapsed - (before.time)) / ((after.time - before.time) as f32);
            try!(calculate_curve(&before.curve, before.angle, after.angle, position))
        },
        None => {
            // we didn't find an interval, assuming we are past the end
            timeline.rotate.last().map(|t| t.angle).unwrap_or(0.0)
        }
    };

    // calculating the current scale
    // finding in which interval we are
    let scale = match timeline.scale.iter().zip(timeline.scale.iter().skip(1))
        .find(|&(before, after)| elapsed >= before.time && elapsed < after.time)
    {
        Some((ref before, ref after)) => {
            // calculating the value using the curve function
            let position = (elapsed - (before.time)) / ((after.time - before.time) as f32);

            (
                try!(calculate_curve(&before.curve, before.x.unwrap_or(1.0) as f32,
                    after.x.unwrap_or(1.0) as f32, position)),
                try!(calculate_curve(&before.curve, before.y.unwrap_or(1.0) as f32,
                    after.y.unwrap_or(1.0) as f32, position))
            )
        },
        None => {
            // we didn't find an interval, assuming we are past the end
            timeline.scale.last().map(|t| (t.x.unwrap_or(1.0) as f32, t.y.unwrap_or(1.0) as f32))
                .unwrap_or((1.0, 1.0))
        }
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
fn calculate_curve(formula: &format::Curve, from: f32, to: f32,
    position: f32) -> Result<f32, CalculationError>
{
    assert!(position >= 0.0 && position <= 1.0);

    match *formula {
        format::Curve::Linear  => Ok(from + position * (to - from)),
        format::Curve::Stepped => Ok(from),
        format::Curve::Bezier(cx1, cy1, cx2, cy2) => {
            let factor = (0 ..).map(|v| v as f32 * 0.02)
                .take_while(|v| *v <= 1.0)
                .map(|t| {
                    let x = 3f32 * cx1 * t * (1.0f32 - t) * (1.0f32 - t)
                        + 3f32 * cx2 * t * t * (1.0f32 - t) + t * t * t;
                    let y = 3f32 * cy1 * t * (1.0f32 - t) * (1.0f32 - t)
                        + 3f32 * cy2 * t * t * (1.0f32 - t) + t * t * t;

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
    }
}

/// Builds the color and attachment corresponding to a slot timeline.
fn timelines_to_slotdata(timeline: &format::SlotTimeline, elapsed: f32) -> (Option<&str>, Option<&str>)
{
    // calculating the attachment
    let attachment = timeline.attachment.as_ref().and_then(|timeline|
        timeline.windows(2)
                 // finding in which interval we are
                 .find(|&w| w[0].time <= elapsed && elapsed < w[1].time)
                 .map(|ref w| &*w[0].name)
                 .or_else(|| timeline.last().map(|ref t| &*t.name))
    );

    let color = timeline.color.as_ref().and_then(|timeline|
        timeline.windows(2)
                 // finding in which interval we are
                 .find(|&w| w[0].time <= elapsed && elapsed < w[1].time)
                 .map(|ref w| &*w[0].color)
                 .or_else(|| timeline.last().map(|ref t| &*t.color))
    );

    // returning
    (color, attachment)
}
