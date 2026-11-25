use crate::delay::Delay;
use crate::lorenz::Lorenz;

const LORENZ_SCALE: f64 = 1.0 / 25.0;

pub struct Channel {
    delay: Delay,
    lorenz: Lorenz,
}

impl Channel {
    pub fn new(sample_rate: f64) -> Channel {
        Channel {
            delay: Delay::new(sample_rate, 150.0),
            lorenz: Lorenz::new(),
        }
    }

    pub fn process(&mut self) -> f64 {
        self.lorenz.step();
        self.delay.process(self.lorenz.x()) * LORENZ_SCALE
    }

    pub fn set_dt(&mut self, dt: f64) {
        self.lorenz.set_dt(dt);
    }
}