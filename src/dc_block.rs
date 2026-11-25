use std::f64::consts::TAU;

pub struct DcBlock {
    x1: f64,
    y1: f64,
    coef: f64,
    sample_rate: f64,
}

fn hz_to_coeff(sample_rate: f64, hz: f64) -> f64 {
    (hz.abs() * -TAU / sample_rate).exp()
}

impl DcBlock {
    pub fn new(sample_rate: f64) -> DcBlock {
        DcBlock {
            x1: 0.0,
            y1: 0.0,
            coef: hz_to_coeff(sample_rate, 3.0),
            sample_rate
        }
    }

    pub fn process(&mut self, x: f64) -> f64 {
        self.y1 = x - self.x1 + self.coef * self.y1;
        self.x1 = x;
        self.y1
    }
}
