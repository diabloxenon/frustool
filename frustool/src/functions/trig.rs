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

pub fn cos(input: f64) -> f64 {
    input.cos()
}
pub fn tan(input: f64) -> f64 {
    input.tan()
}
pub fn sin(input: f64) -> f64 {
    input.sin()
}
pub fn cosh(input: f64) -> f64 {
    input.cosh()
}
pub fn tanh(input: f64) -> f64 {
    input.tanh()
}
pub fn sinh(input: f64) -> f64 {
    input.sinh()
}
pub fn acos(input: f64) -> f64 {
    input.acos()
}
pub fn atan(input: f64) -> f64 {
    input.atan()
}
pub fn asin(input: f64) -> f64 {
    input.asin()
}
pub fn acosh(input: f64) -> f64 {
    input.acosh()
}
pub fn atanh(input: f64) -> f64 {
    input.atanh()
}
pub fn asinh(input: f64) -> f64 {
    input.asinh()
}

/*********************************************
 * Python bindings section
 */

pub fn sin_py(_: Python, input: f64) -> PyResult<f64> {
    let out = sin(input);
    Ok(out)
}

pub fn cos_py(_: Python, input: f64) -> PyResult<f64> {
    let out = cos(input);
    Ok(out)
}

pub fn tan_py(_: Python, input: f64) -> PyResult<f64> {
    let out = tan(input);
    Ok(out)
}

pub fn sinh_py(_: Python, input: f64) -> PyResult<f64> {
    let out = sinh(input);
    Ok(out)
}

pub fn cosh_py(_: Python, input: f64) -> PyResult<f64> {
    let out = cosh(input);
    Ok(out)
}

pub fn tanh_py(_: Python, input: f64) -> PyResult<f64> {
    let out = tanh(input);
    Ok(out)
}

pub fn asin_py(_: Python, input: f64) -> PyResult<f64> {
    let out = asin(input);
    Ok(out)
}

pub fn acos_py(_: Python, input: f64) -> PyResult<f64> {
    let out = acos(input);
    Ok(out)
}

pub fn atan_py(_: Python, input: f64) -> PyResult<f64> {
    let out = atan(input);
    Ok(out)
}

pub fn asinh_py(_: Python, input: f64) -> PyResult<f64> {
    let out = asinh(input);
    Ok(out)
}

pub fn acosh_py(_: Python, input: f64) -> PyResult<f64> {
    let out = acosh(input);
    Ok(out)
}

pub fn atanh_py(_: Python, input: f64) -> PyResult<f64> {
    let out = atanh(input);
    Ok(out)
}
