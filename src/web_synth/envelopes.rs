pub trait Envelope {
    fn amplitude(&self, t: f32, time_on: f32, time_off: f32) -> f32;
}

pub struct ADSREnvelope {
    attack_time: f32,
    decay_time: f32,
    sustain_amplitude: f32,
    release_time: f32,
    start_amplitude: f32,
}

impl Envelope for ADSREnvelope {
    fn amplitude(&self, t: f32, time_on: f32, time_off: f32) -> f32 {
        let mut amplitude: f32 = 0.0;
        let mut release_amplitude: f32 = 0.0;

        // note is currently on
        if time_on > time_off {
            let lifetime = t - time_on;
            amplitude = self.calculate_amplitude(lifetime);
        } else {
            let lifetime = time_off - time_on;
            release_amplitude = self.calculate_amplitude(lifetime);
            amplitude = ((t - time_off) / self.release_time) * (0.0 - release_amplitude) + release_amplitude;
        }

        if amplitude <= 0.01 {
            amplitude = 0.0;
        }

        amplitude
    }
}

impl ADSREnvelope {
    pub fn new() -> ADSREnvelope {
        ADSREnvelope {
            attack_time: 0.1,
            decay_time: 0.1,
            sustain_amplitude: 1.0,
            release_time: 0.2,
            start_amplitude: 1.0,
        }
    }

    fn calculate_amplitude(&self, lifetime: f32) -> f32 {
        if lifetime <= self.attack_time {
            (lifetime / self.attack_time) * self.start_amplitude
        } else if lifetime <= self.attack_time + self.decay_time {
            ((lifetime - self.attack_time) / self.decay_time) * (self.sustain_amplitude - self.start_amplitude) + self.start_amplitude
        } else if lifetime > self.attack_time + self.decay_time {
            self.sustain_amplitude
        }
    }
}