use crate::web_synth::{Source, Note, SAMPLE_SIZE, MutSource, SAMPLE_RATE};
use crate::web_synth::instruments::{Instrument, BELL, Bell};

fn calc_offset_time(t: f64, sample_idx: usize) -> f64 {
    t + sample_idx as f64 / SAMPLE_RATE
}

pub struct Keyboard {
    instrument: Box<dyn Instrument>,
    notes: Vec<Note>,
    master_volume: f64,

    keys_pressed: [bool; 16],

    note_ids: Vec<u32>
}

impl MutSource for Keyboard {
    fn get_sample_block(&mut self, t: f64) -> [f64; 128] {
        let mut output: [f64; 128] = [0.0; 128];
        for i in 0..SAMPLE_SIZE {
            for n in self.notes.iter_mut() {
                let mut note_finished = false;
                output[i] += self.instrument.sound(calc_offset_time(t, i), n, &mut note_finished);

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

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            instrument: Box::new(Bell::new()),
            notes: Vec::new(),
            master_volume: 0.2,

            keys_pressed: [false; 16],

            note_ids: Vec::new()
        }
    }

    pub fn get_keys_ptr(&self) -> *const bool {
        self.keys_pressed.as_ptr()
    }

    pub fn get_note_ids_ptr(&self) -> *const u32 {
        self.note_ids.as_ptr()
    }

    pub fn get_note_ids_size(&self) -> usize {
        self.note_ids.len()
    }

    pub fn update_notes(&mut self, t: f64) {
        for (i, pressed) in self.keys_pressed.iter().enumerate() {
            let opt_note = self.notes.iter_mut().find(|n| n.id == (i as u32 + 64));
            match opt_note {
                Some(note_found) => {
                    match pressed {
                        true => {
                            if note_found.off > note_found.on {
                                note_found.on = t;
                                note_found.active = true;
                            }
                        }
                        false => {
                            if note_found.off < note_found.on {
                                note_found.off = t;
                            }
                        }
                    }
                }
                None => {
                    if *pressed {
                        let mut new_note = Note::new();
                        new_note.id = i as u32 + 64;
                        new_note.on = t;
                        new_note.active = true;
                        self.notes.push(new_note);
                        self.note_ids.push(i as u32 + 64);
                    }
                }
            }
        }
    }

    fn clear_finished_notes(&mut self) {
        self.notes.retain(|n| n.active);
        self.note_ids.clear();
        for n in self.notes.iter() {
            self.note_ids.push(n.id);
        }
    }
}