pub mod oscillators;
pub mod envelopes;
pub mod synths;

const SAMPLE_RATE: f32 = 44100.0;
const SAMPLE_SIZE: usize = 128;

pub trait Source {
    fn get_sample_block(&self, t: f32) -> [f32; 128];
}

pub trait Effect {
    fn process_sample_block(&self, t: f32, block: [f32; 128]) -> [f32; 128];
}