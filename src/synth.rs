use crate::channel::Channel;
use crate::shared_params::{Params, PointWriter};

fn to_audio_f32(sample: f64) -> f32 {
    // sample as f32
    sample.clamp(-1.0, 1.0) as f32
}

pub struct Synth {
    channel: Channel,
}

impl Synth {
    pub fn new(sample_rate: f64) -> Self {
        Synth {
            channel: Channel::new(sample_rate)
        }
    }

    pub fn apply_params(&mut self, params: &Params) {
        if let Some(dt) = params.get("dt") {
            self.channel.set_dt(dt);
        }
    }

    pub fn process(&mut self, writer: &mut PointWriter) -> f32 {
        to_audio_f32(self.channel.process())
    }
}
