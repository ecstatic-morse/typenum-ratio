//! Each [SI prefix][si] expressed as a [Ratio].
//!
//! [Ratio]: ./struct.Ratio.html
//! [si]: https://en.wikipedia.org/wiki/Metric_prefix
//!
//! | Prefix    | Symbol | Value |                                                                |
//! |-----------|--------|-------|----------------------------------------------------------------|
//! | [`Exa`]   | `E`    | 10¹⁸  |                                                                |
//! | [`Peta`]  | `P`    | 10¹⁵  |                                                                |
//! | [`Tera`]  | `T`    | 10¹²  |                                                                |
//! | [`Giga`]  | `G`    | 10⁹   |                                                                |
//! | [`Mega`]  | `M`    | 10⁶   |                                                                |
//! | [`Kilo`]  | `k`    | 10³   |                                                                |
//! | [`Hecta`] | `h`    | 10²   |                                                                |
//! | [`Deca`]  | `da`   | 10¹   |                                                                |
//! | [`Deci`]  | `d`    | 10⁻¹  |                                                                |
//! | [`Centi`] | `c`    | 10⁻²  |                                                                |
//! | [`Milli`] | `m`    | 10⁻³  |                                                                |
//! | [`Micro`] | `μ`    | 10⁻⁶  |                                                                |
//! | [`Nano`]  | `n`    | 10⁻⁹  |                                                                |
//! | [`Pico`]  | `p`    | 10⁻¹² |                                                                |
//! | [`Femto`] | `f`    | 10⁻¹⁵ |                                                                |
//! | [`Atto`]  | `a`    | 10⁻¹⁸ |                                                                |
//!
//! [`Exa`]: ./type.Exa.html
//! [`Peta`]: ./type.Peta.html
//! [`Tera`]: ./type.Tera.html
//! [`Giga`]: ./type.Giga.html
//! [`Mega`]: ./type.Mega.html
//! [`Kilo`]: ./type.Kilo.html
//! [`Hecta`]: ./type.Hecta.html
//! [`Deca`]: ./type.Deca.html
//! [`Deci`]: ./type.Deci.html
//! [`Centi`]: ./type.Centi.html
//! [`Milli`]: ./type.Milli.html
//! [`Micro`]: ./type.Micro.html
//! [`Nano`]: ./type.Nano.html
//! [`Pico`]: ./type.Pico.html
//! [`Femto`]: ./type.Femto.html
//! [`Atto`]: ./type.Atto.html

use typenum::consts::*;

use super::Ratio;

/// 10¹⁸
pub type Exa = Ratio<P1000000000000000000>;
/// 10¹⁵
pub type Peta = Ratio<P1000000000000000>;
/// 10¹²
pub type Tera = Ratio<P1000000000000>;
/// 10⁹
pub type Giga = Ratio<P1000000000>;
/// 10⁶
pub type Mega = Ratio<P1000000>;
/// 10³
pub type Kilo = Ratio<P1000>;
/// 10²
pub type Hecta = Ratio<P100>;
/// 10¹
pub type Deca = Ratio<P10>;

/// 10⁻¹
pub type Deci = Ratio<P1, P10>;
/// 10⁻²
pub type Centi = Ratio<P1, P100>;
/// 10⁻³
pub type Milli = Ratio<P1, P1000>;
/// 10⁻⁶
pub type Micro = Ratio<P1, P1000000>;
/// 10⁻⁹
pub type Nano = Ratio<P1, P1000000000>;
/// 10⁻¹²
pub type Pico = Ratio<P1, P1000000000000>;
/// 10⁻¹⁵
pub type Femto = Ratio<P1, P1000000000000000>;
/// 10⁻¹⁸
pub type Atto = Ratio<P1, P1000000000000000000>;
