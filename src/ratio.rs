use std::{cmp, fmt};
use std::marker::PhantomData;
use std::ops::*;

use typenum::{
    Bit,
    Integer,
    NonZero,
    Ord,
    P1, Z0,
    NInt, PInt,
    Unsigned,
    operator_aliases::*,
    type_operators::*,
};

use super::{Rational, Num, Den, ReducedRatio};

/// A rational number whose value is known at compile time.
///
/// This type implements [`Rational`] if *all* of the following conditions are met:
///
/// * `N` and `D` are [`typenum::Integer`]s.
/// * `D` is [`NonZero`].
///
/// `numerator   = sign(D) * N / gcd(N, D)`
/// `denominator = abs(D) / gcd(N, D)`
///
/// Most uses of `Ratio` will be as a type parameter. For the rare cases when an instance of a
/// `Ratio` type is needed, use the [`rat!`] macro.
///
/// # Example
///
/// ```
/// extern crate typenum;
/// extern crate typenum_ratio;
///
/// use typenum::consts::*;
/// use typenum_ratio::Ratio;
///
/// assert_eq!(Ratio::<N1, P3>::default() + Ratio::<P4, N6>::default(),
///            Ratio::<N1, P1>::default());
/// ```
///
/// [`Rational`]: ./trait.Rational.html
/// [`NonZero`]: https://docs.rs/typenum/1.10.0/typenum/marker_traits/trait.NonZero.html
/// [`rat!`]: ./macro.rat.html
/// [`typenum::Integer`]: https://docs.rs/typenum/1.10.0/typenum/marker_traits/trait.Integer.html
/// [reduced]: http://mathworld.wolfram.com/ReducedFraction.html
pub struct Ratio<N, D = P1>(PhantomData<(N, D)>);

impl<N, D> Ratio<N, D> {
    fn _new() -> Self {
        Ratio(PhantomData)
    }
}

impl<N, D> Ratio<N, D>
    where N: Integer,
          D: Integer + NonZero,
{
    /// Constructs a new `Ratio` with the given numerator and denominator.
    pub fn new(_num: N, _den: D) -> Self {
        Self::_new()
    }
}

impl<N, D> Default for Ratio<N, D> {
    fn default() -> Self {
        Self::_new()
    }
}

/// 0/D => 0/1
impl<D> Rational for Ratio<Z0, D>
    where D: Integer,
{
    type Num = Z0;
    type Den = P1;
}

/// N/D => N/D
impl<N, D> Rational for Ratio<PInt<N>, PInt<D>>
    where N: Unsigned + NonZero + Div<Gcf<N, D>> + Gcd<D>,
          D: Unsigned + NonZero + Div<Gcf<N, D>>,
          Quot<N, Gcf<N, D>>: Unsigned + NonZero,
          Quot<D, Gcf<N, D>>: Unsigned + NonZero,
{
    type Num = PInt<Quot<N, Gcf<N, D>>>;
    type Den = PInt<Quot<D, Gcf<N, D>>>;
}

/// N/-D => -N/D
impl<N, D> Rational for Ratio<PInt<N>, NInt<D>>
    where N: Unsigned + NonZero + Div<Gcf<N, D>> + Gcd<D>,
          D: Unsigned + NonZero + Div<Gcf<N, D>>,
          Quot<N, Gcf<N, D>>: Unsigned + NonZero,
          Quot<D, Gcf<N, D>>: Unsigned + NonZero,
{
    type Num = NInt<Quot<N, Gcf<N, D>>>;
    type Den = PInt<Quot<D, Gcf<N, D>>>;
}

/// -N/D => -N/D
impl<N, D> Rational for Ratio<NInt<N>, PInt<D>>
    where N: Unsigned + NonZero + Div<Gcf<N, D>> + Gcd<D>,
          D: Unsigned + NonZero + Div<Gcf<N, D>>,
          Quot<N, Gcf<N, D>>: Unsigned + NonZero,
          Quot<D, Gcf<N, D>>: Unsigned + NonZero,
{
    type Num = NInt<Quot<N, Gcf<N, D>>>;
    type Den = PInt<Quot<D, Gcf<N, D>>>;
}

