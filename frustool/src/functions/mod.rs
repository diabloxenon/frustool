/*
 * functions.rs - Associated Functions for the Technical Analysis Library
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

pub mod trig;

use cpython::{py_fn, py_module_initializer};
use trig::{
    acos_py, acosh_py, asin_py, asinh_py, atan_py, atanh_py, cos_py, cosh_py, sin_py, sinh_py,
    tan_py, tanh_py,
};

py_module_initializer!(libfrustool, |py, m| {
    m.add(
        py,
        "__doc__",
        "This is Frustool, A Financial analyzer for python",
    )?;
    m.add(py, "sin", py_fn!(py, sin_py(input: f64)))?;
    m.add(py, "cos", py_fn!(py, cos_py(input: f64)))?;
    m.add(py, "tan", py_fn!(py, tan_py(input: f64)))?;
    m.add(py, "sinh", py_fn!(py, sinh_py(input: f64)))?;
    m.add(py, "cosh", py_fn!(py, cosh_py(input: f64)))?;
    m.add(py, "tanh", py_fn!(py, tanh_py(input: f64)))?;
    m.add(py, "asin", py_fn!(py, asin_py(input: f64)))?;
    m.add(py, "acos", py_fn!(py, acos_py(input: f64)))?;
    m.add(py, "atan", py_fn!(py, atan_py(input: f64)))?;
    m.add(py, "asinh", py_fn!(py, asinh_py(input: f64)))?;
    m.add(py, "acosh", py_fn!(py, acosh_py(input: f64)))?;
    m.add(py, "atanh", py_fn!(py, atanh_py(input: f64)))?;
    Ok(())
});
