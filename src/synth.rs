use crate::lorenz::Lorenz;
use crate::shared_params::{Params, PointWriter};

const LORENZ_SCALE: f64 = 1.0 / 25.0;

fn to_audio_f32(sample: f64) -> f32 {
    // sample as f32
    sample.clamp(-1.0, 1.0) as f32
}

pub struct Synth {
    lorenz: Lorenz,
}

impl Synth {
    pub fn new(_sample_rate: f64) -> Self {
        Synth {
            lorenz: Lorenz::new(),
        }
    }

    /// Read the latest parameter values. Called once per audio buffer, so each
    /// name is looked up once per buffer rather than once per sample.
    pub fn apply_params(&mut self, params: &Params) {
        if let Some(dt) = params.get("dt") {
            self.lorenz.set_dt(dt);
        }
    }

    pub fn process(&mut self, writer: &mut PointWriter) -> f32 {
        self.lorenz.step();
        writer.push(
            self.lorenz.x() as f32,
            self.lorenz.y() as f32,
            self.lorenz.z() as f32,
        );

        let signal = self.lorenz.x();
        to_audio_f32(signal * LORENZ_SCALE)
    }
}
