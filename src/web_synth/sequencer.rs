use crate::web_synth::{Note, MutSource, SAMPLE_SIZE, calc_offset_time, DELTA_TIME};
use crate::web_synth::instruments::{Instrument, KickDrum};
use web_sys::console;

pub struct Sequencer {
    beats: u32,
    subbeats: u32,
    tempo: f64,
    beat_time: f64,
    pattern: String,
    instrument: Box<dyn Instrument>,
    notes: Vec<Note>,
    elapsed_time: f64,
    all_beats: u32,
    current_beat: u32,
    master_volume: f64
}

impl MutSource for Sequencer {
    fn get_sample_block(&mut self, t: f64) -> [f64; 128] {
        let mut output: [f64; 128] = [0.0; 128];
        for i in 0..SAMPLE_SIZE {
            self.elapsed_time += DELTA_TIME;
            if self.elapsed_time >= self.beat_time {
                self.elapsed_time = self.elapsed_time - self.beat_time;
                if self.pattern.chars().nth(self.current_beat as usize).unwrap() == 'x' {
                    console::log_1(&self.current_beat.into());
                    let new_note = Note::new_with_params(20, calc_offset_time(t, i));
                    self.notes.push(new_note);
                }
                self.current_beat += 1;
                if self.current_beat == self.all_beats {
                    self.current_beat = 0;
                }
            }

            for n in self.notes.iter_mut() {
                let mut note_finished = false;
                output[i] += self.instrument.sound(calc_offset_time(t, i), n, &mut note_finished);

                if note_finished {
                    n.active = false;
                }
                output[i] *= self.master_volume;
            }
        }

        self.clear_finished_notes();

        output
    }
}

impl Sequencer {
    pub fn new(beats: u32, subbeats: u32, tempo: f64) -> Sequencer {
        Sequencer {
            beats, // 4
            subbeats, // 4
            tempo, // 90.0
            beat_time: (60.0 / tempo) / (subbeats as f64),
            pattern: String::from("x...x...x...x..."),
            // pattern: String::from("x..............."),
            instrument: Box::new(KickDrum::new()),
            notes: Vec::new(),
            elapsed_time: 0.0,
            all_beats: beats * subbeats,
            current_beat: 0,
            master_volume: 0.2
        }
    }

    fn clear_finished_notes(&mut self) {
        self.notes.retain(|n| n.active);
    }
}