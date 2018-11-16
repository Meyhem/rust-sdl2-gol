use std::time::Duration;

pub trait ToMilis {
    fn as_safe_milis(self: Self) -> u64;
}

impl ToMilis for Duration {
    fn as_safe_milis(self: Self) -> u64 {
        let ret = (self.subsec_millis() as u64) + (self.as_secs() as u64) * 1_000u64;
        ret
    }
}
