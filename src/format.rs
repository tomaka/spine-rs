#![allow(dead_code)]
#![allow(non_snake_case)]

use from_json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Document {
    pub bones: Option<Vec<Bone>>,
    pub slots: Option<Vec<Slot>>,
    pub skins: Option<HashMap<String, HashMap<String, HashMap<String, Attachment>>>>,
    pub animations: Option<HashMap<String, Animation>>,
}

derive_from_json!(Document, bones, slots, skins, animations);

#[derive(Debug, Clone)]
pub struct Bone {
    pub name: String,
    pub parent: Option<String>,
    pub length: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub scaleX: Option<f64>,
    pub scaleY: Option<f64>,
    pub rotation: Option<f64>,
}

derive_from_json!(Bone, name, parent, length, x, y, scaleX, scaleY, rotation);

#[derive(Debug, Clone)]
pub struct Slot {
    pub name: String,
    pub bone: String,
    pub color: Option<String>,
    pub attachment: Option<String>,
}

derive_from_json!(Slot, name, bone, color, attachment);

#[derive(Debug, Clone)]
pub struct Attachment {
    pub name: Option<String>,
    pub type_: Option<AttachmentType>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub scaleX: Option<f64>,
    pub scaleY: Option<f64>,
    pub rotation: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub fps: Option<f64>,
    pub mode: Option<f64>,
    //vertices: Option<Vec<??>>     // TODO: ?
}

derive_from_json!(Attachment, name, type_ as "type", x, y, scaleX, scaleY, rotation, width, height,
                  fps, mode);

#[derive(Debug, Clone)]
pub enum AttachmentType {
    Region,
    RegionSequence,
    BoundingBox,
}

impl from_json::FromJson for AttachmentType {
    fn from_json(input: &from_json::Json) -> Result<AttachmentType, from_json::FromJsonError> {
        use from_json::FromJson;

        let string: String = try!(FromJson::from_json(input));

        match &string[..] {
            "region" => Ok(AttachmentType::Region),
            "regionsequence" => Ok(AttachmentType::RegionSequence),
            "boundingbox" => Ok(AttachmentType::BoundingBox),
            _ => Err(from_json::FromJsonError::ExpectError("AttachmentType", input.clone()))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub int_: Option<i32>,
    pub float_: Option<f64>,
    pub string: Option<String>,
}

derive_from_json!(Event, name, int_ as "int", float_ as "float", string);

#[derive(Debug, Clone)]
pub struct Animation {
    pub bones: Option<HashMap<String, BoneTimeline>>,
    pub slots: Option<HashMap<String, SlotTimeline>>,
    pub events: Option<Vec<EventKeyframe>>,
    pub draworder: Option<Vec<DrawOrderTimeline>>,
}

derive_from_json!(Animation, bones, slots, events, draworder);

#[derive(Debug, Clone)]
pub struct BoneTimeline {
    pub translate: Option<Vec<BoneTranslateTimeline>>,
    pub rotate: Option<Vec<BoneRotateTimeline>>,
    pub scale: Option<Vec<BoneScaleTimeline>>,
}

derive_from_json!(BoneTimeline, translate, rotate, scale);

#[derive(Debug, Clone)]
pub struct BoneTranslateTimeline {
    pub time: f64,
    pub curve: Option<TimelineCurve>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

derive_from_json!(BoneTranslateTimeline, time, curve, x, y);

#[derive(Debug, Clone)]
pub struct BoneRotateTimeline {
    pub time: f64,
    pub curve: Option<TimelineCurve>,
    pub angle: Option<f64>,
}

derive_from_json!(BoneRotateTimeline, time, curve, angle);

#[derive(Debug, Clone)]
pub struct BoneScaleTimeline {
    pub time: f64,
    pub curve: Option<TimelineCurve>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

derive_from_json!(BoneScaleTimeline, time, curve, x, y);

#[derive(Debug, Clone)]
pub enum TimelineCurve {
    CurveBezier(Vec<f64>),
    CurvePredefined(String),
}

impl from_json::FromJson for TimelineCurve {
    fn from_json(input: &from_json::Json) -> Result<TimelineCurve, from_json::FromJsonError> {
        use from_json::FromJson;

        if input.is_array() {
            Ok(TimelineCurve::CurveBezier(try!(FromJson::from_json(input))))
        } else {
            Ok(TimelineCurve::CurvePredefined(try!(FromJson::from_json(input))))
        }
    }
}

#[derive(Debug, Clone)]
pub struct SlotTimeline {
    pub attachment: Option<Vec<SlotAttachmentTimeline>>,
    pub color: Option<Vec<SlotColorTimeline>>,
}

derive_from_json!(SlotTimeline, attachment, color);

#[derive(Debug, Clone)]
pub struct SlotAttachmentTimeline {
    pub time: f64,
    pub name: Option<String>,
}

derive_from_json!(SlotAttachmentTimeline, time, name);

#[derive(Debug, Clone)]
pub struct SlotColorTimeline {
    pub time: f64,
    pub color: Option<String>,
    pub curve: Option<TimelineCurve>,
}

derive_from_json!(SlotColorTimeline, time, color, curve);

#[derive(Debug, Clone)]
pub struct EventKeyframe {
    time: f64,
    name: String,
    int_: Option<i32>,
    float_: Option<f64>,
    string_: Option<String>,
}

derive_from_json!(EventKeyframe, time, name, int_ as "int", float_ as "float",
                  string_ as "string");

#[derive(Debug, Clone)]
pub struct DrawOrderTimeline {
    time: f64,
    offsets: Option<Vec<DrawOrderTimelineOffset>>,
}

derive_from_json!(DrawOrderTimeline, time, offsets);

#[derive(Debug, Clone)]
pub struct DrawOrderTimelineOffset {
    slot: String,
    offset: i32,
}

derive_from_json!(DrawOrderTimelineOffset, slot, offset);
