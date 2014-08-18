extern crate serialize;

use std::collections::HashMap;
use serialize::{Decoder, Decodable};

#[deriving(Decodable, Show, Clone)]
pub struct Document {
    pub bones: Option<Vec<Bone>>,
    pub slots: Option<Vec<Slot>>,
    pub skins: Option<HashMap<String, SkinSlotsList>>,
    pub animations: Option<HashMap<String, Animation>>,
}

#[deriving(Decodable, Show, Clone)]
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

#[deriving(Decodable, Show, Clone)]
pub struct Slot {
    pub name: String,
    pub bone: String,
    pub color: Option<String>,
    pub attachment: Option<String>,
}

#[deriving(Show, Clone)]
pub struct SkinSlotsList(pub HashMap<String, HashMap<String, Attachment>>);

impl<D: Decoder<E>, E> Decodable<D, E> for SkinSlotsList {
    fn decode(d: &mut D) -> Result<SkinSlotsList, E> {
        Decodable::decode(d).map(|v| SkinSlotsList(v))
    }
}

#[deriving(Show, Clone)]
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

#[automatically_derived]
impl <__D: ::serialize::Decoder<__E>, __E>
     ::serialize::Decodable<__D, __E> for Attachment {
    fn decode(__arg_0: &mut __D) ->
     ::std::result::Result<Attachment, __E> {
        __arg_0.read_struct("Attachment", 11u,
                            ref |_d|
                                ::std::result::Ok(Attachment{name:
                                                                 match _d.read_struct_field("name",
                                                                                            0u,
                                                                                            ref
                                                                                                |_d|
                                                                                                ::serialize::Decodable::decode(_d))
                                                                     {
                                                                     Ok(__try_var)
                                                                     =>
                                                                     __try_var,
                                                                     Err(__try_var)
                                                                     =>
                                                                     return Err(__try_var),
                                                                 },
                                                             type_:
                                                                 match _d.read_struct_field("type",
                                                                                            1u,
                                                                                            ref
                                                                                                |_d|
                                                                                                ::serialize::Decodable::decode(_d))
                                                                     {
                                                                     Ok(__try_var)
                                                                     =>
                                                                     __try_var,
                                                                     Err(__try_var)
                                                                     =>
                                                                     return Err(__try_var),
                                                                 },
                                                             x:
                                                                 match _d.read_struct_field("x",
                                                                                            2u,
                                                                                            ref
                                                                                                |_d|
                                                                                                ::serialize::Decodable::decode(_d))
                                                                     {
                                                                     Ok(__try_var)
                                                                     =>
                                                                     __try_var,
                                                                     Err(__try_var)
                                                                     =>
                                                                     return Err(__try_var),
                                                                 },
                                                             y:
                                                                 match _d.read_struct_field("y",
                                                                                            3u,
                                                                                            ref
                                                                                                |_d|
                                                                                                ::serialize::Decodable::decode(_d))
                                                                     {
                                                                     Ok(__try_var)
                                                                     =>
                                                                     __try_var,
                                                                     Err(__try_var)
                                                                     =>
                                                                     return Err(__try_var),
                                                                 },
                                                             scaleX:
                                                                 match _d.read_struct_field("scaleX",
                                                                                            4u,
                                                                                            ref
                                                                                                |_d|
                                                                                                ::serialize::Decodable::decode(_d))
                                                                     {
                                                                     Ok(__try_var)
                                                                     =>
                                                                     __try_var,
                                                                     Err(__try_var)
                                                                     =>
                                                                     return Err(__try_var),
                                                                 },
                                                             scaleY:
                                                                 match _d.read_struct_field("scaleY",
                                                                                            5u,
                                                                                            ref
                                                                                                |_d|
                                                                                                ::serialize::Decodable::decode(_d))
                                                                     {
                                                                     Ok(__try_var)
                                                                     =>
                                                                     __try_var,
                                                                     Err(__try_var)
                                                                     =>
                                                                     return Err(__try_var),
                                                                 },
                                                             rotation:
                                                                 match _d.read_struct_field("rotation",
                                                                                            6u,
                                                                                            ref
                                                                                                |_d|
                                                                                                ::serialize::Decodable::decode(_d))
                                                                     {
                                                                     Ok(__try_var)
                                                                     =>
                                                                     __try_var,
                                                                     Err(__try_var)
                                                                     =>
                                                                     return Err(__try_var),
                                                                 },
                                                             width:
                                                                 match _d.read_struct_field("width",
                                                                                            7u,
                                                                                            ref
                                                                                                |_d|
                                                                                                ::serialize::Decodable::decode(_d))
                                                                     {
                                                                     Ok(__try_var)
                                                                     =>
                                                                     __try_var,
                                                                     Err(__try_var)
                                                                     =>
                                                                     return Err(__try_var),
                                                                 },
                                                             height:
                                                                 match _d.read_struct_field("height",
                                                                                            8u,
                                                                                            ref
                                                                                                |_d|
                                                                                                ::serialize::Decodable::decode(_d))
                                                                     {
                                                                     Ok(__try_var)
                                                                     =>
                                                                     __try_var,
                                                                     Err(__try_var)
                                                                     =>
                                                                     return Err(__try_var),
                                                                 },
                                                             fps:
                                                                 match _d.read_struct_field("fps",
                                                                                            9u,
                                                                                            ref
                                                                                                |_d|
                                                                                                ::serialize::Decodable::decode(_d))
                                                                     {
                                                                     Ok(__try_var)
                                                                     =>
                                                                     __try_var,
                                                                     Err(__try_var)
                                                                     =>
                                                                     return Err(__try_var),
                                                                 },
                                                             mode:
                                                                 match _d.read_struct_field("mode",
                                                                                            10u,
                                                                                            ref
                                                                                                |_d|
                                                                                                ::serialize::Decodable::decode(_d))
                                                                     {
                                                                     Ok(__try_var)
                                                                     =>
                                                                     __try_var,
                                                                     Err(__try_var)
                                                                     =>
                                                                     return Err(__try_var),
                                                                 },}))
    }
}

