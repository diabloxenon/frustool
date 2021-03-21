/*
 * operations.rs - Vector math operations for libfrustool.
 *
 * Copyright 2020-2021 Naman Bishnoi
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/

//! Vector coupling math operations
//!
//! Calculates two vector operations fast and effortlessly.

use crate::errors::Error;
use cpython::{exc, PyErr, PyResult, Python};

pub struct Vectors {
    input_vector1: Vec<f64>,
    input_vector2: Vec<f64>,
}

impl Vectors {
    pub fn new(input_vector1: Vec<f64>, input_vector2: Vec<f64>) -> Result<Self, Error> {
        if input_vector1.len() != input_vector2.len() {
            return Err(Error::InvalidParams(String::from(
                "Vectors length mismatch",
            )));
        }
        Ok(Self {
            input_vector1,
            input_vector2,
        })
    }

    /// Add - Vector arithmetic addition
    pub fn add(&self) -> Vec<f64> {
        let mut output_vector = Vec::new();
        for idx in 0..self.input_vector1.len() {
            output_vector.push(self.input_vector1[idx] + self.input_vector2[idx])
        }
        output_vector
    }

    /// Subtraction - Vector arithmetic subtraction
    pub fn sub(&self) -> Vec<f64> {
        let mut output_vector = Vec::new();
        for idx in 0..self.input_vector1.len() {
            output_vector.push(self.input_vector1[idx] - self.input_vector2[idx])
        }
        output_vector
    }
    /// Divide - Vector arithmetic division
    pub fn div(&self) -> Vec<f64> {
        let mut output_vector = Vec::new();
        for idx in 0..self.input_vector1.len() {
            output_vector.push(self.input_vector1[idx] / self.input_vector2[idx])
        }
        output_vector
    }
    /// Muliplication - Vector arithmetic multiplication
    pub fn mul(&self) -> Vec<f64> {
        let mut output_vector = Vec::new();
        for idx in 0..self.input_vector1.len() {
            output_vector.push(self.input_vector1[idx] * self.input_vector2[idx])
        }
        output_vector
    }
}

pub fn add_py(py: Python, vector1: Vec<f64>, vector2: Vec<f64>) -> PyResult<Vec<f64>> {
    let vec_zip = Vectors::new(vector1, vector2);
    let vectors = match vec_zip {
        Ok(x) => x,
        Err(err_typ) => match err_typ {
            Error::InvalidParams(msg) => return Err(PyErr::new::<exc::IndexError, _>(py, msg)),
        },
    };
    let out_vec = vectors.add();
    Ok(out_vec)
}

pub fn sub_py(py: Python, vector1: Vec<f64>, vector2: Vec<f64>) -> PyResult<Vec<f64>> {
    let vec_zip = Vectors::new(vector1, vector2);
    let vectors = match vec_zip {
        Ok(x) => x,
        Err(err_typ) => match err_typ {
            Error::InvalidParams(msg) => return Err(PyErr::new::<exc::IndexError, _>(py, msg)),
        },
    };
    let out_vec = vectors.sub();
    Ok(out_vec)
}

pub fn mul_py(py: Python, vector1: Vec<f64>, vector2: Vec<f64>) -> PyResult<Vec<f64>> {
    let vec_zip = Vectors::new(vector1, vector2);
    let vectors = match vec_zip {
        Ok(x) => x,
        Err(err_typ) => match err_typ {
            Error::InvalidParams(msg) => return Err(PyErr::new::<exc::IndexError, _>(py, msg)),
        },
    };
    let out_vec = vectors.mul();
    Ok(out_vec)
}

pub fn div_py(py: Python, vector1: Vec<f64>, vector2: Vec<f64>) -> PyResult<Vec<f64>> {
    let vec_zip = Vectors::new(vector1, vector2);
    let vectors = match vec_zip {
        Ok(x) => x,
        Err(err_typ) => match err_typ {
            Error::InvalidParams(msg) => return Err(PyErr::new::<exc::IndexError, _>(py, msg)),
        },
    };
    let out_vec = vectors.div();
    Ok(out_vec)
}

/**
 *
 * Ahoy mate! Unit tests ahead, if you want to test every
 * functionality of the library/program then feel free to do so;
 *
 */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vect_mul() {
        let vec1 = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 = vec![1.0, 2.0, 3.0, 4.0];
        let vectors = Vectors::new(vec1, vec2).unwrap();
        let vec_mul = vectors.mul();

        assert_eq!(vec_mul, vec![1.0, 4.0, 9.0, 16.0])
    }
    #[test]
    fn test_vect_add() {
        let vec1 = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 = vec![1.0, 2.0, 3.0, 4.0];

        let vectors = Vectors::new(vec1, vec2).unwrap();
        let vec_add = vectors.add();

        assert_eq!(vec_add, vec![2.0, 4.0, 6.0, 8.0])
    }
    #[test]
    fn test_vect_div() {
        let vec1 = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 = vec![1.0, 2.0, 3.0, 4.0];

        let vectors = Vectors::new(vec1, vec2).unwrap();
        let vec_div = vectors.div();

        assert_eq!(vec_div, vec![1.0, 1.0, 1.0, 1.0])
    }
    #[test]
    fn test_vect_sub() {
        let vec1 = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 = vec![1.0, 2.0, 3.0, 4.0];

        let vectors = Vectors::new(vec1, vec2).unwrap();
        let vec_sub = vectors.sub();

        assert_eq!(vec_sub, vec![0.0, 0.0, 0.0, 0.0])
    }
}
