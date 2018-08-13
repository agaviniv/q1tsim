extern crate num_complex;

use cmatrix;
use gates;

/// The `T` gate
///
/// The `T` gate rotates the state over π/4 radians around the `z` axis of
/// the Bloch sphere. It is the square root of the `S` gate.
pub struct T
{
}

impl T
{
    /// Create a new `T` gate.
    pub fn new() -> Self
    {
        T { }
    }
}

impl gates::Gate for T
{
    fn cost(&self) -> f64
    {
        gates::U1::cost()
    }

    fn description(&self) -> &str
    {
        "T"
    }

    fn nr_affected_bits(&self) -> usize
    {
        1
    }

    fn matrix(&self) -> cmatrix::CMatrix
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let x = cmatrix::COMPLEX_HSQRT2;
        let i = cmatrix::COMPLEX_I;
        array![[o, z], [z, x+x*i]]
    }

    fn apply_slice(&self, state: &mut cmatrix::CVecSliceMut)
    {
        assert!(state.len() % 2 == 0, "Number of rows is not even.");

        let n = state.len() / 2;
        let mut slice = state.slice_mut(s![n..]);
        slice *= num_complex::Complex::from_polar(&1.0, &::std::f64::consts::FRAC_PI_4);
    }

    fn apply_mat_slice(&self, state: &mut cmatrix::CMatSliceMut)
    {
        assert!(state.rows() % 2 == 0, "Number of rows is not even.");

        let n = state.rows() / 2;
        let mut slice = state.slice_mut(s![n.., ..]);
        slice *= num_complex::Complex::from_polar(&1.0, &::std::f64::consts::FRAC_PI_4);
    }
}

/// Conjugate of `T` gate
///
/// The `T`<sup>`†`</sup> gate rotates the state over -π/4 radians around the
/// `z` axis of the Bloch sphere. It is the conjugate of the `T` gate.
pub struct Tdg
{
}

impl Tdg
{
    /// Create a new `T`<sup>`†`</sup> gate.
    pub fn new() -> Self
    {
        Tdg { }
    }
}

impl gates::Gate for Tdg
{
    fn cost(&self) -> f64
    {
        gates::U1::cost()
    }

    fn description(&self) -> &str
    {
        "T†"
    }

    fn nr_affected_bits(&self) -> usize
    {
        1
    }

    fn matrix(&self) -> cmatrix::CMatrix
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let x = cmatrix::COMPLEX_HSQRT2;
        let i = cmatrix::COMPLEX_I;
        array![[o, z], [z, x-x*i]]
    }

    fn apply_slice(&self, state: &mut cmatrix::CVecSliceMut)
    {
        assert!(state.len() % 2 == 0, "Number of rows is not even.");

        let n = state.len() / 2;
        let mut slice = state.slice_mut(s![n..]);
        slice *= num_complex::Complex::from_polar(&1.0, &-::std::f64::consts::FRAC_PI_4);
    }

    fn apply_mat_slice(&self, state: &mut cmatrix::CMatSliceMut)
    {
        assert!(state.rows() % 2 == 0, "Number of rows is not even.");

        let n = state.rows() / 2;
        let mut slice = state.slice_mut(s![n.., ..]);
        slice *= num_complex::Complex::from_polar(&1.0, &-::std::f64::consts::FRAC_PI_4);
    }
}

#[cfg(test)]
mod tests
{
    extern crate num_complex;

    use super::{T, Tdg};
    use gates::Gate;
    use cmatrix;

    #[test]
    fn test_description()
    {
        let gate = T::new();
        assert_eq!(gate.description(), "T");
        let gate = Tdg::new();
        assert_eq!(gate.description(), "T†");
    }

    #[test]
    fn test_matrix()
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let t = num_complex::Complex::from_polar(&1.0, &::std::f64::consts::FRAC_PI_4);

        let gate = T::new();
        assert_complex_matrix_eq!(gate.matrix(), array![[o, z], [z, t]]);

        let gate = Tdg::new();
        assert_complex_matrix_eq!(gate.matrix(), array![[o, z], [z, t.conj()]]);
    }

    #[test]
    fn test_apply_mat()
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let i = cmatrix::COMPLEX_I;
        let h = 0.5 * o;
        let x = cmatrix::COMPLEX_HSQRT2;
        let t = num_complex::Complex::from_polar(&1.0, &::std::f64::consts::FRAC_PI_4);
        let td = t.conj();

        let mut state = array![
            [o, z, x,  h, z],
            [z, o, z, -h, z],
            [z, z, x,  h, z],
            [z, z, z, -h, o]
        ];
        T::new().apply_mat(&mut state);
        assert_complex_matrix_eq!(&state, &array![
            [o, z,   x,    h, z],
            [z, o,   z,   -h, z],
            [z, z, t*x,  t*h, z],
            [z, z,   z, -t*h, t]
        ]);

        let mut state = array![
            [o, z, x,  h, z],
            [z, o, z, -h, z],
            [z, z, x,  h, z],
            [z, z, z, -h, o]
        ];
        Tdg::new().apply_mat(&mut state);
        assert_complex_matrix_eq!(&state, &array![
            [o, z,    x,     h,  z],
            [z, o,    z,    -h,  z],
            [z, z, td*x,  td*h,  z],
            [z, z,    z, -td*h, td]
        ]);
    }
}