use json;
use serialize::hex::FromHex;

const BEZIER_SEGMENTS: usize = 10;

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

impl_timeline!(json::SlotAttachmentTimeline);
impl_timeline!(json::EventKeyframe);
impl_timeline!(json::DrawOrderTimeline);

pub struct BoneTimeline {
    pub translate: Option<Vec<BoneTranslateTimeline>>,
    pub rotate: Option<Vec<BoneRotateTimeline>>,
    pub scale: Option<Vec<BoneScaleTimeline>>,
}

impl BoneTimeline {
    pub fn from_json(json: json::BoneTimeline) -> BoneTimeline {
        BoneTimeline {
            translate: json.translate.map(|t| t.into_iter()
                .map(|t| BoneTranslateTimeline::from_json(t)).collect()),
            scale: json.scale.map(|t| t.into_iter()
                .map(|t| BoneScaleTimeline::from_json(t)).collect()),
            rotate: json.rotate.map(|t| t.into_iter()
                .map(|t| BoneRotateTimeline::from_json(t)).collect()),
        }
    }
}

/// Timeline with curve
pub trait CurveTimeline {

    /// return time value
    fn time(&self) -> f32;

    /// curve definition
    fn curve(&self) -> &json::TimelineCurve;

    /// all interpolations
    fn interpolations(&self) -> Option<(&[f32], &[f32])>;

    /// interpolation values (x, y)
    /// Sets the control handle positions for an interpolation bezier curve used to transition
    /// from this keyframe to the next.
	/// cx1 and cx2 are from 0 to 1, representing the percent of time between the two keyframes.
    /// cy1 and cy2 are the percent of the difference between the keyframe's values.
    fn interpolate(&self) -> Option<(Vec<f32>, Vec<f32>)> {

        let (cx1, cy1, cx2, cy2) = match *self.curve() {
            json::TimelineCurve::CurveStepped |
            json::TimelineCurve::CurveLinear  => return None,
            json::TimelineCurve::CurveBezier(ref p)  => (p[0], p[1], p[2], p[3])
        };

        let subdiv1 = 1f32 / BEZIER_SEGMENTS as f32;
        let subdiv2 = subdiv1 * subdiv1;
        let subdiv3 = subdiv2 * subdiv1;
        let (pre1, pre2, pre4, pre5) = (3f32 * subdiv1, 3f32 * subdiv2, 6f32 * subdiv2, 6f32 * subdiv3);
        let (tmp1x, tmp1y) = (-cx1 * 2f32 + cx2, -cy1 * 2f32 + cy2);
        let (tmp2x, tmp2y) = ((cx1 - cx2) * 3f32 + 1f32, (cy1 - cy2) * 3f32 + 1f32);
        let mut dfx = cx1 * pre1 + tmp1x * pre2 + tmp2x * subdiv3;
        let mut dfy = cy1 * pre1 + tmp1y * pre2 + tmp2y * subdiv3;
        let (mut ddfx, mut ddfy) = (tmp1x * pre4 + tmp2x * pre5, tmp1y * pre4 + tmp2y * pre5);
        let (dddfx, dddfy) = (tmp2x * pre5, tmp2y * pre5);

        let (mut vec_x, mut vec_y) = (Vec::with_capacity(BEZIER_SEGMENTS), Vec::with_capacity(BEZIER_SEGMENTS));
        let (mut x, mut y) = (dfx, dfy);
        for _ in 0..BEZIER_SEGMENTS {
            vec_x.push(x);
            vec_y.push(y);
            dfx += ddfx;
			dfy += ddfy;
			ddfx += dddfx;
			ddfy += dddfy;
			x += dfx;
			y += dfy;
        }
        Some((vec_x, vec_y))
    }

    /// Get percent conversion depending on curve type
    fn get_percent(&self, percent: f32) -> f32 {

        let (x, y) = match *self.curve() {
            json::TimelineCurve::CurveStepped => return 0f32,
            json::TimelineCurve::CurveLinear  => return percent,
            json::TimelineCurve::CurveBezier(..)  => self.interpolations().unwrap()
        };

        // bezier curve
        match x.iter().position(|&xi| percent >= xi) {
            Some(0) => y[0] * percent / x[0],
            Some(i) => y[i - 1] + (y[i] - y[i - 1]) * (percent - x[i - 1]) / (x[i] - x[i - 1]),
            None => y[x.len()] + (1f32 - y[x.len()] * (percent - x[x.len()]) / (1f32 - x[x.len()]))
        }
    }
}

