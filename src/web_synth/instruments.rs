use crate::web_synth::envelopes::{Envelope, ADSREnvelope, ADSRFixedEnvelope};
use crate::web_synth::{Note, piano_scale};
use crate::web_synth::oscillators::{sine_osc, noise_osc};

// Synth with oscillator(s) and an envelope
pub trait Instrument {
    fn sound(&self, t: f64, note: &Note, note_finished: &mut bool) -> f64;
}

pub struct Bell {
    envelope: ADSREnvelope
}

impl Instrument for Bell {
    fn sound(&self, t: f64, note: &Note, note_finished: &mut bool) -> f64 {
        let amplitude = self.envelope.amplitude(t, note.on, note.off);
        if t != note.on && amplitude <= 0.0 {
            *note_finished = true;
        }

        let sound =
            1.00 * sine_osc(t - note.on, piano_scale(note.id), 0.001, 5.0) +
            0.50 * sine_osc(t - note.on, piano_scale(note.id + 12), 0.0, 0.0) +
            0.25 * sine_osc(t - note.on, piano_scale(note.id + 24), 0.0, 0.0);

        amplitude * sound
    }
}

impl Bell {
    pub const fn new() -> Bell {
        Bell {
            envelope: ADSREnvelope::new()
        }
    }
}

pub struct KickDrum {
    envelope: ADSRFixedEnvelope
}

impl Instrument for KickDrum {
    fn sound(&self, t: f64, note: &Note, note_finished: &mut bool) -> f64 {
        let amplitude = self.envelope.amplitude(t, note.on, note.off);
        if t != note.on && amplitude <= 0.0 {
            *note_finished = true;
        }

        let sound =
            0.99 * sine_osc(t - note.on, piano_scale(note.id), 1.0, 1.0) +
            0.01 * noise_osc();

        amplitude * sound
    }
}

impl KickDrum {
    pub fn new() -> KickDrum {
        KickDrum {
            envelope: ADSRFixedEnvelope::new_with_params(
                0.1,
                0.15,
                0.0,
                0.0,
                1.0,
                1.5
            )
        }
    }
}

pub(crate) const BELL: Bell = Bell::new();
// pub(crate) const KICKDRUM: KickDrum = KickDrum::new();