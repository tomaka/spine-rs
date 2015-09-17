#![allow(dead_code)]
#![allow(non_snake_case)]

use std::collections::HashMap;
// use serde::de::{Deserialize, Deserializer, Error, Visitor, SeqVisitor};

#[derive(Deserialize, Debug, Clone)]
pub struct Document {
    pub bones: Option<Vec<Bone>>,
    pub slots: Option<Vec<Slot>>,
    pub skins: Option<HashMap<String, HashMap<String, HashMap<String, Attachment>>>>,
    pub animations: Option<HashMap<String, Animation>>,
}

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
pub struct Slot {
    pub name: String,
    pub bone: String,
    pub color: Option<String>,
    pub attachment: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Attachment {
    pub name: Option<String>,
    #[serde(alias="type")]
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

// #[derive(Deserialize, Debug, Clone)]
// pub enum AttachmentType {
//     Region,
//     RegionSequence,
//     BoundingBox,
// }

//
// impl from_json::FromJson for AttachmentType {
//     fn from_json(input: &from_json::Json) -> Result<AttachmentType, from_json::FromJsonError> {
//         use from_json::FromJson;
//
//         let string: String = try!(FromJson::from_json(input));
//
//         match &string[..] {
//             "region" => Ok(AttachmentType::Region),
//             "regionsequence" => Ok(AttachmentType::RegionSequence),
//             "boundingbox" => Ok(AttachmentType::BoundingBox),
//             _ => Err(from_json::FromJsonError::ExpectError("AttachmentType", input.clone()))
//         }
//     }
// }

#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    pub name: String,
    #[serde(alias="int")]
    pub int_: Option<i32>,
    #[serde(alias="float")]
    pub float_: Option<f64>,
    pub string: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Animation {
    pub bones: Option<HashMap<String, BoneTimeline>>,
    pub slots: Option<HashMap<String, SlotTimeline>>,
    pub events: Option<Vec<EventKeyframe>>,
    pub draworder: Option<Vec<DrawOrderTimeline>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BoneTimeline {
    pub translate: Option<Vec<BoneTranslateTimeline>>,
    pub rotate: Option<Vec<BoneRotateTimeline>>,
    pub scale: Option<Vec<BoneScaleTimeline>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BoneTranslateTimeline {
    pub time: f64,
    pub curve: Option<TimelineCurve>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

pub type TimelineCurve = ::serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
pub struct BoneRotateTimeline {
    pub time: f64,
    pub curve: Option<TimelineCurve>,
    pub angle: Option<f64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BoneScaleTimeline {
    pub time: f64,
    pub curve: Option<TimelineCurve>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum TimelineCurve {
//     CurveBezier(Vec<f64>),
//     CurvePredefined(String),
// }
//
// struct TimelineCurveVisitor;
//
// impl Visitor for TimelineCurveVisitor {
//     type Value = TimelineCurve;
//
//     fn visit_str<E: Error>(&mut self, value: &str) -> Result<TimelineCurve, E> {
//         Ok(TimelineCurve::CurvePredefined(value.to_string()))
//     }
//
//     fn visit_seq<V: SeqVisitor>(&mut self, mut visitor: V) -> Result<TimelineCurve, V::Error> {
//         visitor.visit().map(|v| TimelineCurve::CurveBezier(v))
//     }
// }
//
// impl Deserialize for TimelineCurve {
//     fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
//         deserializer.visit(TimelineCurveVisitor)
//     }
// }


//
// impl from_json::FromJson for TimelineCurve {
//     fn from_json(input: &from_json::Json) -> Result<TimelineCurve, from_json::FromJsonError> {
//         use from_json::FromJson;
//
//         if input.is_array() {
//             Ok(TimelineCurve::CurveBezier(try!(FromJson::from_json(input))))
//         } else {
//             Ok(TimelineCurve::CurvePredefined(try!(FromJson::from_json(input))))
//         }
//     }
// }

#[derive(Deserialize, Debug, Clone)]
pub struct SlotTimeline {
    pub attachment: Option<Vec<SlotAttachmentTimeline>>,
    pub color: Option<Vec<SlotColorTimeline>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SlotAttachmentTimeline {
    pub time: f64,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SlotColorTimeline {
    pub time: f64,
    pub color: Option<String>,
    pub curve: Option<TimelineCurve>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EventKeyframe {
    time: f64,
    name: String,
    #[serde(alias="int")]
    int_: Option<i32>,
    #[serde(alias="float")]
    float_: Option<f64>,
    #[serde(alias="string")]
    string_: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DrawOrderTimeline {
    time: f64,
    offsets: Option<Vec<DrawOrderTimelineOffset>>,
}

#[derive(Debug, Clone)]
pub struct DrawOrderTimelineOffset {
    slot: String,
    offset: i32,
}

#[cfg(test)]
mod test {

    use super::*;
    use serde_json;
    use serde_json::error::Error;

    #[test]
    fn test_slot() {
        let txt = "{ \"name\": \"left shoulder\", \"bone\": \"left shoulder\", \"attachment\": \"left-shoulder\" }";
        let slot: Slot = serde_json::from_str(&txt).unwrap();
        assert!(slot.name == "left shoulder" &&
                slot.bone == "left shoulder" &&
                slot.attachment == Some("left-shoulder".to_string()) &&
                slot.color == None);
    }

    #[test]
    fn test_bone() {
        let txt = "{ \"name\": \"left foot\", \"parent\": \"left lower leg\", \"length\": 46.5, \"x\": 64.02, \"y\": -8.67, \"rotation\": 102.43 }";
        let bone: Bone = serde_json::from_str(&txt).unwrap();
        assert!(bone.name == "left foot" &&
                bone.parent.unwrap() == "left lower leg" &&
                bone.length == Some(46.5));
    }

    #[test]
    fn test_translation() {
        let txt = "{ \"time\": 0, \"x\": -3, \"y\": -2.25 }";
        let trans: BoneTranslateTimeline = serde_json::from_str(&txt).unwrap();
        assert!(trans.time == 0.0 &&
                trans.x == Some(-3.0) &&
                trans.y == Some(-2.25) &&
                trans.curve == None);
    }

    #[test]
    fn test_rotation() {
        let mut txt = "{ \"time\": 0.1333, \"angle\": -8.78 }";
        let mut rot: BoneRotateTimeline = serde_json::from_str(&txt).unwrap();
        assert!(rot.time == 0.1333 &&
                rot.angle == Some(-8.78) &&
                rot.curve == None);
    }

    #[test]
    fn test_timeline() {
        let txt = "{
            \"rotate\": [
                { \"time\": 0, \"angle\": -26.55 },
                { \"time\": 0.1333, \"angle\": -8.78 },
                { \"time\": 0.2666, \"angle\": 9.51 },
                { \"time\": 0.4, \"angle\": 30.74 }
            ],
            \"translate\": [
                { \"time\": 0, \"x\": -3, \"y\": -2.25 },
                { \"time\": 0.4, \"x\": -2.18, \"y\": -2.25 },
                { \"time\": 1.0666, \"x\": -3, \"y\": -2.25 }
            ]
        }";
        let timeline: BoneTimeline = serde_json::from_str(&txt).unwrap();

        assert!(timeline.rotate.unwrap().len() == 4 &&
                timeline.translate.unwrap().len() == 3);
    }

}
