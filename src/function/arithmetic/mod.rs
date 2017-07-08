use super::{ Function, Error };

/// Addition
pub mod add;
pub use self::add::*;
/// Division
pub mod div;
pub use self::div::*;
/// Greater than
pub mod greater;
pub use self::greater::*;
/// Maximum
pub mod max;
pub use self::max::*;
/// Minimum
pub mod min;
pub use self::min::*;
/// Modulo
pub mod mod_;
pub use self::mod_::*;
/// Multiplication
pub mod mul;
pub use self::mul::*;
/// Multiplication then division
pub mod muldiv;
pub use self::muldiv::*;
/// Substraction
pub mod sub;
pub use self::sub::*;