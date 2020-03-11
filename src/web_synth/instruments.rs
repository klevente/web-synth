use crate::web_synth::envelopes::Envelope;
use crate::web_synth::{Note, scale};
use crate::web_synth::oscillators::sine_osc;

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

        let sound = 1.00 * sine_osc(t, scale(note.id + 12), 0.001, 5.0);

    }
}