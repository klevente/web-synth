use crate::web_synth::{Source, Note, SAMPLE_SIZE, MutSource};
use crate::web_synth::instruments::{Instrument, BELL};

pub struct Keyboard<'a> {
    instrument: &'a dyn Instrument,
    notes: Vec<Note>,
    master_volume: f32
}

impl MutSource for Keyboard<'_> {
    fn get_sample_block(&mut self, t: f32) -> [f32; 128] {
        let mut output: [f32; 128] = [0.0; 128];
        for i in 0..SAMPLE_SIZE {
            for n in self.notes.iter_mut() {
                let mut note_finished = false;
                output[i] += self.instrument.sound(t, n, &mut note_finished);

                if note_finished {
                    n.active = false;
                }
            }
            output[i] *= self.master_volume;
        }

        self.clear_finished_notes();

        output
    }
}

impl Keyboard<'_> {
    pub fn new() -> Keyboard<'static> {
        Keyboard {
            instrument: &BELL,
            notes: Vec::new(),
            master_volume: 0.2
        }
    }

    pub fn update_notes(&mut self) {
        
    }

    fn clear_finished_notes(&mut self) {
        self.notes.retain(|n| n.active);
    }
}