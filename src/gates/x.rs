extern crate num_complex;
extern crate rulinalg;

use cmatrix;
use gates;

use rulinalg::matrix::{BaseMatrix, BaseMatrixMut};

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
        cmatrix::CMatrix::new(2, 2, vec![z, o, o, z])
    }

    fn apply_slice(&self, state: &mut rulinalg::matrix::MatrixSliceMut<num_complex::Complex64>)
    {
        assert!(state.rows() % 2 == 0, "Number of rows is not even.");

        let n = state.rows() / 2;
        for i in 0..n
        {
            state.swap_rows(i, i+n);
        }
    }
}

#[cfg(test)]
mod tests
{
    use gates::Gate;
    use gates::X;
    use cmatrix;
    use rulinalg::matrix::BaseMatrix;

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
        assert_complex_matrix_eq!(x.matrix().as_ref(), matrix![z, o; o, z]);
    }

    #[test]
    fn test_apply()
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let x = cmatrix::COMPLEX_HSQRT2;
        let mut state = cmatrix::CMatrix::new(2, 4, vec![o, z, x, x, z, o, x, -x]);

        X::new().apply(state.as_mut());
        assert_complex_matrix_eq!(state.as_ref(), matrix![z, o, x, -x; o, z, x, x]);
    }
}