#[deriving(Decodable, Show, Clone)]
#[allow(non_camel_case_types)]
pub enum AttachmentType {
    region,
    regionsequence,
    boundingbox,
}

#[deriving(Show, Clone)]
pub struct Event {
    pub name: String,
    pub int_: Option<int>,
    pub float_: Option<f64>,
    pub string: Option<String>,
}

#[automatically_derived]
impl <__D: ::serialize::Decoder<__E>, __E>
     ::serialize::Decodable<__D, __E> for Event {
    fn decode(__arg_0: &mut __D) -> ::std::result::Result<Event, __E> {
        __arg_0.read_struct("Event", 4u,
                            ref |_d|
                                ::std::result::Ok(Event{name:
                                                            match _d.read_struct_field("name",
                                                                                       0u,
                                                                                       ref
                                                                                           |_d|
                                                                                           ::serialize::Decodable::decode(_d))
                                                                {
                                                                Ok(__try_var)
                                                                =>
                                                                __try_var,
                                                                Err(__try_var)
                                                                =>
                                                                return Err(__try_var),
                                                            },
                                                        int_:
                                                            match _d.read_struct_field("int",
                                                                                       1u,
                                                                                       ref
                                                                                           |_d|
                                                                                           ::serialize::Decodable::decode(_d))
                                                                {
                                                                Ok(__try_var)
                                                                =>
                                                                __try_var,
                                                                Err(__try_var)
                                                                =>
                                                                return Err(__try_var),
                                                            },
                                                        float_:
                                                            match _d.read_struct_field("float",
                                                                                       2u,
                                                                                       ref
                                                                                           |_d|
                                                                                           ::serialize::Decodable::decode(_d))
                                                                {
                                                                Ok(__try_var)
                                                                =>
                                                                __try_var,
                                                                Err(__try_var)
                                                                =>
                                                                return Err(__try_var),
                                                            },
                                                        string:
                                                            match _d.read_struct_field("string",
                                                                                       3u,
                                                                                       ref
                                                                                           |_d|
                                                                                           ::serialize::Decodable::decode(_d))
                                                                {
                                                                Ok(__try_var)
                                                                =>
                                                                __try_var,
                                                                Err(__try_var)
                                                                =>
                                                                return Err(__try_var),
                                                            },}))
    }
}

#[deriving(Decodable, Show, Clone)]
pub struct Animation {
    pub bones: HashMap<String, BoneTimeline>,
    pub slots: HashMap<String, SlotTimeline>,
    pub events: Option<Vec<EventKeyframe>>,
    pub draworder: Option<Vec<DrawOrderTimeline>>,
}

