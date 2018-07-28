extern crate num_complex;

use cmatrix;
use gates;

/// The Pauli X gate.
///
/// The X, or NOT, gate rotates the state over π radians around the `x` axis of
/// the Bloch sphere, i.e. it swaps the |0〉 and |1〉 components of the qubit.
pub struct X
{
}

impl X
{
    /// Create a new Pauli X gate.
    pub fn new() -> Self
    {
        X { }
    }

    pub fn transform(state: &mut cmatrix::CVecSliceMut)
    {
        assert!(state.len() % 2 == 0, "Number of rows is not even.");

        let n = state.len() / 2;
        for i in 0..n
        {
            state.swap(i, i+n);
        }
    }
}

impl gates::Gate for X
{
    fn description(&self) -> &str
    {
        "X"
    }

    fn nr_affected_bits(&self) -> usize
    {
        1
    }

    fn matrix(&self) -> cmatrix::CMatrix
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        array![[z, o], [o, z]]
    }

    fn apply_slice(&self, state: &mut cmatrix::CVecSliceMut)
    {
        Self::transform(state);
    }
}

#[cfg(test)]
mod tests
{
    use gates::{gate_test, Gate, X};
    use cmatrix;

    #[test]
    fn test_description()
    {
        let x = X::new();
        assert_eq!(x.description(), "X");
    }

    #[test]
    fn test_matrix()
    {
        let x = X::new();
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        assert_complex_matrix_eq!(x.matrix(), array![[z, o], [o, z]]);
    }

    #[test]
    fn test_apply()
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let x = cmatrix::COMPLEX_HSQRT2;
        let mut state = array![[o, z, x, x], [z, o, x, -x]];
        let result = array![[z, o, x, -x],[o, z, x, x]];
        gate_test(X::new(), &mut state, &result);
    }
}