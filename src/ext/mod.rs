/// Extensions for booleans
pub trait BoolExt {
    /// Toggle the value
    fn toggle(&mut self);
}

impl BoolExt for bool {
    fn toggle(&mut self) {
        // so `true` becomes `false` and vice versa
        *self = !*self;
    }
}

/// Extensions for floats
pub trait FloatExt {
    /// Add a number with a saturating upper bound
    fn cap_add(&mut self, rhs: Self, cap: Self);
    /// Subtract a number with a saturating lower bound
    fn cap_sub(&mut self, rhs: Self, cap: Self);
}

impl FloatExt for f32 {
    fn cap_add(&mut self, rhs: Self, cap: Self) {
        *self = (*self + rhs).min(cap)
    }
    fn cap_sub(&mut self, rhs: Self, cap: Self) {
        *self = (*self - rhs).max(cap)
    }
}

#[derive(Debug, Default)]
/// Tracks how many buttons are being pressed in specific directions
pub struct InputState {
    /// Up keys down
    pub up: u8,
    /// Down keys down
    pub down: u8,
    /// Left keys down
    pub left: u8,
    /// Right keys down
    pub right: u8,
}

impl InputState {
    #[inline]
    /// Returns `-1`, `0` or `1` depending on whether `self.hor` is negative, zero or positive
    pub fn hor(&self) -> f32 {
        (self.right as i8 - self.left as i8).signum() as f32
    }
    /// Returns `-1`, `0` or `1` depending on whether `self.ver` is negative, zero or positive
    #[inline]
    pub fn ver(&self) -> f32 {
        (self.down as i8 - self.up as i8).signum() as f32
    }
    #[inline]
    pub fn acltr(&self) -> bool {
        self.up != 0
    }
    #[inline]
    pub fn brk(&self) -> bool {
        self.down != 0
    }
}

#[derive(Debug, Default)]
pub struct MouseDown {
    pub left: bool,
    pub middle: bool,
    pub right: bool,
}
