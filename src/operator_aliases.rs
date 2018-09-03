use super::{Ratio, Num, Den};

pub type ReducedRatio<N, D> = Ratio<Num<N, D>, Den<N, D>>;
