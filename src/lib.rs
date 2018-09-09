//! Compile-time rational arithmetic built on top of [`typenum`].
//!
//! [`typenum`]: https://docs.rs/typenum/

extern crate typenum;

pub mod consts;
mod operator_aliases;
mod ratio;

pub use ratio::Ratio;
pub use operator_aliases::{Num, Den};
use operator_aliases::ReducedRatio;

use typenum::{Integer, NonZero};

/// A type representing a rational number whose value is known at compile time.
///
/// All implementors of `Rational` must ensure the following:
///
/// * `Self::Den` is a positive integer.
///
/// * `Self::Num / Self::Den` is a [reduced fraction][reduced]. In other words, the greatest common
///   divisor of `Self::Num` and `Self::Den` is `1`.
///
/// [reduced]: http://mathworld.wolfram.com/ReducedFraction.html
pub trait Rational {
    /// The numerator of the rational number.
    type Num: Integer;

    /// The denominator of the rational number.
    ///
    /// Must be positive.
    type Den: Integer + NonZero;
}

/// Creates a [`Ratio`] from two type-level integers.
///
/// [`Ratio`]: ./struct.Ratio.html
///
/// # Example
///
/// ```rust
///
/// # #[macro_use] extern crate typenum_ratio;
/// extern crate typenum;
///
/// use typenum::consts::*;
///
/// assert_eq!(rat!(P3/P4) + rat!(P3/P4), rat!(P3/P2));
/// ```
#[macro_export]
macro_rules! rat {
    ($n:ident / $d:ident) => {
        $crate::Ratio::new($n::new(), $d::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::{consts::*, operator_aliases::*};

    #[test]
    fn reduce() {
        assert_eq!(rat!(P1/P3), rat!(P3/P9));
        assert_eq!(rat!(P1/P1), rat!(P2/P2));

        assert_eq!(rat!(N1/N1), rat!(P1/P1));
        assert_eq!(rat!(N1/P1), rat!(P1/N1));
    }

    #[test]
    fn neg() {
        assert_eq!(-rat!(P1/P2), rat!(N1/P2));
        assert_eq!(-rat!(P1/N2), rat!(P1/P2));
        assert_eq!(-rat!(N1/P2), rat!(P1/P2));
        assert_eq!(-rat!(N1/N2), rat!(N1/P2));
    }

    #[test]
    fn cmp() {
        assert!(rat!(P2/P3) > rat!(P3/P5));
        assert!(rat!(N1/N2) > rat!(P1/N2));
    }

    #[test]
    fn add() {
        assert_eq!(rat!(P1/P3) + rat!(P1/P2), rat!(P5/P6));
        assert_eq!(rat!(P3/P5) + rat!(P2/P3), rat!(P19/P15));
        assert_eq!(rat!(P3/P4) + rat!(P3/P4), rat!(P3/P2));

        assert_eq!(rat!(P2/P3) + P2::new(), rat!(P8/P3));
    }

    #[test]
    fn sub() {
        assert_eq!(rat!(P1/P2) - rat!(P1/P3), rat!(P1/P6));

        assert_eq!(rat!(P2/P3) - P2::new(), rat!(N4/P3));
    }

    #[test]
    fn mul() {
        assert_eq!(rat!(P1/P2) * rat!(P1/P3), rat!(P1/P6));
        assert_eq!(rat!(P4/P5) * rat!(P2/P3), rat!(P8/P15));
        assert_eq!(rat!(P2/P3) * rat!(P3/P2), rat!(P1/P1));

        assert_eq!(rat!(P2/P3) * P2::new(), rat!(P4/P3));
    }

    #[test]
    fn div() {
        assert_eq!(rat!(P1/P2) / rat!(P1/P3), rat!(P3/P2));
        assert_eq!(rat!(P4/P5) / rat!(P2/P3), rat!(P6/P5));

        assert_eq!(rat!(P2/P3) / P2::new(), rat!(P1/P3));
    }

    #[test]
    fn rem() {
        assert_eq!(rat!(P9/P8) % rat!(P3/P16), rat!(Z0/P1));
        assert_eq!(rat!(P3/P8) % rat!(P1/P4),  rat!(P1/P8));
    }

    #[test]
    fn gcd() {
        assert_eq!(Gcf::<Ratio<P9, P8>, Ratio<P3, P16>>::default(), rat!(P3/P16));
        assert_eq!(Gcf::<Ratio<P3, P7>, Ratio<P12, P22>>::default(), rat!(P3/P77));
        assert_eq!(Gcf::<Ratio<P13, P6>, Ratio<P3, P4>>::default(), rat!(P1/P12));
    }
}
