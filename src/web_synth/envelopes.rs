use web_sys::console;

pub trait Envelope {
    fn amplitude(&self, t: f64, time_on: f64, time_off: f64) -> f64;
}

pub struct ADSREnvelope {
    attack_time: f64,
    decay_time: f64,
    sustain_amplitude: f64,
    release_time: f64,
    start_amplitude: f64
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

        if amplitude <= 0.0000001 {
            return 0.0;
        }

        amplitude
    }
}

impl ADSREnvelope {
    pub const fn new() -> ADSREnvelope {
        ADSREnvelope {
            attack_time: 0.1,
            decay_time: 0.1,
            sustain_amplitude: 1.0,
            release_time: 0.2,
            start_amplitude: 1.0
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

pub struct ADSRFixedEnvelope {
    attack_time: f64,
    decay_time: f64,
    sustain_amplitude: f64,
    release_time: f64,
    start_amplitude: f64,
    lifetime: f64
}

impl Envelope for ADSRFixedEnvelope {
    fn amplitude(&self, t: f64, time_on: f64, time_off: f64) -> f64 {

        let act_time_off =
            if time_off == 0.0 && t - time_on >= self.lifetime {
                t
            } else {
                time_off
            };

        let amplitude =
            // note is currently on
            if time_on > act_time_off {
                let lifetime = t - time_on;
                self.calculate_amplitude(lifetime)
                // note is off
            } else {
                let lifetime = act_time_off - time_on;
                let release_amplitude = self.calculate_amplitude(lifetime);
                ((t - act_time_off) / self.release_time) * (0.0 - release_amplitude) + release_amplitude
            };

        if amplitude <= 0.0000001 {
            return 0.0;
        }

        amplitude
    }
}

impl ADSRFixedEnvelope {

    pub const fn new() -> ADSRFixedEnvelope {
        ADSRFixedEnvelope {
            attack_time: 0.1,
            decay_time: 0.1,
            sustain_amplitude: 1.0,
            release_time: 0.2,
            start_amplitude: 1.0,
            lifetime: 15.0 // set to inf and add two functions: with lifetime and without
        }
    }

    pub const fn new_with_params(attack_time: f64, decay_time: f64, sustain_amplitude: f64, release_time: f64, start_amplitude: f64, lifetime: f64) -> ADSRFixedEnvelope {
        ADSRFixedEnvelope {
            attack_time,
            decay_time,
            sustain_amplitude,
            release_time,
            start_amplitude,
            lifetime
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