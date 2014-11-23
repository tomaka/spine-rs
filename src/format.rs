#![allow(dead_code)]
#![allow(non_snake_case)]

use from_json;
use serialize;
use std::collections::HashMap;

#[deriving(Show, Clone)]
#[from_json_struct]
pub struct Document {
    pub bones: Option<Vec<Bone>>,
    pub slots: Option<Vec<Slot>>,
    pub skins: Option<HashMap<String, HashMap<String, HashMap<String, Attachment>>>>,
    pub animations: Option<HashMap<String, Animation>>,
}

#[deriving(Show, Clone)]
#[from_json_struct]
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

#[deriving(Show, Clone)]
#[from_json_struct]
pub struct Slot {
    pub name: String,
    pub bone: String,
    pub color: Option<String>,
    pub attachment: Option<String>,
}

#[deriving(Show, Clone)]
#[from_json_struct]
pub struct Attachment {
    pub name: Option<String>,
    #[from_json_name = "type"]
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

#[deriving(Show, Clone)]
pub enum AttachmentType {
    Region,
    RegionSequence,
    BoundingBox,
}

impl from_json::FromJson for AttachmentType {
    fn from_json(input: &serialize::json::Json) -> Result<AttachmentType, from_json::FromJsonError> {
        use from_json::FromJson;

        let string: String = try!(FromJson::from_json(input));

        match string.as_slice() {
            "region" => Ok(AttachmentType::Region),
            "regionsequence" => Ok(AttachmentType::RegionSequence),
            "boundingbox" => Ok(AttachmentType::BoundingBox),
            _ => Err(from_json::ExpectError("AttachmentType", input.clone()))
        }
    }
}

#[deriving(Show, Clone)]
#[from_json_struct]
pub struct Event {
    pub name: String,
    #[from_json_name = "int"]
    pub int_: Option<int>,
    #[from_json_name = "float"]
    pub float_: Option<f64>,
    pub string: Option<String>,
}

#[deriving(Show, Clone)]
#[from_json_struct]
pub struct Animation {
    pub bones: Option<HashMap<String, BoneTimeline>>,
    pub slots: Option<HashMap<String, SlotTimeline>>,
    pub events: Option<Vec<EventKeyframe>>,
    pub draworder: Option<Vec<DrawOrderTimeline>>,
}

#[deriving(Show, Clone)]
#[from_json_struct]
pub struct BoneTimeline {
    pub translate: Option<Vec<BoneTranslateTimeline>>,
    pub rotate: Option<Vec<BoneRotateTimeline>>,
    pub scale: Option<Vec<BoneScaleTimeline>>,
}

#[deriving(Show, Clone)]
#[from_json_struct]
pub struct BoneTranslateTimeline {
    pub time: f64,
    pub curve: Option<TimelineCurve>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[deriving(Show, Clone)]
#[from_json_struct]
pub struct BoneRotateTimeline {
    pub time: f64,
    pub curve: Option<TimelineCurve>,
    pub angle: Option<f64>,
}

#[deriving(Show, Clone)]
#[from_json_struct]
pub struct BoneScaleTimeline {
    pub time: f64,
    pub curve: Option<TimelineCurve>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[deriving(Show, Clone)]
pub enum TimelineCurve {
    CurveBezier(Vec<f64>),
    CurvePredefined(String),
}

impl from_json::FromJson for TimelineCurve {
    fn from_json(input: &serialize::json::Json) -> Result<TimelineCurve, from_json::FromJsonError> {
        use from_json::FromJson;

        if input.is_array() {
            Ok(TimelineCurve::CurveBezier(try!(FromJson::from_json(input))))
        } else {
            Ok(TimelineCurve::CurvePredefined(try!(FromJson::from_json(input))))
        }
    }
}

#[deriving(Show, Clone)]
#[from_json_struct]
pub struct SlotTimeline {
    pub attachment: Option<Vec<SlotAttachmentTimeline>>,
    pub color: Option<Vec<SlotColorTimeline>>,
}

#[deriving(Show, Clone)]
#[from_json_struct]
pub struct SlotAttachmentTimeline {
    pub time: f64,
    pub name: Option<String>,
}

#[deriving(Show, Clone)]
#[from_json_struct]
pub struct SlotColorTimeline {
    pub time: f64,
    pub color: Option<String>,
    pub curve: Option<TimelineCurve>,
}

#[deriving(Show, Clone)]
#[from_json_struct]
pub struct EventKeyframe {
    time: f64,
    name: String,
    #[from_json_name = "int"]
    int_: Option<int>,
    #[from_json_name = "float"]
    float_: Option<f64>,
    #[from_json_name = "string"]
    string_: Option<String>,
}

#[deriving(Show, Clone)]
#[from_json_struct]
pub struct DrawOrderTimeline {
    time: f64,
    offsets: Option<Vec<DrawOrderTimelineOffset>>,
}

#[deriving(Show, Clone)]
#[from_json_struct]
pub struct DrawOrderTimelineOffset {
    slot: String,
    offset: int,
}
