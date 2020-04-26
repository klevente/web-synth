use crate::web_synth::envelopes::{Envelope, ADSREnvelope, ADSRFixedEnvelope};
use crate::web_synth::{Note, piano_scale};
use crate::web_synth::oscillators::{sine_osc, noise_osc, OscillatorType, OscillatorFunction, square_osc, triangle_osc};

pub struct InstrumentParam {
    amplitude: f64,
    oscillator: OscillatorFunction,
    note_offset: i32,
    lfo_amp: f64,
    lfo_freq_hz: f64

}

impl InstrumentParam {
    pub fn new(amplitude: f64, oscillator: OscillatorType, note_offset: i32, lfo_amp: f64, lfo_freq_hz: f64) -> InstrumentParam {
        InstrumentParam {
            amplitude,
            oscillator: InstrumentParam::get_oscillator_function(oscillator),
            note_offset,
            lfo_amp,
            lfo_freq_hz
        }
    }

    pub fn gen(&self, t: f64, note_id: u32) -> f64 {
        self.amplitude * (self.oscillator)(t, piano_scale((note_id as i32 + self.note_offset) as u32), self.lfo_amp, self.lfo_freq_hz)
    }

    fn get_oscillator_function(oscillator: OscillatorType) -> OscillatorFunction {
        match oscillator {
            OscillatorType::Sine => sine_osc,
            OscillatorType::Square => square_osc,
            OscillatorType::Triangle => triangle_osc,
            OscillatorType::Noise => noise_osc
        }
    }
}

// Synth with oscillator(s) and an envelope
pub trait Instrument {
    fn sound(&self, t: f64, note: &Note, note_finished: &mut bool) -> f64;
}

pub struct CustomInstrument {
    envelope: Box<dyn Envelope>,
    params: Vec<InstrumentParam>
}

impl Instrument for CustomInstrument {
    fn sound(&self, t: f64, note: &Note, note_finished: &mut bool) -> f64 {
        let amplitude = self.envelope.amplitude(t, note.on, note.off);
        if t != note.on && amplitude <= 0.0 {
            *note_finished = true;
        }

        let sound =
            self.params
                .iter()
                .fold(0.0, |acc, p| {
                    acc + p.gen(t - note.on, note.id)
                });


        amplitude * sound
    }
}

impl CustomInstrument {
    pub fn new(envelope: Box<dyn Envelope>, params: Vec<InstrumentParam>) -> CustomInstrument {
        CustomInstrument {
            envelope,
            params
        }
    }
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
            0.01 * noise_osc(0.0, 0.0, 0.0, 0.0);

        amplitude * sound
    }
}

impl KickDrum {
    pub const fn new() -> KickDrum {
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

/*struct Instruments {
    pub bell: Bell,
    pub kickdrum: KickDrum
}

// maybe use pointers instead of references
impl Instruments {
    pub const fn new() -> Instruments {
        Instruments {
            bell: Bell::new(),
            kickdrum: KickDrum:: new()
        }
    }

    pub fn get_by_name(&self, name: &str) -> &'static dyn Instrument {
        match name {
            "bell" => &self.bell,
            "kickdrum" => &self.kickdrum,
            _ => &self.bell
        }
    }
}

pub(crate) const INSTRUMENTS: Instruments = Instruments::new();
*/

pub fn get_instrument(name: &str) -> Box<dyn Instrument> {
    Box::new(
        match name {
            "bell" => CustomInstrument::new(
                Box::new(ADSREnvelope::new()),
                vec![
                    InstrumentParam::new(1.0, OscillatorType::Sine, 0, 0.001, 5.0),
                    InstrumentParam::new(0.5, OscillatorType::Sine, 12, 0.0, 0.0),
                    InstrumentParam::new(0.25, OscillatorType::Sine, 24, 0.0, 0.0)
                ]
            ),
            "kickdrum" => CustomInstrument::new(
                Box::new(ADSRFixedEnvelope::new_with_params(0.1, 0.15, 0.0, 0.0, 1.0, 1.5)),
                vec![
                    InstrumentParam::new(0.99, OscillatorType::Sine, 0, 1.0, 1.0),
                    InstrumentParam::new(0.01, OscillatorType::Noise, 0, 0.0, 0.0)
                ]
            ),
            "snaredrum" => CustomInstrument::new(
                Box::new(ADSRFixedEnvelope::new_with_params(0.0, 0.2, 0.0, 0.0, 1.0, 1.0)),
                vec![
                    InstrumentParam::new(0.5, OscillatorType::Sine, -24, 1.0, 0.5),
                    InstrumentParam::new(0.5, OscillatorType::Noise, 0, 0.0, 0.0)
                ]
            ),
            "hihat" => CustomInstrument::new(
                Box::new(ADSRFixedEnvelope::new_with_params(0.01, 0.05, 0.0, 0.0, 1.0, 1.0)),
                vec![
                    InstrumentParam::new(0.1, OscillatorType::Square, -12, 1.0, 1.5),
                    InstrumentParam::new(0.9, OscillatorType::Noise, 0, 0.0, 0.0)
                ]
            ),
            _ => CustomInstrument::new(
                Box::new(ADSREnvelope::new()),
                vec![]
            )
        }
    )


    /*Box::new(
        match name {
            "bell" => Bell:new(),
            "kickdrum" => KickDrum::new(),
            _ => Bell::new()
        }
    )*/

    /*match name {
        "bell" => Box::new(Bell::new()),
        "kickdrum" => Box::new(KickDrum::new()),
        _ => Box::new(Bell::new())
    }*/
}

// pub(crate) const BELL: Bell = Bell::new();
// pub(crate) const KICKDRUM: KickDrum = KickDrum::new();