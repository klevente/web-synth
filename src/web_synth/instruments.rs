use web_sys::console;
use crate::web_synth::envelopes::{Envelope, ADSREnvelope};
use crate::web_synth::{Note, scale, Source, SAMPLE_SIZE, SAMPLE_RATE};
use crate::web_synth::oscillators::sine_osc;

fn calc_offset_time(t: f64, sample_idx: usize) -> f64 {
    t + sample_idx as f64 / SAMPLE_RATE
}

// Synth with oscillator(s) and an envelope
pub trait Instrument {
    fn sound(&self, t: f64, note: &Note, note_finished: &mut bool) -> f64;
}

pub struct Bell {
    envelope: ADSREnvelope
}

impl Instrument for Bell {
    fn sound(&self, t: f64, note: &Note, note_finished: &mut bool) -> f64 {
        // console::log_3(&note.id.into(), &note.off.into(), &note.on.into());
        let amplitude = self.envelope.amplitude(t, note.on, note.off);
        if amplitude <= 0.0 {
            *note_finished = true;
        }

        let sound =
            1.00 * sine_osc(t - note.on, scale(note.id), 0.00, 0.0); // +
            // 0.50 * sine_osc(t - note.on, scale(note.id + 12), 0.0, 0.0) +
            // 0.25 * sine_osc(t - note.on, scale(note.id + 24), 0.0, 0.0);

        // amplitude * sound
        sound
    }
}

impl Bell {
    pub const fn new() -> Bell {
        Bell {
            envelope: ADSREnvelope::new()
        }
    }
}

pub(crate) const BELL: Bell = Bell::new();