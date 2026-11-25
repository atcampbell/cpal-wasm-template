const MAX_DELAY_MS: f64 = 5000.0;

pub struct Delay {
    buffer: Vec<f64>,
    ms: f64,
    feedback: f64,
    sample_rate: f64,
    write_pos: usize,
    read_pos: usize,
}

fn ms_to_samps(sample_rate: f64, ms: f64) -> f64 {
    ms / (1000.0 / sample_rate)
}

impl Delay {
    pub fn new(sample_rate: f64, ms: f64) -> Delay {
        let buffer_length = ms_to_samps(sample_rate, ms);

        let mut buffer = vec![0.0; buffer_length as usize];

        web_sys::console::error_1(&format!("Delay buffer len samps: {}", buffer.len()).into());

        Delay {
            feedback: 0.8,
            read_pos: 0,
            write_pos: 0,
            buffer,
            ms,
            sample_rate,
        }
    }

    pub fn process(&mut self, input: f64) -> f64 {
        let curr = self.buffer[self.read_pos];
        let err = input + curr * self.feedback;
        self.buffer[self.read_pos] = err;

        self.read_pos = (self.read_pos + 1) % self.buffer.len();

        err
    }
}