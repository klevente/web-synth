use crate::web_synth::{Source, Note, SAMPLE_SIZE, MutSource, SAMPLE_RATE, scale, piano_scale};
use crate::web_synth::instruments::{Instrument, BELL, Bell};

use web_sys::console;
use std::f64::consts::PI;

fn calc_offset_time(t: f64, sample_idx: usize) -> f64 {
    t + sample_idx as f64 / SAMPLE_RATE
}

fn w(freq_hz: f64) -> f64 {
    2.0 * PI * freq_hz
}

pub struct Keyboard {
    instrument: Box<dyn Instrument>,
    notes: Vec<Note>,
    master_volume: f64,

    keys_pressed: [bool; 17],
    octave_offset: u32
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
            notes: Vec::with_capacity(17),
            master_volume: 0.2,

            keys_pressed: [false; 17],
            octave_offset: 0
        }
    }

    pub fn get_keys_ptr(&self) -> *const bool {
        self.keys_pressed.as_ptr()
    }

    pub fn set_octave(&mut self, new_octave: u32) {
        self.octave_offset = new_octave * 12 + 4;
    }

    pub fn update_notes(&mut self, t: f64) {
        for (i, pressed) in self.keys_pressed.iter().enumerate() {
            let offset = self.octave_offset;
            let opt_note = self.notes.iter_mut().find(|n| n.id == (i as u32 + offset));
            match opt_note {
                Some(note_found) => {
                    match *pressed {
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
                        /*let mut new_note = Note::new();
                        new_note.id = i as u32 + offset;
                        new_note.on = t;
                        new_note.active = true;*/
                        let new_note = Note::new_with_params(i as u32 + offset, t);
                        console::log_1(&piano_scale(new_note.id).into());
                        self.notes.push(new_note);
                    }
                }
            }
        }
    }

    fn clear_finished_notes(&mut self) {
        self.notes.retain(|n| n.active);
    }
}