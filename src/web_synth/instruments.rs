use crate::web_synth::envelopes::Envelope;
use crate::web_synth::{Note, scale, Source, SAMPLE_SIZE, SAMPLE_RATE};
use crate::web_synth::oscillators::sine_osc;

fn calc_offset_time(t: f32, sample_idx: usize) -> f32 {
    t + sample_idx as f32 / SAMPLE_RATE
}

// Synth with oscillator(s) and an envelope
pub trait Instrument {
    fn sound(&self, t: f32, note: Note, &mut note_finished: bool) -> f32;
}

pub struct Bell {
    envelope: Box<dyn Envelope>
}

impl Instrument for Bell {
    fn sound(&self, t: f32, note: Note, &mut note_finished: bool) -> f32 {
        let amplitude = self.envelope.amplitude(t, note.on, note.off);
        if amplitude <= 0.0 {
            note_finished = true;
        }

        let sound =
            1.00 * sine_osc(t - note.on, scale(note.id), 0.001, 5.0) +
            0.50 * sine_osc(t - note.on, scale(note.id + 12), 0.0, 0.0) +
            0.25 * sine_osc(t - note.on, scale(note.id + 24), 0.0, 0.0);

        sound
    }
}

impl Source for Bell {
    fn get_sample_block(&self, t: f32) -> [f32; 128] {
        let mut samples: [f32; 128] = [0.0; 128];
        for i in 0..SAMPLE_SIZE {
            samples[i] = self.sound(calc_offset_time(t, i));
        }
        samples
    }
}