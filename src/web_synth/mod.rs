use wasm_bindgen::prelude::*;

pub mod oscillators;
pub mod envelopes;
pub mod instruments;
pub mod keyboard;
pub mod sequencer;

type FLOAT = f64;

pub(crate) const SAMPLE_RATE: f64 = 48000.0;
pub(crate) const SAMPLE_SIZE: usize = 128;

pub trait Source {
    fn get_sample_block(&self, t: f64) -> [f64; 128];
}

pub trait MutSource {
    fn get_sample_block(&mut self, t: f64) -> [f64; 128];
}

pub trait Effect {
    fn process_sample_block(&self, t: f64, block: [f64; 128]) -> [f64; 128];
}

pub struct Note {
    id: u32,
    on: f64,
    off: f64,
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

    pub fn new_with_params(id: u32, t_on: f64) -> Note {
        Note {
            id,
            on: t_on,
            off: 0.0,
            active: true,
            instrument: 0
        }
    }
}

pub fn scale(note_id: u32) -> f64 {
    8.0 * 1.0594630943592952645618252949463_f64.powi(note_id as i32)
}

pub fn piano_scale(note_id: u32) -> f64 {
    440.0 * 1.0594630943592952645618252949463_f64.powi(note_id as i32 - 49)
}