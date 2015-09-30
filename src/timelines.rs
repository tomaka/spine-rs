use json;

/// Timeline trait to define struct with time property
pub trait Timeline {
    /// return time value
    fn time(&self) -> f32;
}

macro_rules! impl_timeline {
    ($struct_name:ty) => {
        impl Timeline for $struct_name {
            fn time(&self) -> f32 {
                self.time
            }
        }
    }
}

impl_timeline!(json::BoneTranslateTimeline);
impl_timeline!(json::BoneRotateTimeline);
impl_timeline!(json::BoneScaleTimeline);
impl_timeline!(json::SlotAttachmentTimeline);
impl_timeline!(json::SlotColorTimeline);
impl_timeline!(json::EventKeyframe);
impl_timeline!(json::DrawOrderTimeline);



// pub fn interval<T: Timeline>(timelines: &[T]) -> Option<(T, T)> {
//     match timeline.iter().zip(timeline.iter().skip(1))
//         .find(|&(before, after)| elapsed >= before.time && elapsed < after.time)
// }
