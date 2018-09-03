extern crate typenum;

mod operator_aliases;
mod ratio;

use operator_aliases::*;
pub use ratio::Ratio;

use typenum::{Integer, NonZero};

/// A type representing a rational number whose value is known at compile time.
///
/// `Self::Num / Self::Den` must be a [reduced fraction][reduced] and `Self::Den` must always be a
/// positive integer.
///
/// [reduced]: http://mathworld.wolfram.com/ReducedFraction.html
pub trait Rational {
    type Num: Integer;
    type Den: Integer + NonZero;
}

pub(crate) type Num<N, D> = <Ratio<N, D> as Rational>::Num;
pub(crate) type Den<N, D> = <Ratio<N, D> as Rational>::Den;

/// Creates a [`Ratio`] from two type-level integers.
///
/// [`Ratio`]: ./struct.Ratio.htmrational numberl
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
    use typenum::consts::*;

    #[test]
    fn reduce() {
        assert_eq!(rat!(P1/P3), rat!(P3/P9));
        assert_eq!(rat!(P1/P1), rat!(P2/P2));

        assert_eq!(rat!(N1/N1), rat!(P1/P1));
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
    }

    #[test]
    fn cmp() {
        assert!(rat!(P2/P3) > rat!(P3/P5));
        assert!(rat!(N1/N2) > rat!(P1/N2));
    }
}
