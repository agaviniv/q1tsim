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

pub const COMPLEX_ZERO:   num_complex::Complex64 = num_complex::Complex { re: 0.0, im: 0.0 };
pub const COMPLEX_ONE:    num_complex::Complex64 = num_complex::Complex { re: 1.0, im: 0.0 };
pub const COMPLEX_HSQRT2: num_complex::Complex64 = num_complex::Complex { re: ::std::f64::consts::FRAC_1_SQRT_2, im: 0.0 };
pub const COMPLEX_I:      num_complex::Complex64 = num_complex::Complex { re: 0.0, im: 1.0 };

pub type CNumber = num_complex::Complex64;
pub type CVector = ndarray::Array1<CNumber>;
pub type CMatrix = ndarray::Array2<CNumber>;
pub type CVecSliceMut<'a> = ndarray::ArrayViewMut1<'a, CNumber>;
pub type CMatSliceMut<'a> = ndarray::ArrayViewMut2<'a, CNumber>;

/// Compute the Kronecker product `v0` ⊗ `v1`.
pub fn kron_vec(v0: &CVector, v1: &CVector) -> CVector
{
    let (n0, n1) = (v0.len(), v1.len());
    let mut res = CVector::zeros(n0*n1);
    for (i, &x) in v0.iter().enumerate()
    {
        res.slice_mut(s![i*n1..(i+1)*n1]).assign(&(v1 * x));
    }
    res
}

/// Compute the Kronecker product `m0` ⊗ `m1`.
pub fn kron_mat(a0: &CMatrix, a1: &CMatrix) -> CMatrix
{
    let (n0, m0, n1, m1) = (a0.rows(), a0.cols(), a1.rows(), a1.cols());
    let mut res = CMatrix::zeros((n0*n1, m0*m1));
    for i in 0..n0
    {
        for j in 0..m0
        {
            res.slice_mut(s![i*n1..(i+1)*n1, j*m1..(j+1)*m1]).assign(&(a1 * a0[[i, j]]));
        }
    }
    res
}

#[macro_export]
macro_rules! assert_complex_vector_eq
{
    ($a0:expr, $a1:expr) => {
        {
            let (n0, n1) = ($a0.len(), $a1.len());

            assert!(n0 == n1, "Incompatible array dimensions, {} vs {}", n0, n1);

            let diff = $a0 - $a1;
            let tol = 1.0e-15;
            let mut diff_elems = vec![];
            for i in 0..n0
            {
                if diff[i].norm() > tol
                {
                    diff_elems.push(i);
                }
            }

            if !diff_elems.is_empty()
            {
                let mut msg = String::from("Differences found:\n");
                for i in diff_elems
                {
                    msg += &format!("At ({}): x = {}, y = {}, difference = {}\n",
                        i, $a0[i], $a1[i], diff[i]);
                }
                panic!("{}", msg);
            }
        }
    }
}

#[macro_export]
macro_rules! assert_complex_matrix_eq
{
    ($a0:expr, $a1:expr) => {
        {
            let (n0, m0, n1, m1) = ($a0.rows(), $a0.cols(), $a1.rows(), $a1.cols());

            assert!(n0 == n1 && m0 == m1, "Incompatible array dimensions, {} × {} vs {} × {}",
                n0, m0, n1, m1);

            let diff = $a0 - $a1;
            let tol = 1.0e-15;
            let mut diff_elems = vec![];
            for i in 0..n0
            {
                for j in 0..m0
                {
                    if diff[[i, j]].norm() > tol
                    {
                        diff_elems.push((i, j));
                    }
                }
            }

            if !diff_elems.is_empty()
            {
                let mut msg = String::from("Differences found:\n");
                for (i, j) in diff_elems
                {
                    msg += &format!("At ({}, {}): x = {}, y = {}, difference = {}\n",
                        i, j, $a0[[i, j]], $a1[[i, j]], diff[[i, j]]);
                }
                panic!("{}", msg);
            }
        }
    }
}
