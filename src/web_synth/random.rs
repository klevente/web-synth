
pub struct Random {
    seed: u32
}

impl Random {
    pub const fn new() -> Random {
        Random {
            seed: 0
        }
    }

    pub fn gen(&mut self) -> f64 {
        self.seed += 0xe120fc15;
        let mut tmp: u64 = self.seed as u64 * 0x4a39b70d;
        let m1: u32 = ((tmp >> 32) ^ tmp) as u32;
        tmp = m1 as u64 * 0x12fad5c9;
        let m2: u32 = ((tmp >> 32) ^ tmp) as u32;
        m2 as f64 / 4_294_967_295u32 as f64
    }
}

pub(crate) const RNG: Random = Random::new();