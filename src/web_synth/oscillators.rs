use std::f32::consts::PI;
use crate::web_synth::{Source, SAMPLE_RATE, SAMPLE_SIZE};


fn w(freq_hz: f32) -> f32 {
    2.0 * PI * freq_hz
}

fn modulate_freq(t: f32, freq_hz: f32, lfo_amplitude: f32, lfo_freq_hz: f32) -> f32 {
    w(freq_hz) * t + lfo_amplitude * freq_hz * (w(lfo_freq_hz) * t).sin()
}

fn calc_offset_time(t: f32, sample_idx: usize) -> f32 {
    t + sample_idx as f32 / SAMPLE_RATE
}

fn generate_samples<G>(t: f32, freq_hz: f32, lfo_amplitude: f32, lfo_freq_hz: f32, samples: &mut [f32], generator: G) where G: Fn(f32) -> f32 {
    for i in 0..SAMPLE_SIZE {
        let fm_freq = modulate_freq(calc_offset_time(t, i), freq_hz, lfo_amplitude, lfo_freq_hz);
        samples[i] = generator(fm_freq);
    }
}

pub fn sine_osc(t: f32, freq_hz: f32, lfo_amplitude: f32, lfo_freq_hz: f32) -> [f32; 128] {
    let mut samples: [f32; 128] = [0.0; 128];
    generate_samples(t, freq_hz, lfo_amplitude, lfo_freq_hz, &mut samples[..], |f| f.sin());
    samples
}

pub fn square_osc(t: f32, freq_hz: f32, lfo_amplitude: f32, lfo_freq_hz: f32) -> [f32; 128] {
    let mut samples: [f32; 128] = [0.0; 128];
    generate_samples(t, freq_hz, lfo_amplitude, lfo_freq_hz, &mut samples[..], |f| {
       let sine_sample = f.sin();
        if sine_sample > 0.0 { 1.0 } else { -1.0 }
    });
    samples
}

pub fn triangle_osc(t: f32, freq_hz: f32, lfo_amplitude: f32, lfo_freq_hz: f32) -> [f32; 128] {
    let mut samples: [f32; 128] = [0.0; 128];
    generate_samples(t, freq_hz, lfo_amplitude, lfo_freq_hz, &mut samples[..], |f| (freq.sin() * (2.0 / PI)).asin());
    samples
}

pub struct SineOscillator {
    freq_hz: f32,
    lfo_amplitude: f32,
    lfo_freq_hz: f32
}

impl SineOscillator {
    pub fn new(freq_hz: f32, lfo_amplitude: f32, lfo_freq_hz: f32) -> SineOscillator {
        SineOscillator {
            freq_hz,
            lfo_amplitude,
            lfo_freq_hz
        }
    }
}

impl Source for SineOscillator {
    fn get_sample_block(&self, t: f32) -> [f32; 128] {
        let mut samples: [f32; 128] = [0.0; 128];

        /*for i in 0..128 {
            samples[i] = modulate_freq(calc_offset_time(t, i), self.freq_hz, self.lfo_amplitude, self.lfo_freq_hz)
                            .sin();
        }*/
        generate_samples(t, self.freq_hz, self.lfo_amplitude, self.lfo_freq_hz, &mut samples[..], |f| f.sin());

        samples
    }
}

pub struct SquareOscillator {
    freq_hz: f32,
    lfo_amplitude: f32,
    lfo_freq_hz: f32
}

impl SquareOscillator {
    pub fn new(freq_hz: f32, lfo_amplitude: f32, lfo_freq_hz: f32) -> SquareOscillator {
        SquareOscillator {
            freq_hz,
            lfo_amplitude,
            lfo_freq_hz
        }
    }
}

impl Source for SquareOscillator {
    fn get_sample_block(&self, t: f32) -> [f32; 128] {
        let mut samples: [f32; 128] = [0.0; 128];

        for i in 0..128 {
            let sine_sample = modulate_freq(calc_offset_time(t, i), self.freq_hz, self.lfo_amplitude, self.lfo_freq_hz)
                .sin();

            samples[i] = if sine_sample > 0.0 { 1.0 } else { -1.0 };
        }

        samples
    }
}

pub struct TriangleOscillator {
    freq_hz: f32,
    lfo_amplitude: f32,
    lfo_freq_hz: f32
}

impl TriangleOscillator {
    pub fn new(freq_hz: f32, lfo_amplitude: f32, lfo_freq_hz: f32) -> TriangleOscillator {
        TriangleOscillator {
            freq_hz,
            lfo_amplitude,
            lfo_freq_hz
        }
    }
}

impl Source for TriangleOscillator {
    fn get_sample_block(&self, t: f32) -> [f32; 128] {
        let mut samples: [f32; 128] = [0.0; 128];

        for i in 0..128 {
            let freq = modulate_freq(calc_offset_time(t, i), self.freq_hz, self.lfo_amplitude, self.lfo_freq_hz);
            samples[i] = (freq.sin() * (2.0 / PI)).asin();
        }

        samples
    }
}