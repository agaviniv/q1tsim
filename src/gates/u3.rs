// Copyright 2019 Q1t BV
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::gates::Gate;

/// U<sub>3</sub> gate.
///
/// The `U`<sub>`3`</sub>`(θ, ϕ, λ)` gate is the univeral single-qubit
/// transformation, all unary gates can be written in terms of `U`<sub>`3`</sub>.
/// It transforms a qubit by the matrix
/// ```text
/// ┌                                     ┐
/// │       cos(θ/2)      -exp(iλ)sin(θ/2)│
/// │                                     │
/// │exp(iϕ)sin(θ/2)   exp(i(λ+ϕ))cos(θ/2)│
/// └                                     ┘
/// ```
#[derive(Clone)]
pub struct U3
{
    theta: crate::gates::Parameter,
    phi: crate::gates::Parameter,
    lambda: crate::gates::Parameter,
    desc: String
}

impl U3
{
    /// Create a new `U`<sub>`3`</sub> gate.
    pub fn new<Tt, Tp, Tl>(theta: Tt, phi: Tp, lambda: Tl) -> Self
    where crate::gates::Parameter: From<Tt> + From<Tp> + From<Tl>
    {
        let ptheta = crate::gates::Parameter::from(theta);
        let pphi = crate::gates::Parameter::from(phi);
        let plambda = crate::gates::Parameter::from(lambda);
        let desc = format!("U3({:.4}, {:.4}, {:.4})", ptheta, pphi, plambda);
        U3 { theta: ptheta, phi: pphi, lambda: plambda, desc: desc }
    }

    pub fn cost() -> f64
    {
        201.0
    }
}

impl crate::gates::Gate for U3
{
    fn cost(&self) -> f64
    {
        Self::cost()
    }

    fn description(&self) -> &str
    {
        &self.desc
    }

    fn nr_affected_bits(&self) -> usize
    {
        1
    }

    fn matrix(&self) -> crate::cmatrix::CMatrix
    {
        let htheta = 0.5 * self.theta.value();
        let phi = self.phi.value();
        let lambda = self.lambda.value();
        let (c, s) = (htheta.cos(), htheta.sin());
        array![[ num_complex::Complex::new(c, 0.0),
                -num_complex::Complex::from_polar(&s, &lambda)],
               [ num_complex::Complex::from_polar(&s, &phi),
                 num_complex::Complex::from_polar(&c, &(phi+lambda))]]
    }
}

impl crate::export::OpenQasm for U3
{
    fn open_qasm(&self, bit_names: &[String], bits: &[usize])
        -> crate::error::Result<String>
    {
        Ok(format!("u3({}, {}, {}) {}",
            self.theta, self.phi, self.lambda, bit_names[bits[0]]))
    }
}

impl crate::export::CQasm for U3
{
    fn c_qasm(&self, bit_names: &[String], bits: &[usize])
        -> crate::error::Result<String>
    {
        let name = &bit_names[bits[0]];
        Ok(format!("rz {}, {}\nry {}, {}\n; rz {} {}", name, self.lambda,
            name, self.theta, name, self.phi))
    }
}

impl crate::export::Latex for U3
{
    fn latex(&self, bits: &[usize], state: &mut crate::export::LatexExportState)
        -> crate::error::Result<()>
    {
        self.check_nr_bits(bits.len())?;
        let contents = format!("U_3({:.4}, {:.4}, {:.4})", self.theta, self.phi, self.lambda);
        state.add_block_gate(bits, &contents)
    }
}

// Use default implementation for Square (i.e., fail)
impl crate::arithmetic::Square for U3
{
    type SqType = Self;
}

#[cfg(test)]
mod tests
{
    use super::U3;
    use crate::arithmetic::Square;
    use crate::gates::{gate_test, Gate};
    use crate::export::{Latex, LatexExportState, OpenQasm, CQasm};
    use num_complex::Complex;

    #[test]
    fn test_description()
    {
        let gate = U3::new(0.17, ::std::f64::consts::FRAC_PI_4, ::std::f64::consts::LN_2);
        assert_eq!(gate.description(), "U3(0.1700, 0.7854, 0.6931)");
    }