/// -N/-D => N/D
impl<N, D> Rational for Ratio<NInt<N>, NInt<D>>
    where N: Unsigned + NonZero + Div<Gcf<N, D>> + Gcd<D>,
          D: Unsigned + NonZero + Div<Gcf<N, D>>,
          Quot<N, Gcf<N, D>>: Unsigned + NonZero,
          Quot<D, Gcf<N, D>>: Unsigned + NonZero,
{
    type Num = PInt<Quot<N, Gcf<N, D>>>;
    type Den = PInt<Quot<D, Gcf<N, D>>>;
}

/// N1/D1 == N2/D2 for two reduced fractions iff N1 == N2 && N2 == D2
impl<N1, D1, N2, D2> PartialEq<Ratio<N2, D2>> for Ratio<N1, D1>
    where Ratio<N1, D1>: Rational,
          Ratio<N2, D2>: Rational,
          Num<N1, D1>: IsEqual<Num<N2, D2>>,
          Den<N1, D1>: IsEqual<Den<N2, D2>>,
{
    fn eq(&self, _: &Ratio<N2, D2>) -> bool {
        Eq::<Num<N1, D1>, Num<N2, D2>>::to_bool()
            && Eq::<Den<N1, D1>, Den<N2, D2>>::to_bool()
    }
}

/// Equality is reflexive for a given `Ratio` since all instances have the same semantic value.
impl<N, D> cmp::Eq for Ratio<N, D>
    where Ratio<N, D>: Rational + PartialEq,
{}

/// N1/D1 < N2/D2 iff N1*D2 < N2*D1
impl<N1, D1, N2, D2> PartialOrd<Ratio<N2, D2>> for Ratio<N1, D1>
    where Ratio<N1, D1>: Rational + PartialEq<Ratio<N2, D2>>,
          Ratio<N2, D2>: Rational,
          Num<N1, D1>: Mul<Den<N2, D2>>,
          Num<N2, D2>: Mul<Den<N1, D1>>,
          Prod<Num<N1, D1>, Den<N2, D2>>: Cmp<Prod<Num<N2, D2>, Den<N1, D1>>>,
          Compare<Prod<Num<N1, D1>, Den<N2, D2>>, Prod<Num<N2, D2>, Den<N1, D1>>>: Ord,
{
    fn partial_cmp(&self, _: &Ratio<N2, D2>) -> Option<cmp::Ordering> {
        Compare::<
            Prod<Num<N1, D1>, Den<N2, D2>>,
            Prod<Num<N2, D2>, Den<N1, D1>>
        >::to_ordering().into()
    }
}

impl<N, D> cmp::Ord for Ratio<N, D>
    where Ratio<N, D>: Rational + PartialOrd,
{
    fn cmp(&self, _: &Self) -> cmp::Ordering {
        cmp::Ordering::Equal
    }
}

/// (N1/D1) + (N2/D2) = (N1*D2 + N2*D1)/(D1*D2)
impl<N1, D1, N2, D2> Add<Ratio<N2, D2>> for Ratio<N1, D1>
    where N1: Mul<D2>,
          N2: Mul<D1>,
          D1: Mul<D2>,
          Prod<N1, D2>: Add<Prod<N2, D1>>,
          Ratio<Sum<Prod<N1, D2>, Prod<N2, D1>>, Prod<D1, D2>>: Rational,

{
    type Output =
        ReducedRatio<
            Sum<Prod<N1, D2>, Prod<N2, D1>>,
            Prod<D1, D2>
        >;

    fn add(self, _: Ratio<N2, D2>) -> Self::Output {
        Self::Output::_new()
    }
}