#[deriving(Decodable, Show, Clone)]
pub struct BoneTimeline {
    pub translate: Option<BoneTranslateTimeline>,
    pub rotate: Option<BoneRotateTimeline>,
    pub scale: Option<BoneScaleTimeline>,
}

#[deriving(Decodable, Show, Clone)]
pub struct BoneTranslateTimeline {
    pub time: f64,
    pub curve: Option<Vec<f64>>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub angle: Option<f64>,
}

#[deriving(Decodable, Show, Clone)]
pub struct BoneRotateTimeline {
    pub time: f64,
    pub curve: Option<Vec<f64>>,
    pub angle: Option<f64>,
}

#[deriving(Decodable, Show, Clone)]
pub struct BoneScaleTimeline {
    pub time: f64,
    pub curve: Option<Vec<f64>>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[deriving(Decodable, Show, Clone)]
pub struct SlotTimeline {
    pub attachment: Option<SlotAttachmentTimeline>,
    pub color: Option<SlotColorTimeline>,
}

#[deriving(Decodable, Show, Clone)]
pub struct SlotAttachmentTimeline {
    pub time: f64,
    pub name: Option<String>,
}

#[deriving(Decodable, Show, Clone)]
pub struct SlotColorTimeline {
    pub time: f64,
    pub color: Option<String>,
    pub curve: Option<Vec<f64>>,
}

#[deriving(Show, Clone)]
pub struct EventKeyframe {
    time: f64,
    name: String,
    int_: Option<int>,
    float_: Option<f64>,
    string_: Option<String>,
}

#[automatically_derived]
impl <__D: ::serialize::Decoder<__E>, __E>
     ::serialize::Decodable<__D, __E> for EventKeyframe {
    fn decode(__arg_0: &mut __D) ->
     ::std::result::Result<EventKeyframe, __E> {
        __arg_0.read_struct("EventKeyframe", 5u,
                            ref |_d|
                                ::std::result::Ok(EventKeyframe{time:
                                                                    match _d.read_struct_field("time",
                                                                                               0u,
                                                                                               ref
                                                                                                   |_d|
                                                                                                   ::serialize::Decodable::decode(_d))
                                                                        {
                                                                        Ok(__try_var)
                                                                        =>
                                                                        __try_var,
                                                                        Err(__try_var)
                                                                        =>
                                                                        return Err(__try_var),
                                                                    },
                                                                name:
                                                                    match _d.read_struct_field("name",
                                                                                               1u,
                                                                                               ref
                                                                                                   |_d|
                                                                                                   ::serialize::Decodable::decode(_d))
                                                                        {
                                                                        Ok(__try_var)
                                                                        =>
                                                                        __try_var,
                                                                        Err(__try_var)
                                                                        =>
                                                                        return Err(__try_var),
                                                                    },
                                                                int_:
                                                                    match _d.read_struct_field("int",
                                                                                               2u,
                                                                                               ref
                                                                                                   |_d|
                                                                                                   ::serialize::Decodable::decode(_d))
                                                                        {
                                                                        Ok(__try_var)
                                                                        =>
                                                                        __try_var,
                                                                        Err(__try_var)
                                                                        =>
                                                                        return Err(__try_var),
                                                                    },
                                                                float_:
                                                                    match _d.read_struct_field("float",
                                                                                               3u,
                                                                                               ref
                                                                                                   |_d|
                                                                                                   ::serialize::Decodable::decode(_d))
                                                                        {
                                                                        Ok(__try_var)
                                                                        =>
                                                                        __try_var,
                                                                        Err(__try_var)
                                                                        =>
                                                                        return Err(__try_var),
                                                                    },
                                                                string_:
                                                                    match _d.read_struct_field("string",
                                                                                               4u,
                                                                                               ref
                                                                                                   |_d|
                                                                                                   ::serialize::Decodable::decode(_d))
                                                                        {
                                                                        Ok(__try_var)
                                                                        =>
                                                                        __try_var,
                                                                        Err(__try_var)
                                                                        =>
                                                                        return Err(__try_var),
                                                                    },}))
    }
}

#[deriving(Decodable, Show, Clone)]
pub struct DrawOrderTimeline {
    time: f64,
    offsets: Option<Vec<DrawOrderTimelineOffset>>,
}

#[deriving(Decodable, Show, Clone)]
pub struct DrawOrderTimelineOffset {
    slot: String,
    offset: int,
}
