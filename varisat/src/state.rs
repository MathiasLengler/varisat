//! Miscellaneous solver state.
use crate::solver::SolverError;

/// Satisfiability state.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SatState {
    Unknown,
    Sat,
    Unsat,
    UnsatUnderAssumptions,
}

impl Default for SatState {
    fn default() -> SatState {
        SatState::Unknown
    }
}

/// Miscellaneous solver state.
///
/// Anything larger or any larger group of related state variables should be moved into a separate
/// part of [`Context`](crate::context::Context).
pub struct SolverState {
    pub sat_state: SatState,
    pub formula_is_empty: bool,
    /// Whether solve was called at least once.
    pub solver_invoked: bool,
    pub state_is_invalid: bool,
    pub solver_error: Option<SolverError>,
}

impl Clone for SolverState {
    fn clone(&self) -> Self {
        Self {
            sat_state: self.sat_state.clone(),
            formula_is_empty: self.formula_is_empty.clone(),
            solver_invoked: self.solver_invoked.clone(),
            state_is_invalid: self.state_is_invalid.clone(),
            solver_error: None,
        }
    }
}

impl Default for SolverState {
    fn default() -> SolverState {
        SolverState {
            sat_state: SatState::Unknown,
            formula_is_empty: true,
            solver_invoked: false,
            state_is_invalid: false,
            solver_error: None,
        }
    }
}
