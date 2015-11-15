//! # spine
//!
//! Parses a Spine document and calculates what needs to be drawn.
//!
//! ## Step 1: loading the document
//!
//! Call `skeleton::Skeleton::from_reader` to parse the content of a document.
//!
//! This function returns an `Err` if the document is not valid JSON or if something is not
//!  recognized in it.
//!
//! ```no_run
//! # use std::fs::File;
//! # use std::path::Path;
//! let skeleton = spine::skeleton::Skeleton::from_reader(File::open(&Path::new("skeleton.json")).unwrap())
//!     .unwrap();
//! ```
//!
//! ## Step 2: preparing for drawing
//!
//! You can retreive the list of animations and skins provided a document:
//!
//! ```no_run
//! # let skeleton: spine::skeleton::Skeleton = unsafe { std::mem::uninitialized() };
//! let skins = skeleton.get_skins_names();
//! let animations = skeleton.get_animations_names();
//! ```
//!
//! You can also get a list of the names of all the sprites that can possibly be drawn by this
//!  Spine animation.
//!
//! ```no_run
//! # let skeleton: spine::skeleton::Skeleton = unsafe { std::mem::uninitialized() };
//! let sprites = skeleton.get_attachments_names();
//! ```
//!
//! Note that the names do not necessarly match file names. They are the same names that you have in
//!  the Spine editor. It is your job to turn these resource names into file names if necessary.
//!
//! ## Step 3: animating
//!
//! To run an animation, you first need to call `skeleton.get_animated_skin` to get a `SkinAnimation`.
//! You then have 2 methods to get the sprites you need to draw:
//! - directly call `animation.interpolate` for a given time
//! - use a built-in `AnimationIter` iterator by calling `animation.run()` to run the animation
//! with a constant period
//!
//! Both methods returns a `Sprites` iterator over the `Sprite`s do be drawn.
//!
//! ```no_run
//! # let skeleton: spine::skeleton::Skeleton = unsafe { std::mem::uninitialized() };
//! let animation = skeleton.get_animated_skin("default", Some("walk")).unwrap();
//! let sprites = animation.interpolate(0.3).unwrap();
//! // equivalent to
//! let sprites = animation
//!               .run(0.1)  // iterator that interpolates sprites every 0.1s
//!               .nth(3);   // get the 3rd item generated when time = 0.3
//! ```
//!
//! The result contains an iterator over the sprites that need to be drawn with the `skeleton::SRT`
//! (scale, rotate, translate)). The srt supposes that each sprite would cover the whole viewport
//! (ie. drawn from `(-1, -1)` to `(1, 1)`).  You can convert it to a premultiplied matrix using
//! `srt.to_matrix3()` or `srt.to_matrix4` ; if you want to apply your own matrix `C` over
//!  the one returned `M`, you need to call `C * M`.
//!
//! ```no_run
//! # use std::collections::HashMap;
//! # let skeleton: spine::skeleton::Skeleton = unsafe { std::mem::uninitialized() };
//! # let textures_list: HashMap<&str, i32> = unsafe { std::mem::uninitialized() };
//! # let animation = skeleton.get_animated_skin("default", Some("walk")).unwrap();
//! # fn draw<A,B,C>(_: A, _: B, _: C) {}
//! for sprite in animation.interpolate(0.3).unwrap() {
//!     let texture = textures_list.get(&&*sprite.attachment).unwrap();
//!     draw(texture, &sprite.srt, &sprite.color);
//! }
//! ```
//!

#![deny(missing_docs)]

#[macro_use]
extern crate from_json;
extern crate rustc_serialize as serialize;

mod json;
pub mod skeleton;
pub mod atlas;
