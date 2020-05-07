use crate::web_synth::{Note, MutSource, SAMPLE_SIZE, calc_offset_time, DELTA_TIME};
use crate::web_synth::instruments::{Instrument, KickDrum, Bell, get_instrument};
use web_sys::console;


struct Channel {
    instrument: Box<dyn Instrument>,
    pattern: Vec<BeatNote>,
}

impl Channel {
    pub fn new(instrument: Box<dyn Instrument>, pattern: &str) -> Channel {
        /*let mut p: Vec<BeatNote> = Vec::with_capacity(pattern.len());

        pattern.chars().for_each(|c| {
            match c {
                'x' => p.push(BeatNote::Present { note_id: 20 }),
                '.' => p.push(BeatNote::Empty),
                _ =>   p.push(BeatNote::Empty)
            }
        });*/

        /* let out: Vec<BeatNote> = pattern.chars().map(|c| {
            match c {
                'x' => BeatNote::Present { note_id: 20 },
                '.' => BeatNote::Empty,
                _ =>   BeatNote::Empty
            }
        }).collect();*/

        Channel {
            instrument,
            pattern: Channel::build_pattern(pattern),
        }
    }

    pub fn update_instrument(&mut self, instrument: Box<dyn Instrument>) {
        self.instrument = instrument;
    }

    pub fn update_pattern(&mut self, pattern: &str) {
        self.pattern = Channel::build_pattern(pattern);
    }

    fn build_pattern(pattern: &str) -> Vec<BeatNote> {
        pattern
            .chars()
            .map(|c| {
                match c {
                    'x' => BeatNote::Present { note_id: 20 },
                    '.' => BeatNote::Empty,
                    _ => BeatNote::Empty
                }
            })
            .collect()
    }
}

#[derive(Debug)]
enum BeatNote {
    Empty,
    Present { note_id: u32 }, // add extra params, like length here
}

pub struct MultiSequencer {
    beat_time: f64,
    all_beats: usize,
    current_beat: usize,
    elapsed_time: f64,
    master_volume: f64,

    notes: Vec<Note>,

    channels: Vec<Channel>,
}

impl MutSource for MultiSequencer {
    fn get_sample_block(&mut self, t: f64) -> [f64; 128] {
        let mut output: [f64; 128] = [0.0; 128];
        for i in 0..SAMPLE_SIZE {
            self.elapsed_time += DELTA_TIME;
            if self.elapsed_time >= self.beat_time {
                self.elapsed_time -= self.beat_time;

                for (channel_idx, channel) in self.channels.iter().enumerate() {
                    let current_beat: &BeatNote = channel.pattern.get(self.current_beat).unwrap_or(&BeatNote::Empty);
                    console::log_1(&format!("{:?}", current_beat).into());

                    match current_beat {
                        BeatNote::Present { note_id } => {
                            let new_note = Note::new_with_params(*note_id, calc_offset_time(t, i), channel_idx);
                            self.notes.push(new_note);
                        }
                        _ => ()
                    }
                }

                self.current_beat += 1;
                if self.current_beat == self.all_beats {
                    self.current_beat = 0;
                }
            }

            for n in self.notes.iter_mut() {
                let mut note_finished = false;
                output[i] +=
                    match self.channels
                        .get(n.channel) {
                        Some(channel) => {
                            channel
                                .instrument
                                .sound(calc_offset_time(t, i), n, &mut note_finished)
                        }
                        None => {
                            console::log_2(&"Did not find channel:".into(), &(n.channel as u32).into());
                            0.0
                        }
                    };


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

impl MultiSequencer {
    pub fn new(beats: usize, sub_beats: usize, tempo: f64) -> MultiSequencer {
        MultiSequencer {
            beat_time: (60.0 / tempo) / (sub_beats as f64),
            all_beats: beats * sub_beats,
            current_beat: 0,
            elapsed_time: 0.0,
            master_volume: 1.0,

            notes: Vec::with_capacity(16),

            channels: Vec::with_capacity(8),
        }
    }

    pub fn add_channel(&mut self, instrument_name: &str, pattern: &str) -> usize {
        self.channels.push(Channel::new(get_instrument(instrument_name), pattern));
        self.channels.len() - 1
    }

    pub fn update_channel_instrument(&mut self, channel_index: usize, instrument_name: &str) {
        self.channels
            .get_mut(channel_index)
            .unwrap()
            .update_instrument(get_instrument(instrument_name));
    }

    pub fn update_channel_pattern(&mut self, channel_index: usize, pattern: &str) {
        self.channels
            .get_mut(channel_index)
            .unwrap()
            .update_pattern(pattern)
    }

    pub fn remove_channel(&mut self, channel_index: usize) {
        self.notes.retain(|n| n.channel != channel_index);
        self.channels.remove(channel_index);
    }

    pub fn clear_channels(&mut self) {
        self.channels.clear();
    }

    pub fn update_global_data(&mut self, beats: usize, sub_beats: usize, tempo: f64) {
        self.channels.clear();
        self.notes.clear();
        self.beat_time = (60.0 / tempo) / (sub_beats as f64);
        self.all_beats = beats * sub_beats;
        self.current_beat = 0;
        self.elapsed_time = 0.0;
    }

    fn clear_finished_notes(&mut self) {
        self.notes.retain(|n| n.active);
    }
}

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
    master_volume: f64,
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
                    let new_note = Note::new_with_params(20, calc_offset_time(t, i), 0);
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
            }
            output[i] *= self.master_volume;
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
            master_volume: 0.2,
        }
    }

    fn clear_finished_notes(&mut self) {
        self.notes.retain(|n| n.active);
    }
}