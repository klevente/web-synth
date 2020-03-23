pub trait Envelope {
    fn amplitude(&self, t: f64, time_on: f64, time_off: f64) -> f64;
}

pub struct ADSREnvelope {
    attack_time: f64,
    decay_time: f64,
    sustain_amplitude: f64,
    release_time: f64,
    start_amplitude: f64,
}

impl Envelope for ADSREnvelope {
    fn amplitude(&self, t: f64, time_on: f64, time_off: f64) -> f64 {

        let amplitude =
            // note is currently on
            if time_on > time_off {
                let lifetime = t - time_on;
                self.calculate_amplitude(lifetime)
            // note is off
            } else {
                let lifetime = time_off - time_on;
                let release_amplitude = self.calculate_amplitude(lifetime);
                ((t - time_off) / self.release_time) * (0.0 - release_amplitude) + release_amplitude
            };

        if amplitude <= 0.01 {
            return 0.0;
        }

        amplitude
    }
}

impl ADSREnvelope {
    pub const fn new() -> ADSREnvelope {
        ADSREnvelope {
            attack_time: 5.0,
            decay_time: 1.0,
            sustain_amplitude: 1.0,
            release_time: 5.0,
            start_amplitude: 1.6,
        }
    }

    fn calculate_amplitude(&self, lifetime: f64) -> f64 {
        if lifetime <= self.attack_time {
            (lifetime / self.attack_time) * self.start_amplitude
        } else if lifetime <= self.attack_time + self.decay_time {
            ((lifetime - self.attack_time) / self.decay_time) * (self.sustain_amplitude - self.start_amplitude) + self.start_amplitude
        } else if lifetime > self.attack_time + self.decay_time {
            self.sustain_amplitude
        } else {
            0.0
        }
    }
}