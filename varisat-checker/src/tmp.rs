//! Temporary data.
use varisat_formula::Lit;

#[derive(Default, Clone)]
pub struct TmpData {
    /// Temporary storage for literals.
    pub tmp: Vec<Lit>,
}
