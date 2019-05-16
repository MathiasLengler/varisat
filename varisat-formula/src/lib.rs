//! Basic formula data types used by the Varisat SAT solver.

/// Shortcut for tests
#[cfg(any(test, feature = "internal-test-macros"))]
#[doc(hidden)]
#[macro_export]
macro_rules! lit {
    ($x:expr) => {
        $crate::lit::Lit::from_dimacs($x)
    };
}

/// Shortcut for tests
#[cfg(any(test, feature = "internal-test-macros"))]
#[doc(hidden)]
#[macro_export]
macro_rules! var {
    ($x:expr) => {
        $crate::lit::Var::from_dimacs($x)
    };
}

/// Shortcut for tests
#[cfg(any(test, feature = "internal-test-macros"))]
#[doc(hidden)]
#[macro_export]
macro_rules! lits {
    ( $( $x:expr ),* ) => { [ $( $crate::lit!( $x ) ),* ] };
    ( $( $x:expr ),* , ) => { $crate::lits! [ $( $ x),* ] };
}

/// Shortcut for tests
#[cfg(any(test, feature = "internal-test-macros"))]
#[doc(hidden)]
#[macro_export]
macro_rules! vars {
    ( $( $x:expr ),* ) => { [ $( $crate::var!( $x ) ),* ] };
    ( $( $x:expr ),* , ) => { $crate::vars! [ $( $ x),* ] };
}

/// Shortcut for tests
#[cfg(any(test, feature = "internal-test-macros"))]
#[doc(hidden)]
#[macro_export]
macro_rules! cnf {
    ( $( $( $x:expr ),* );* ; ) => { [ $( &[ $( $crate::lit!( $x ) ),* ] as &[$crate::Lit] ),* ] };
}

/// Shortcut for tests
#[cfg(any(test, feature = "internal-test-macros"))]
#[doc(hidden)]
#[macro_export]
macro_rules! cnf_formula {
    ( $( $t:tt )* ) => { $crate::cnf::CnfFormula::from($crate::cnf![ $($t)* ].iter().cloned()) };
}

pub mod cnf;
pub mod lit;

pub use cnf::CnfFormula;
pub use lit::{Lit, Var};
