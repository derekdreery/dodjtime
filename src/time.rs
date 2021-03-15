//! A time representation that works nicely with the RTC on an embedded chip.
//!
//! Uses the same type `Time` for both instants and durations.

use core::{
    fmt::{self, Display},
    ops,
};

/// frequency of the LFCLK in Hz (32,768)
const FREQ: u32 = 1 << 15;
/// Mask the bits from `Time` to only get the ticks. Truncate to u32 first.
const TICK_MASK: u32 = 0x00ff_ffff;
/// Mask the bits from `Time` to only get the overflows.
const OVERFLOW_MASK: u64 = !0x00ff_ffff;
/// The part of the time that represents less than 1 second.
const SUBSECS_MASK: u32 = FREQ - 1; // TODO check.

/// Our version of a duration/instant.
///
/// Counts in ticks and overflows of the RTC. We store the ticks in the bottom 24 bits the count of
/// overflows in the remaining 40 bits.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Time {
    bits: u64,
}

impl Time {
    pub const fn from_bits(bits: u64) -> Self {
        Time { bits }
    }

    pub const fn bits(&self) -> u64 {
        self.bits
    }

    pub fn ticks(&self) -> u32 {
        self.bits as u32 & TICK_MASK
    }

    /// Set the ticks part of the time.
    ///
    /// Only the bottom 24 ticks should be used. If the top ticks are set it's UB, but not memory
    /// error, and will panic in debug.
    pub fn set_ticks(&mut self, ticks: u32) {
        debug_assert_eq!(
            ticks & !TICK_MASK,
            0,
            "set_ticks called with top 8 bits not 0"
        );
        self.bits = (self.bits & OVERFLOW_MASK) | (ticks as u64);
    }

    pub fn increment_overflows(&mut self) {
        self.bits += 0x0100_0000;
    }

    pub fn overflows(&self) -> u64 {
        self.bits & OVERFLOW_MASK
    }
}

impl ops::Add for Time {
    type Output = Time;

    fn add(self, rhs: Self) -> Self::Output {
        debug_assert!(u64::MAX - self.bits > rhs.bits);
        Time {
            bits: self.bits + rhs.bits,
        }
    }
}

impl ops::Sub for Time {
    type Output = Time;

    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert!(self.bits > rhs.bits);
        Time {
            bits: self.bits - rhs.bits,
        }
    }
}

// Since we know the clock speed, we can convert between the standard duration and our time repr.
impl From<core::time::Duration> for Time {
    fn from(d: core::time::Duration) -> Self {
        // This checks that our duration (with a 64bit seconds field) will fit in a field that has
        // 64 bits for multiples 1/32758 of a second. i.e. we need more space because our tick is
        // smaller.
        assert!(d.as_secs() < (1 << (64 - 15)));

        // I think we lose some accuracy doing the arithmetic this way, but we can fit it all in a
        // u32.
        let subsec_ticks = d.subsec_nanos() / (1_000_000_000 / FREQ);
        // Scale up by the clock speed (in hz);
        let sec_ticks = d.as_secs() << 15;

        Self {
            bits: subsec_ticks as u64 + sec_ticks,
        }
    }
}

impl From<Time> for core::time::Duration {
    fn from(t: Time) -> Self {
        let secs = t.bits >> 15;
        let subsec_nanos = (t.bits as u32 & SUBSECS_MASK) * (1_000_000_000 / FREQ);
        Self::new(secs, subsec_nanos)
    }
}

// Use chrono
impl Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        chrono::Duration::from_std(core::time::Duration::from(*self))
            .unwrap()
            .fmt(f)
    }
}