/// (N/D) + I = (N+D*I)/D
impl<N, D, I> Add<I> for Ratio<N, D>
    where I: Integer,
          D: Mul<I>,
          N: Add<Prod<D, I>>,
          Ratio<Sum<N, Prod<D, I>>, D>: Rational,
{
    // No need to reduce.
    type Output = Ratio<Sum<N, Prod<D, I>>, D>;

    fn add(self, _: I) -> Self::Output {
        Self::Output::_new()
    }
}

/// (N1/D1) - (N2/D2) = (N1*D2 - N2*D1)/(D1*D2)
impl<N1, D1, N2, D2> Sub<Ratio<N2, D2>> for Ratio<N1, D1>
    where N1: Mul<D2>,
          N2: Mul<D1>,
          D1: Mul<D2>,
          Prod<N1, D2>: Sub<Prod<N2, D1>>,
          Ratio<Diff<Prod<N1, D2>, Prod<N2, D1>>, Prod<D1, D2>>: Rational,

{
    type Output =
        ReducedRatio<
            Diff<Prod<N1, D2>, Prod<N2, D1>>,
            Prod<D1, D2>
        >;

    fn sub(self, _: Ratio<N2, D2>) -> Self::Output {
        Self::Output::_new()
    }
}

/// (N/D) - I = (N-D*I)/D
impl<N, D, I> Sub<I> for Ratio<N, D>
    where I: Integer,
          D: Mul<I>,
          N: Sub<Prod<D, I>>,
          Ratio<Diff<N, Prod<D, I>>, D>: Rational,
{
    // No need to reduce.
    type Output = Ratio<Diff<N, Prod<D, I>>, D>;

    fn sub(self, _: I) -> Self::Output {
        Self::Output::_new()
    }
}

/// (N1/D1) * (N2/D2) = (N1*N2)/(D1*D2)
impl<N1, D1, N2, D2> Mul<Ratio<N2, D2>> for Ratio<N1, D1>
    where N1: Mul<N2>,
          D1: Mul<D2>,
          Ratio<Prod<N1, N2>, Prod<D1, D2>>: Rational,
{
    type Output = ReducedRatio<Prod<N1, N2>, Prod<D1, D2>>;

    fn mul(self, _: Ratio<N2, D2>) -> Self::Output {
        Self::Output::_new()
    }
}

/// (N/D) * I = (N*I)/D
impl<N, D, I> Mul<I> for Ratio<N, D>
    where I: Integer,
          N: Mul<I>,
          Ratio<Prod<N, I>, D>: Rational,
{
    type Output = ReducedRatio<Prod<N, I>, D>;

    fn mul(self, _: I) -> Self::Output {
        Self::Output::_new()
    }
}

/// (N1/D1) / (N2/D2) = (N1*D2)/(D1*N2)
impl<N1, D1, N2, D2> Div<Ratio<N2, D2>> for Ratio<N1, D1>
    where N1: Mul<D2>,
          D1: Mul<N2>,
          Ratio<Prod<N1, D2>, Prod<D1, N2>>: Rational,
{
    type Output = ReducedRatio<Prod<N1, D2>, Prod<D1, N2>>;

    fn div(self, _: Ratio<N2, D2>) -> Self::Output {
        Self::Output::_new()
    }
}

/// (N/D) / I = N/(D*I)
impl<N, D, I> Div<I> for Ratio<N, D>
    where I: Integer,
          D: Mul<I>,
          Prod<D, I>: Integer,
{
    // No need to reduce.
    type Output = Ratio<N, Prod<D, I>>;

    fn div(self, _: I) -> Self::Output {
        Self::Output::_new()
    }
}

// TODO: Can't implement e.g `Div<Ratio<N, D>> for PInt<U>` due to coherence issues.
// Maybe add a feature to `typenum`?

impl<N, D> fmt::Debug for Ratio<N, D>
    where Ratio<N, D>: Rational,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl<N, D> fmt::Display for Ratio<N, D>
    where Ratio<N, D>: Rational,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", Num::<N, D>::to_i64(), Den::<N, D>::to_i64())
    }
}
