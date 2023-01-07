pub const PCG32_MULTIPLIER: u64 = 6364136223846793005_u64;
pub const PCG32_INCREMENT: u64 = 1442695040888963407_u64;

pub struct IterMut<'a> {
    pcg: &'a mut PCG32,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.pcg.get_u32())
    }
}

pub struct PCG32 {
    state: u64,
    multiplier: u64,
    increment: u64,
}

impl PCG32 {
    /// `stream` should be less than 1 << 63
    pub fn new(seed: u64, multiplier: u64, stream: u64) -> Self {
        // We should make sure that increment is odd
        let increment = (stream << 1) | 1;
        let mut pcg = PCG32 {
            state: seed.wrapping_add(increment),
            multiplier,
            increment,
        };

        pcg.next();
        pcg
    }

    pub fn new_default(seed: u64) -> Self {
        let multiplier = PCG32_MULTIPLIER;
        let increment = PCG32_INCREMENT;
        let mut pcg = PCG32 {
            state: seed.wrapping_add(increment),
            multiplier,
            increment,
        };

        pcg.next();
        pcg
    }

    #[inline]
    pub fn next(&mut self) {
        self.state = self
            .state
            .wrapping_mul(self.multiplier)
            .wrapping_add(self.increment);
    }

    #[inline]
    /// Advance the PCG by `delta` steps in O(lg(`delta)) time. By passing
    /// a negative i64 as u64, it can go back too.
    pub fn advance(&mut self, mut delta: u64) {
        let mut acc_mult = 1u64;
        let mut acc_incr = 0u64;
        let mut cur_mult = self.multiplier;
        let mut cur_incr = self.increment;

        while delta > 0 {
            if delta & 1 != 0 {
                acc_mult = acc_mult.wrapping_mul(cur_mult);
                acc_incr = acc_incr.wrapping_mul(cur_mult).wrapping_add(cur_incr);
            }

            cur_incr = cur_mult.wrapping_add(1).wrapping_mul(cur_incr);
            cur_mult = cur_mult.wrapping_mul(cur_mult);
            delta >>= 1;
        }

        self.state = acc_mult.wrapping_mul(self.state).wrapping_add(acc_incr);
    }

    #[inline]
    pub fn get_u32(&mut self) -> u32 {
        let mut x = self.state;
        let count = (x >> 59) as u32;

        self.next();

        x ^= x >> 18;
        ((x >> 27) as u32).rotate_right(count)
    }

    #[inline]
    pub fn get_u64(&mut self) -> u64 {
        self.get_u32() as u64 ^ ((self.get_u32() as u64) << 32)
    }

    #[inline]
    pub fn get_u16(&mut self) -> (u16, u16) {
        let res = self.get_u32();

        (res as u16, (res >> 16) as u16)
    }

    #[inline]
    pub fn get_u8(&mut self) -> (u8, u8, u8, u8) {
        let res = self.get_u32();

        (
            res as u8,
            (res >> 8) as u8,
            (res >> 16) as u8,
            (res >> 24) as u8,
        )
    }

    #[inline]
    pub fn get_state(&mut self) -> u64 {
        self.state
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut { pcg: self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_birthday() {
        let numbers = 1e5 as usize;
        let mut pcg = PCG32::new_default(314159);
        let mut pcg2 = PCG32::new_default(314159);

        assert_eq!(pcg.get_u32(), pcg2.get_u32());

        let mut randoms: Vec<u32> = pcg.iter_mut().take(numbers).collect::<Vec<u32>>();

        pcg2.advance(1000);
        assert_eq!(pcg2.get_u32(), randoms[1000]);

        pcg2.advance((-1001_i64) as u64);
        assert_eq!(pcg2.get_u32(), randoms[0]);
        randoms.sort_unstable();
        randoms.dedup();
        assert_eq!(randoms.len(), numbers);
    }
}
