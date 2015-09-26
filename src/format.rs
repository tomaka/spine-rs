#![allow(dead_code)]
#![allow(non_snake_case)]

use std::collections::HashMap;
use serde::de::{Deserialize, Deserializer, Error, Visitor, SeqVisitor};

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
    #[serde(default)]
    pub length: f64,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    pub scaleX: Option<f64>,
    pub scaleY: Option<f64>,
    #[serde(default)]
    pub rotation: f64,
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
    #[serde(default, alias="type")]
    pub type_: AttachmentType,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    pub scaleX: Option<f64>,
    pub scaleY: Option<f64>,
    #[serde(default)]
    pub rotation: f64,
    #[serde(default)]
    pub width: f64,
    #[serde(default)]
    pub height: f64,
    pub fps: Option<f64>,
    pub mode: Option<f64>,
    //vertices: Option<Vec<??>>     // TODO: ?
}

#[derive(Debug, Clone)]
pub enum AttachmentType {
    Region,
    RegionSequence,
    BoundingBox,
}

impl Default for AttachmentType {
    fn default() -> AttachmentType {
        AttachmentType::Region
    }
}

impl Deserialize for AttachmentType {

    #[inline]
    fn deserialize<D>(deserializer: &mut D) -> Result<AttachmentType, D::Error>
        where D: Deserializer
    {
        struct AttachmentTypeVisitor;

        impl Visitor for AttachmentTypeVisitor {
            type Value = AttachmentType;

            fn visit_str<E>(&mut self, value: &str) -> Result<AttachmentType, E> where E: Error {
                match value {
                    "region" => Ok(AttachmentType::Region),
                    "regionsequence" => Ok(AttachmentType::RegionSequence),
                    "boundingbox" => Ok(AttachmentType::BoundingBox),
                    _ => Err(Error::unknown_field(value)),
                }
            }
        }

        deserializer.visit(AttachmentTypeVisitor)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    pub name: String,
    #[serde(alias="int", default)]
    pub int_: i32,
    #[serde(alias="float", default)]
    pub float_: f64,
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
    #[serde(default)]
    pub curve: Curve,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
}

#[derive(Debug, Clone)]
pub enum Curve {
    Linear,
    Stepped,
    Bezier(f64, f64, f64, f64)
}

impl Default for Curve {
    fn default() -> Curve {
        Curve::Linear
    }
}

impl Deserialize for Curve {

    #[inline]
    fn deserialize<D>(deserializer: &mut D) -> Result<Curve, D::Error>
        where D: Deserializer
    {
        struct CurveVisitor;

        impl Visitor for CurveVisitor {
            type Value = Curve;

            fn visit_str<E>(&mut self, value: &str) -> Result<Curve, E> where E: Error {
                match value {
                    "linear" => Ok(Curve::Linear),
                    "stepped" => Ok(Curve::Stepped),
                    _ => Err(Error::unknown_field(value)),
                }
            }

            fn visit_seq<V>(&mut self, mut _visitor: V) -> Result<Curve, V::Error>
                where V: SeqVisitor
            {
                // bezier curve: 4 elements only
                let cx1: Option<f64> = try!(_visitor.visit());
                let cy1: Option<f64> = try!(_visitor.visit());
                let cx2: Option<f64> = try!(_visitor.visit());
                let cy2: Option<f64> = try!(_visitor.visit());
                try!(_visitor.end());

                match (cx1, cy1, cx2, cy2) {
                    (Some(cx1), Some(cy1), Some(cx2), Some(cy2)) =>
                        Ok(Curve::Bezier(cx1, cy1, cx2, cy2)),
                    _ => Err(Error::unknown_field("cannot parse bezier curve")),
                }
            }
        }

        deserializer.visit(CurveVisitor)
    }
}


#[derive(Deserialize, Debug, Clone)]
pub struct BoneRotateTimeline {
    pub time: f64,
    #[serde(default)]
    pub curve: Curve,
    #[serde(default)]
    pub angle: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BoneScaleTimeline {
    pub time: f64,
    #[serde(default)]
    pub curve: Curve,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SlotTimeline {
    pub attachment: Option<Vec<SlotAttachmentTimeline>>,
    pub color: Option<Vec<SlotColorTimeline>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SlotAttachmentTimeline {
    pub time: f64,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SlotColorTimeline {
    pub time: f64,
    pub color: String,
    #[serde(default)]
    pub curve: Curve,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EventKeyframe {
    time: f64,
    name: String,
    #[serde(alias="int", default)]
    int_: i32,
    #[serde(alias="float", default)]
    float_: f64,
    #[serde(alias="string")]
    string_: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DrawOrderTimeline {
    time: f64,
    offsets: Option<Vec<DrawOrderTimelineOffset>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DrawOrderTimelineOffset {
    slot: String,
    offset: i32,
}

#[cfg(test)]
mod test {

    use super::*;
    use serde_json;

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
                bone.length == 46.5);
    }

    #[test]
    fn test_translation() {
        let txt = "{ \"time\": 0, \"x\": -3, \"y\": -2.25 }";
        let trans: BoneTranslateTimeline = serde_json::from_str(&txt).unwrap();
        assert!(trans.time == 0.0 &&
                trans.x == -3.0 &&
                trans.y == -2.25);
    }

    #[test]
    fn test_rotation() {
        let txt = "{ \"time\": 0.1333, \"angle\": -8.78 }";
        let rot: BoneRotateTimeline = serde_json::from_str(&txt).unwrap();
        assert!(rot.time == 0.1333 &&
                rot.angle == -8.78);
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
