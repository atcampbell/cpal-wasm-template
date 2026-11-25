use crate::dc_block::DcBlock;

const MAX_DELAY_MS: f64 = 5000.0;

pub struct Delay {
    buffer: Vec<f64>,
    ms: f64,
    feedback: f64,
    sample_rate: f64,
    write_pos: usize,
    read_pos: usize,
    balance: f64,
    dc_block: DcBlock,
}

fn crossfade(in1: f64, in2: f64, crossfade: f64) -> f64 {
    ((1.0 - crossfade) * in1) + (crossfade * in2)
}

fn ms_to_samps(sample_rate: f64, ms: f64) -> f64 {
    ms / (1000.0 / sample_rate)
}

impl Delay {
    pub fn new(sample_rate: f64, ms: f64, balance: f64) -> Delay {
        let buffer_length = ms_to_samps(sample_rate, ms);

        let mut buffer = vec![0.0; buffer_length as usize];

        web_sys::console::error_1(&format!("Delay buffer len samps: {}", buffer.len()).into());

        Delay {
            dc_block: DcBlock::new(sample_rate),
            feedback: 0.8,
            read_pos: 0,
            write_pos: 0,
            buffer,
            ms,
            sample_rate,
            balance
        }
    }

    pub fn process(&mut self, input: f64) -> f64 {
        let curr =  self.dc_block.process(self.buffer[self.read_pos]) * self.feedback;
        let out = input + curr;

        self.buffer[self.read_pos] = out;

        let out = crossfade(input, out, self.balance);
        self.read_pos = (self.read_pos + 1) % self.buffer.len();

        out
    }
}