macro_rules! impl_curve_timeline {
    ($struct_name:ty) => {
        impl CurveTimeline for $struct_name {
            fn time(&self) -> f32 {
                self.time
            }
            fn curve(&self) -> &json::TimelineCurve {
                &self.curve
            }
            fn interpolations(&self) -> Option<(&[f32], &[f32])> {
                self.interpolations.as_ref().map(|&(ref x, ref y)| (x as &[f32], y as &[f32]))
            }
        }
    }
}

pub struct BoneTranslateTimeline {
    time: f32,
    curve: json::TimelineCurve,
    x: f32,
    y: f32,
    // bezier curve interpolations
    interpolations: Option<(Vec<f32>, Vec<f32>)>
}

impl_curve_timeline!(BoneTranslateTimeline);

impl BoneTranslateTimeline {
    fn from_json(jtimeline: json::BoneTranslateTimeline) -> BoneTranslateTimeline {
        let mut timeline = BoneTranslateTimeline {
            time: jtimeline.time,
            curve: jtimeline.curve.unwrap_or(json::TimelineCurve::CurveLinear),
            x: jtimeline.x.unwrap_or(0f32),
            y: jtimeline.y.unwrap_or(0f32),
            interpolations: None
        };
        timeline.interpolations = timeline.interpolate();
        timeline
    }
}

pub struct BoneRotateTimeline {
    time: f32,
    curve: json::TimelineCurve,
    angle: f32,
    interpolations: Option<(Vec<f32>, Vec<f32>)>
}

impl_curve_timeline!(BoneRotateTimeline);

impl BoneRotateTimeline {
    fn from_json(jtimeline: json::BoneRotateTimeline) -> BoneRotateTimeline {
        let mut timeline = BoneRotateTimeline {
            time: jtimeline.time,
            curve: jtimeline.curve.unwrap_or(json::TimelineCurve::CurveLinear),
            angle: jtimeline.angle.unwrap_or(0f32),
            interpolations: None
        };
        timeline.interpolations = timeline.interpolate();
        timeline
    }
}

pub struct BoneScaleTimeline {
    time: f32,
    curve: json::TimelineCurve,
    x: f32,
    y: f32,
    interpolations: Option<(Vec<f32>, Vec<f32>)>
}

impl_curve_timeline!(BoneScaleTimeline);

impl BoneScaleTimeline {
    fn from_json(jtimeline: json::BoneScaleTimeline) -> BoneScaleTimeline {
        let mut timeline = BoneScaleTimeline {
            time: jtimeline.time,
            curve: jtimeline.curve.unwrap_or(json::TimelineCurve::CurveLinear),
            x: jtimeline.x.unwrap_or(1f32),
            y: jtimeline.y.unwrap_or(1f32),
            interpolations: None
        };
        timeline.interpolations = timeline.interpolate();
        timeline
    }
}

pub struct SlotColorTimeline {
    time: f32,
    color: Vec<u8>,
    curve: json::TimelineCurve,
    interpolations: Option<(Vec<f32>, Vec<f32>)>
}

impl_curve_timeline!(SlotColorTimeline);

impl SlotColorTimeline {
    fn from_json(jtimeline: json::SlotColorTimeline) -> SlotColorTimeline {
        let color = jtimeline.color.unwrap_or("FFFFFFFF".into());
        let mut timeline = SlotColorTimeline {
            time: jtimeline.time,
            curve: jtimeline.curve.unwrap_or(json::TimelineCurve::CurveLinear),
            color: color.from_hex().unwrap(),
            interpolations: None
        };
        timeline.interpolations = timeline.interpolate();
        timeline
    }
}

pub struct SlotTimeline {
    pub attachment: Option<Vec<json::SlotAttachmentTimeline>>,
    pub color: Option<Vec<SlotColorTimeline>>,
}

impl SlotTimeline {
    pub fn from_json(json: json::SlotTimeline) -> SlotTimeline {
        SlotTimeline {
            attachment: json.attachment,
            color: json.color.map(|c| c.into_iter()
                .map(|c| SlotColorTimeline::from_json(c)).collect())
        }
    }
}
