pub mod oscillators;
pub mod envelopes;
pub mod instruments;

pub(crate) const SAMPLE_RATE: f32 = 44100.0;
pub(crate) const SAMPLE_SIZE: usize = 128;

pub trait Source {
    fn get_sample_block(&self, t: f32) -> [f32; 128];
}

pub trait Effect {
    fn process_sample_block(&self, t: f32, block: [f32; 128]) -> [f32; 128];
}