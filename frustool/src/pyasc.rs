/*
 * pyasc.rs - Python associations and correspondence with frustool.
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

use super::functions::transform::{
    acos_py, acosh_py, asin_py, asinh_py, atan_py, atanh_py, ceil_py, cos_py, cosh_py, exp_py,
    floor_py, ln_py, log10_py, log2_py, rad2deg_py, sin_py, sinh_py, sqrt_py, tan_py, tanh_py,
};
use super::functions::vector::{add_py, div_py, mul_py, sub_py};
use super::metrics::roc::{roc_py, rocp_py, rocr_100_py, rocr_py};
use cpython::{py_fn, py_module_initializer};

py_module_initializer!(libfrustool, |py, m| {
    m.add(
        py,
        "__doc__",
        "This is Frustool, A Financial analyzer for python",
    )?;
    // functions - transform.rs
    m.add(py, "sin", py_fn!(py, sin_py(input: Vec<f64>)))?;
    m.add(py, "cos", py_fn!(py, cos_py(input: Vec<f64>)))?;
    m.add(py, "tan", py_fn!(py, tan_py(input: Vec<f64>)))?;
    m.add(py, "sinh", py_fn!(py, sinh_py(input: Vec<f64>)))?;
    m.add(py, "cosh", py_fn!(py, cosh_py(input: Vec<f64>)))?;
    m.add(py, "tanh", py_fn!(py, tanh_py(input: Vec<f64>)))?;
    m.add(py, "asin", py_fn!(py, asin_py(input: Vec<f64>)))?;
    m.add(py, "acos", py_fn!(py, acos_py(input: Vec<f64>)))?;
    m.add(py, "atan", py_fn!(py, atan_py(input: Vec<f64>)))?;
    m.add(py, "asinh", py_fn!(py, asinh_py(input: Vec<f64>)))?;
    m.add(py, "acosh", py_fn!(py, acosh_py(input: Vec<f64>)))?;
    m.add(py, "atanh", py_fn!(py, atanh_py(input: Vec<f64>)))?;
    m.add(py, "ceil", py_fn!(py, ceil_py(input: Vec<f64>)))?;
    m.add(py, "exp", py_fn!(py, exp_py(input: Vec<f64>)))?;
    m.add(py, "floor", py_fn!(py, floor_py(input: Vec<f64>)))?;
    m.add(py, "ln", py_fn!(py, ln_py(input: Vec<f64>)))?;
    m.add(py, "log10", py_fn!(py, log10_py(input: Vec<f64>)))?;
    m.add(py, "log2", py_fn!(py, log2_py(input: Vec<f64>)))?;
    m.add(py, "sqrt", py_fn!(py, sqrt_py(input: Vec<f64>)))?;
    m.add(py, "rad2deg", py_fn!(py, rad2deg_py(input: Vec<f64>)))?;
    // metrics - roc.rs
    m.add(
        py,
        "roc",
        py_fn!(py, roc_py(input: Vec<f64>, time_period: i32)),
    )?;
    m.add(
        py,
        "rocp",
        py_fn!(py, rocp_py(input: Vec<f64>, time_period: i32)),
    )?;
    m.add(
        py,
        "rocr",
        py_fn!(py, rocr_py(input: Vec<f64>, time_period: i32)),
    )?;
    m.add(
        py,
        "rocr100",
        py_fn!(py, rocr_100_py(input: Vec<f64>, time_period: i32)),
    )?;
    // functions - vector.rs
    m.add(
        py,
        "add",
        py_fn!(py, add_py(vector1: Vec<f64>, vector2: Vec<f64>)),
    )?;
    m.add(
        py,
        "sub",
        py_fn!(py, sub_py(vector1: Vec<f64>, vector2: Vec<f64>)),
    )?;
    m.add(
        py,
        "mul",
        py_fn!(py, mul_py(vector1: Vec<f64>, vector2: Vec<f64>)),
    )?;
    m.add(
        py,
        "div",
        py_fn!(py, div_py(vector1: Vec<f64>, vector2: Vec<f64>)),
    )?;
    Ok(())
});
