use super::{Ratio, Rational};

/// Alias for `Rational::Num`: `Num<N, D> = <Ratio<N, D> as Rational>::Num`.
pub type Num<N, D> = <Ratio<N, D> as Rational>::Num;

/// Alias for `Rational::Den`: `Num<N, D> = <Ratio<N, D> as Rational>::Den`.
pub type Den<N, D> = <Ratio<N, D> as Rational>::Den;

pub type ReducedRatio<N, D> = Ratio<Num<N, D>, Den<N, D>>;