    #[test]
    fn test_cost()
    {
        let gate = U3::new(0.17, ::std::f64::consts::FRAC_PI_4, ::std::f64::consts::LN_2);
        assert_eq!(gate.cost(), 201.0);
    }

    #[test]
    fn test_matrix()
    {
        let gate = U3::new(0.32, ::std::f64::consts::FRAC_PI_4, ::std::f64::consts::LN_2);
        assert_complex_matrix_eq!(gate.matrix(), array![
            [Complex::new(0.9872272833756269,                0.0), Complex::new( -0.1225537622232209, -0.1017981646382380)],
            [Complex::new(0.1126549842634128, 0.1126549842634128), Complex::new(0.09094356700076842,  0.9830294892130130)]
        ]);
    }

    #[test]
    fn test_apply()
    {
        let z = crate::cmatrix::COMPLEX_ZERO;
        let o = crate::cmatrix::COMPLEX_ONE;
        let x = crate::cmatrix::COMPLEX_HSQRT2;
        let i = crate::cmatrix::COMPLEX_I;
        let mut state = array![[x, x], [z, -x], [x, z], [z, z]];
        let result = array![
            [  -o-i,       -o],
            [     z,        o],
            [ 2.0*x,  (o+i)*x],
            [     z, -(o+i)*x]
        ] * (0.5 * o);
        let gate = U3::new(3.0*::std::f64::consts::FRAC_PI_2,
            ::std::f64::consts::FRAC_PI_4, ::std::f64::consts::FRAC_PI_2);
        gate_test(gate, &mut state, &result);
    }

    #[test]
    fn test_open_qasm()
    {
        let bit_names = [String::from("qb")];
        let qasm = U3::new(1.0, 2.25, 3.5).open_qasm(&bit_names, &[0]);
        assert_eq!(qasm, Ok(String::from("u3(1, 2.25, 3.5) qb")));
    }

    #[test]
    fn test_c_qasm()
    {
        let bit_names = [String::from("qb")];
        let qasm = U3::new(1.0, 2.25, 3.5).c_qasm(&bit_names, &[0]);
        assert_eq!(qasm, Ok(String::from("rz qb, 3.5\nry qb, 1\n; rz qb 2.25")));
    }

    #[test]
    fn test_latex()
    {
        let gate = U3::new(0.17, ::std::f64::consts::FRAC_PI_4, ::std::f64::consts::LN_2);
        let mut state = LatexExportState::new(1, 0);
        assert_eq!(gate.latex(&[0], &mut state), Ok(()));
        assert_eq!(state.code(),
r#"\Qcircuit @C=1em @R=.7em {
    \lstick{\ket{0}} & \gate{U_3(0.1700, 0.7854, 0.6931)} & \qw \\
}
"#);

        let gate = U3::new(::std::f64::consts::PI, -1.2, ::std::f64::consts::LN_2);
        let mut state = LatexExportState::new(1, 0);
        assert_eq!(gate.latex(&[0], &mut state), Ok(()));
        assert_eq!(state.code(),
r#"\Qcircuit @C=1em @R=.7em {
    \lstick{\ket{0}} & \gate{U_3(3.1416, -1.2000, 0.6931)} & \qw \\
}
"#);

        let gate = U3::new(::std::f64::consts::FRAC_PI_2, 12.0, -3.14);
        let mut state = LatexExportState::new(1, 0);
        assert_eq!(gate.latex(&[0], &mut state), Ok(()));
        assert_eq!(state.code(),
r#"\Qcircuit @C=1em @R=.7em {
    \lstick{\ket{0}} & \gate{U_3(1.5708, 12.0000, -3.1400)} & \qw \\
}
"#);
    }

    #[test]
    fn test_square()
    {
        let gate = U3::new(::std::f64::consts::FRAC_PI_2, 12.0, -3.14);
        assert!(matches!(gate.square(), Err(crate::error::Error::OpNotImplemented(_, _))));
    }
}
