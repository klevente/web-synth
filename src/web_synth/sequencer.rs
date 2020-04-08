use crate::web_synth::{Source, Note};

struct Sequencer {
    beats: u32,
    subbeats: u32,
    tempo: f64,
    beat_time: f64,
    pattern: String,
    notes: Vec<Note>
}

impl Source for Sequencer {
    fn get_sample_block(&self, t: f64) -> [f64; 128] {
        unimplemented!()
    }
}

impl Sequencer {
    pub fn new() -> Sequencer {
        Sequencer {
            beats: 4,
            subbeats: 4,
            tempo: 90.0,
            beat_time: (60.0 / 90.0) / (4 as f64),
            pattern: "x...x...x...x...".into_string(),
            notes: Vec::new()
        }
    }

    fn schedule(&self, t: f64) {

    }
}