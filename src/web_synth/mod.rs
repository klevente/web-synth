pub mod oscillators;
pub mod envelopes;
pub mod instruments;
pub mod keyboard;

type FLOAT = f32;

pub(crate) const SAMPLE_RATE: f32 = 44100.0;
pub(crate) const SAMPLE_SIZE: usize = 128;

pub trait Source {
    fn get_sample_block(&self, t: f32) -> [f32; 128];
}

pub trait MutSource {
    fn get_sample_block(&mut self, t: f32) -> [f32; 128];
}

pub trait Effect {
    fn process_sample_block(&self, t: f32, block: [f32; 128]) -> [f32; 128];
}

pub struct Note {
    id: u32,
    on: f32,
    off: f32,
    active: bool,
    instrument: u32
}

impl Note {
    pub fn new() -> Note {
        Note {
            id: 0,
            on: 0.0,
            off: 0.0,
            active: false,
            instrument: 0
        }
    }
}

pub fn scale(note_id: u32) -> f32 {
    // 8.0 * 1.0594630943592952645618252949463.powf(note_id as f32)
    let mut pow: f32 = 1.0;
    for _i in 0..note_id {
        pow *= 1.0594630943592952645618252949463;
    }

    8.0 * pow
}