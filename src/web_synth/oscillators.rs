use std::f64::consts::PI;
use crate::web_synth::random::RNG;

fn w(freq_hz: f64) -> f64 {
    2.0 * PI * freq_hz
}

fn modulate_freq(t: f64, freq_hz: f64, lfo_amplitude: f64, lfo_freq_hz: f64) -> f64 {
    w(freq_hz) * t + lfo_amplitude * freq_hz * (w(lfo_freq_hz) * t).sin()
}

pub fn sine_osc(t: f64, freq_hz: f64, lfo_amplitude: f64, lfo_freq_hz: f64) -> f64 {
    modulate_freq(t, freq_hz, lfo_amplitude, lfo_freq_hz).sin()
}

pub fn square_osc(t: f64, freq_hz: f64, lfo_amplitude: f64, lfo_freq_hz: f64) -> f64 {
    // let sine_sample = sine_osc(t, freq_hz, lfo_amplitude, lfo_freq_hz);
    // if sine_sample > 0.0 { 1.0 } else { -1.0 }
    sine_osc(t, freq_hz, lfo_amplitude, lfo_freq_hz).signum()
}

pub fn triangle_osc(t: f64, freq_hz: f64, lfo_amplitude: f64, lfo_freq_hz: f64) -> f64 {
    (sine_osc(t, freq_hz, lfo_amplitude, lfo_freq_hz) * (2.0 / PI)).asin()
}

pub fn noise_osc() -> f64 {
    2.0 * RNG.gen() - 1.0
}

/*fn generate_samples<G>(t: f64, freq_hz: f64, lfo_amplitude: f64, lfo_freq_hz: f64, samples: &mut [f64], generator: G) where G: Fn(f64) -> f64 {
    for i in 0..SAMPLE_SIZE {
        let fm_freq = modulate_freq(calc_offset_time(t, i), freq_hz, lfo_amplitude, lfo_freq_hz);
        samples[i] = generator(fm_freq);
    }
}*/

/* pub struct SineOscillator {
    freq_hz: f64,
    lfo_amplitude: f64,
    lfo_freq_hz: f64,
    note: u32
}

impl SineOscillator {
    pub fn new(freq_hz: f64, lfo_amplitude: f64, lfo_freq_hz: f64, note: u32) -> SineOscillator {
        SineOscillator {
            freq_hz,
            lfo_amplitude,
            lfo_freq_hz,
            note
        }
    }
}

impl Source for SineOscillator {
    fn get_sample_block(&self, t: f64) -> [f64; 128] {
        let mut samples: [f64; 128] = [0.0; 128];

        for i in 0..128 {
            samples[i] = sine_osc(calc_offset_time(t, i), scale(self.note), self.lfo_amplitude, self.lfo_freq_hz);
        }
        // generate_samples(t, self.freq_hz, self.lfo_amplitude, self.lfo_freq_hz, &mut samples[..], |f| f.sin());

        samples
    }
}

pub struct SquareOscillator {
    freq_hz: f64,
    lfo_amplitude: f64,
    lfo_freq_hz: f64
}

impl SquareOscillator {
    pub fn new(freq_hz: f64, lfo_amplitude: f64, lfo_freq_hz: f64) -> SquareOscillator {
        SquareOscillator {
            freq_hz,
            lfo_amplitude,
            lfo_freq_hz
        }
    }
}

impl Source for SquareOscillator {
    fn get_sample_block(&self, t: f64) -> [f64; 128] {
        let mut samples: [f64; 128] = [0.0; 128];

        for i in 0..128 {
            let sine_sample = modulate_freq(calc_offset_time(t, i), self.freq_hz, self.lfo_amplitude, self.lfo_freq_hz)
                .sin();

            samples[i] = if sine_sample > 0.0 { 1.0 } else { -1.0 };
        }

        samples
    }
}

pub struct TriangleOscillator {
    freq_hz: f64,
    lfo_amplitude: f64,
    lfo_freq_hz: f64
}

impl TriangleOscillator {
    pub fn new(freq_hz: f64, lfo_amplitude: f64, lfo_freq_hz: f64) -> TriangleOscillator {
        TriangleOscillator {
            freq_hz,
            lfo_amplitude,
            lfo_freq_hz
        }
    }
}

impl Source for TriangleOscillator {
    fn get_sample_block(&self, t: f64) -> [f64; 128] {
        let mut samples: [f64; 128] = [0.0; 128];

        for i in 0..128 {
            let freq = modulate_freq(calc_offset_time(t, i), self.freq_hz, self.lfo_amplitude, self.lfo_freq_hz);
            samples[i] = (freq.sin() * (2.0 / PI)).asin();
        }

        samples
    }
} */