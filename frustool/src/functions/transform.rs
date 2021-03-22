/*
 * trig.rs - Trigonometric helper functions for libfrustool.
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

use cpython::{PyResult, Python};

const PI: f64 = 3.14159265358979333846264338327950288419716939937510;
// const PI: f64 = 3.14159265358979323846264338327950288419716939937510;

pub fn sin(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].sin())
    }
    output_vector
}
pub fn cos(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].cos())
    }
    output_vector
}
pub fn tan(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].tan())
    }
    output_vector
}

pub fn sinh(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].sinh())
    }
    output_vector
}
pub fn cosh(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].cosh())
    }
    output_vector
}
pub fn tanh(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].tanh())
    }
    output_vector
}

pub fn asin(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].asin())
    }
    output_vector
}
pub fn acos(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].acos())
    }
    output_vector
}
pub fn atan(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].atan())
    }
    output_vector
}

pub fn asinh(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].asinh())
    }
    output_vector
}
pub fn acosh(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].acosh())
    }
    output_vector
}
pub fn atanh(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].atanh())
    }
    output_vector
}
pub fn round(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].round())
    }
    output_vector
}
pub fn ceil(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].ceil())
    }
    output_vector
}
pub fn exp(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].exp())
    }
    output_vector
}
pub fn floor(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].floor())
    }
    output_vector
}
pub fn ln(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].ln())
    }
    output_vector
}
pub fn log10(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].log10())
    }
    output_vector
}
pub fn log2(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].log2())
    }
    output_vector
}
pub fn sqrt(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx].sqrt())
    }
    output_vector
}
pub fn bias(input: Vec<f64>, num: f64) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        output_vector.push(input[idx] + num)
    }
    output_vector
}
pub fn rad2deg(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        let val = (input[idx] * 180.0) / PI;
        output_vector.push(val)
    }
    output_vector
}
pub fn deg2rad(input: Vec<f64>) -> Vec<f64> {
    let mut output_vector = Vec::new();
    for idx in 0..input.len() {
        let val = (input[idx] * PI) / 180.0;
        output_vector.push(val)
    }
    output_vector
}

/*********************************************
 * Python bindings section
 */

pub fn sin_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = sin(input);
    Ok(out)
}

pub fn cos_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = cos(input);
    Ok(out)
}

pub fn tan_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = tan(input);
    Ok(out)
}

pub fn sinh_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = sinh(input);
    Ok(out)
}

pub fn cosh_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = cosh(input);
    Ok(out)
}

pub fn tanh_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = tanh(input);
    Ok(out)
}

pub fn asin_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = asin(input);
    Ok(out)
}

pub fn acos_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = acos(input);
    Ok(out)
}

pub fn atan_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = atan(input);
    Ok(out)
}

pub fn asinh_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = asinh(input);
    Ok(out)
}

pub fn acosh_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = acosh(input);
    Ok(out)
}

pub fn atanh_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = atanh(input);
    Ok(out)
}

pub fn ceil_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = ceil(input);
    Ok(out)
}

pub fn floor_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = floor(input);
    Ok(out)
}

pub fn round_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = round(input);
    Ok(out)
}

pub fn exp_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = exp(input);
    Ok(out)
}

pub fn ln_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = ln(input);
    Ok(out)
}

pub fn log10_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = log10(input);
    Ok(out)
}

pub fn log2_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = log2(input);
    Ok(out)
}

pub fn sqrt_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = sqrt(input);
    Ok(out)
}

pub fn rad2deg_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = rad2deg(input);
    Ok(out)
}

pub fn deg2rad_py(_: Python, input: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = deg2rad(input);
    Ok(out)
}

pub fn bias_py(_: Python, input: Vec<f64>, num: f64) -> PyResult<Vec<f64>> {
    let out = bias(input, num);
    Ok(out)
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
    fn test_sin() {
        let vec = vec![0.0, 30.0, 45.0, 60.0, 90.0];

        let vec_sin = sin(deg2rad(vec));

        assert_eq!(
            vec_sin,
            [0.0, 0.5, 0.7071067811865477, 0.8660254037844387, 1.0]
        )
    }

    #[test]
    fn test_asin() {
        let vec = vec![0.0, 0.5, 0.7071067811865477, 0.8660254037844387, 1.0];

        let vec_sin = round(rad2deg(asin(vec)));

        assert_eq!(vec_sin, [0.0, 30.0, 45.0, 60.0, 90.0])
    }
}
