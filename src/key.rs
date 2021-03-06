use core::num::{NonZeroU16, NonZeroU32, NonZeroU8, NonZeroUsize};

/// Types implementing this trait can be used as keys for all Rodeos
///
/// # Safety
///
/// into/from must be perfectly symmetrical, any key that goes on must be perfectly reproduced with the other
///
/// [`ReadOnlyLasso`]: crate::ReadOnlyLasso
pub unsafe trait Key: Copy + Eq {
    /// Returns the `usize` that represents the current key
    ///
    /// # Safety
    ///
    /// To be safe, `into_usize` and `{try}_from_usize` must be symmetrical, meaning that any usize given
    /// to `into_usize` must be the same after going through `{try}_from_usize`
    ///
    unsafe fn into_usize(self) -> usize;

    /// Attempts to create a key from a `usize`, returning `None` if it fails
    fn try_from_usize(int: usize) -> Option<Self>;
}

/// The default key for every Rodeo, the same size as a `usize`
///
/// Internally is a `NonZeroUsize` to allow for space optimizations when stored inside of an [`Option`]
///
/// [`ReadOnlyLasso`]: crate::ReadOnlyLasso
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html   
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Spur {
    key: NonZeroUsize,
}

unsafe impl Key for Spur {
    #[inline]
    unsafe fn into_usize(self) -> usize {
        self.key.get() - 1
    }

    /// Returns `None` if `int` is greater than `usize::MAX - 1`
    #[inline]
    fn try_from_usize(int: usize) -> Option<Self> {
        if int < usize::max_value() {
            // Safety: The integer is less than the max value and then incremented by one, meaning that
            // is is impossible for a zero to inhabit the NonZeroUsize
            unsafe {
                Some(Self {
                    key: NonZeroUsize::new_unchecked(int + 1),
                })
            }
        } else {
            None
        }
    }
}

/// A small Key, utilizing only 32 bits of space
///
/// Internally is a `NonZeroU32` to allow for space optimizations when stored inside of an [`Option`]
///
/// [`ReadOnlyLasso`]: crate::ReadOnlyLasso
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html   
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SmallSpur {
    key: NonZeroU32,
}

unsafe impl Key for SmallSpur {
    #[inline]
    unsafe fn into_usize(self) -> usize {
        self.key.get() as usize - 1
    }

    /// Returns `None` if `int` is greater than `usize::MAX - 1`
    #[inline]
    fn try_from_usize(int: usize) -> Option<Self> {
        if int < u32::max_value() as usize {
            // Safety: The integer is less than the max value and then incremented by one, meaning that
            // is is impossible for a zero to inhabit the NonZeroU32
            unsafe {
                Some(Self {
                    key: NonZeroU32::new_unchecked(int as u32 + 1),
                })
            }
        } else {
            None
        }
    }
}

/// A miniature Key utilizing only 16 bits of space
///
/// Internally is a `NonZeroU16` to allow for space optimizations when stored inside of an [`Option`]
///
/// [`ReadOnlyLasso`]: crate::ReadOnlyLasso
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html   
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct MiniSpur {
    key: NonZeroU16,
}

unsafe impl Key for MiniSpur {
    #[inline]
    unsafe fn into_usize(self) -> usize {
        self.key.get() as usize - 1
    }

    /// Returns `None` if `int` is greater than `usize::MAX - 1`
    #[inline]
    fn try_from_usize(int: usize) -> Option<Self> {
        if int < u16::max_value() as usize {
            // Safety: The integer is less than the max value and then incremented by one, meaning that
            // is is impossible for a zero to inhabit the NonZeroU16
            unsafe {
                Some(Self {
                    key: NonZeroU16::new_unchecked(int as u16 + 1),
                })
            }
        } else {
            None
        }
    }
}

/// A miniature Key utilizing only 8 bits of space
///
/// Internally is a `NonZeroU8` to allow for space optimizations when stored inside of an [`Option`]
///
/// [`ReadOnlyLasso`]: crate::ReadOnlyLasso
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html   
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct MicroSpur {
    key: NonZeroU8,
}

unsafe impl Key for MicroSpur {
    #[inline]
    unsafe fn into_usize(self) -> usize {
        self.key.get() as usize - 1
    }

    /// Returns `None` if `int` is greater than `usize::MAX - 1`
    #[inline]
    fn try_from_usize(int: usize) -> Option<Self> {
        if int < u8::max_value() as usize {
            // Safety: The integer is less than the max value and then incremented by one, meaning that
            // is is impossible for a zero to inhabit the NonZeroU16
            unsafe {
                Some(Self {
                    key: NonZeroU8::new_unchecked(int as u8 + 1),
                })
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cord() {
        let zero = Spur::try_from_usize(0).unwrap();
        let max = Spur::try_from_usize(usize::max_value() - 1).unwrap();

        unsafe {
            assert_eq!(zero.into_usize(), 0);
            assert_eq!(max.into_usize(), usize::max_value() - 1);
        }
    }

    #[test]
    fn cord_max_returns_none() {
        assert_eq!(None, Spur::try_from_usize(usize::max_value()));
    }

    #[test]
    #[should_panic]
    #[cfg(not(miri))]
    fn cord_max_panics() {
        Spur::try_from_usize(usize::max_value()).unwrap();
    }

    #[test]
    fn small_cord() {
        let zero = SmallSpur::try_from_usize(0).unwrap();
        let max = SmallSpur::try_from_usize(u32::max_value() as usize - 1).unwrap();
        unsafe {
            assert_eq!(zero.into_usize(), 0);
            assert_eq!(max.into_usize(), u32::max_value() as usize - 1);
        }
    }

    #[test]
    fn small_cord_returns_none() {
        assert_eq!(None, SmallSpur::try_from_usize(u32::max_value() as usize));
    }

    #[test]
    #[should_panic]
    #[cfg(not(miri))]
    fn small_cord_panics() {
        SmallSpur::try_from_usize(u32::max_value() as usize).unwrap();
    }

    #[test]
    fn mini_cord() {
        let zero = MiniSpur::try_from_usize(0).unwrap();
        let max = MiniSpur::try_from_usize(u16::max_value() as usize - 1).unwrap();
        unsafe {
            assert_eq!(zero.into_usize(), 0);
            assert_eq!(max.into_usize(), u16::max_value() as usize - 1);
        }
    }

    #[test]
    fn mini_cord_returns_none() {
        assert_eq!(None, MiniSpur::try_from_usize(u16::max_value() as usize));
    }

    #[test]
    #[should_panic]
    #[cfg(not(miri))]
    fn mini_cord_panics() {
        MiniSpur::try_from_usize(u16::max_value() as usize).unwrap();
    }

    #[test]
    fn micro_cord() {
        let zero = MicroSpur::try_from_usize(0).unwrap();
        let max = MicroSpur::try_from_usize(u8::max_value() as usize - 1).unwrap();
        unsafe {
            assert_eq!(zero.into_usize(), 0);
            assert_eq!(max.into_usize(), u8::max_value() as usize - 1);
        }
    }

    #[test]
    fn micro_cord_returns_none() {
        assert_eq!(None, MicroSpur::try_from_usize(u8::max_value() as usize));
    }

    #[test]
    #[should_panic]
    #[cfg(not(miri))]
    fn micro_cord_panics() {
        MicroSpur::try_from_usize(u8::max_value() as usize).unwrap();
    }
}
