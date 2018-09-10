//! Convenient aliases for operations on rational numbers.

use super::{Ratio, Rational};

/// Reduces `N/D` and extracts the numerator.
///
/// # Examples
///
/// ```
/// extern crate typenum;
/// extern crate typenum_ratio;
///
/// use typenum::{Integer, consts::*};
/// use typenum_ratio::operator_aliases::*;
///
/// assert_eq!(Num::<P2, P4>::to_i32(), 1);
/// assert_eq!(Den::<P2, P4>::to_i32(), 2);
/// ```
pub type Num<N, D> = <Ratio<N, D> as Rational>::Num;

/// Reduces `N/D` and extracts the denominator.
///
/// # Examples
///
/// ```
/// extern crate typenum;
/// extern crate typenum_ratio;
///
/// use typenum::{Integer, consts::*};
/// use typenum_ratio::operator_aliases::*;
///
/// assert_eq!(Num::<P2, P4>::to_i32(), 1);
/// assert_eq!(Den::<P2, P4>::to_i32(), 2);
/// ```
pub type Den<N, D> = <Ratio<N, D> as Rational>::Den;

pub(crate) type ReducedRatio<N, D> = Ratio<Num<N, D>, Den<N, D>>;
