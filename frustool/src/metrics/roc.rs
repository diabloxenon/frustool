/*
 * roc.rs - Rate of Change calculation module for frustool.
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
//! Rate of change
//!
use crate::errors::Error;
use cpython::{exc, PyErr, PyResult, Python};

pub struct Roc {
    input: Vec<f64>,
    time_period: i32,
}

impl Roc {
    pub fn new(input: Vec<f64>, time_period: i32) -> Result<Self, Error> {
        if input.is_empty() {
            return Err(Error::InvalidParams(String::from("Input Vector is Empty")));
        }
        if time_period < 2 {
            return Err(Error::InvalidParams(String::from(
                "Time Period should be greater than 2",
            )));
        }
        Ok(Self { input, time_period })
    }
    /// ROCP - Rate Of Change in Percent: (price-prevprice)/prevprice
    pub fn rocp(&self) -> Vec<f64> {
        let mut output_vector = Vec::with_capacity(self.input.len());
        let start_index = self.time_period as usize;
        let trailing_index = start_index - self.time_period as usize;
        for idx in start_index..self.input.len() {
            let tmp = self.input[trailing_index];
            if tmp != 0.0 {
                let result = (self.input[idx] - tmp) / tmp;
                output_vector.push(result);
            } else {
                output_vector.push(0.0)
            }
        }
        output_vector
    }
    /// ROCP - Rate Of Change: ((price/prevprice) - 1) * 100
    pub fn roc(&self) -> Vec<f64> {
        let mut output_vector = Vec::with_capacity(self.input.len());
        let start_index = self.time_period as usize;
        let trailing_index = start_index - self.time_period as usize;
        for idx in start_index..self.input.len() {
            let tmp = self.input[trailing_index];
            if tmp != 0.0 {
                let result = ((self.input[idx] / tmp) - 1.0) * 100.0;
                output_vector.push(result);
            } else {
                output_vector.push(0.0)
            }
        }
        output_vector
    }
    /// ROCr - Rate Of Change ratio: price/prevprice
    pub fn rocr(&self) -> Vec<f64> {
        let mut output_vector = Vec::with_capacity(self.input.len());
        let start_index = self.time_period as usize;
        let trailing_index = start_index - self.time_period as usize;
        for idx in start_index..self.input.len() {
            let tmp = self.input[trailing_index];
            if tmp != 0.0 {
                let result = self.input[idx] / tmp;
                output_vector.push(result);
            } else {
                output_vector.push(0.0)
            }
        }
        output_vector
    }
    /// ROCr100 - Rate Of Change ratio 100 scale: (price/prevprice) * 100
    pub fn rocr100(&self) -> Vec<f64> {
        let mut output_vector = Vec::with_capacity(self.input.len());
        let start_index = self.time_period as usize;
        let trailing_index = start_index - self.time_period as usize;
        for idx in start_index..self.input.len() {
            let tmp = self.input[trailing_index];
            if tmp != 0.0 {
                let result = (self.input[idx] / tmp) * 100.0;
                output_vector.push(result);
            } else {
                output_vector.push(0.0)
            }
        }
        output_vector
    }
}

/**
 *
 * Python library/module function counterparts
 *
 */

pub fn roc_py(py: Python, input: Vec<f64>, time_period: i32) -> PyResult<Vec<f64>> {
    let roc_wrapped = Roc::new(input, time_period);
    let roc = match roc_wrapped {
        Ok(x) => x,
        Err(err_typ) => match err_typ {
            Error::InvalidParams(msg) => return Err(PyErr::new::<exc::ValueError, _>(py, msg)),
        },
    };
    let out_vec = roc.roc();
    Ok(out_vec)
}
pub fn rocp_py(py: Python, input: Vec<f64>, time_period: i32) -> PyResult<Vec<f64>> {
    let roc_wrapped = Roc::new(input, time_period);
    let roc = match roc_wrapped {
        Ok(x) => x,
        Err(err_typ) => match err_typ {
            Error::InvalidParams(msg) => return Err(PyErr::new::<exc::ValueError, _>(py, msg)),
        },
    };
    let out_vec = roc.rocp();
    Ok(out_vec)
}
pub fn rocr_py(py: Python, input: Vec<f64>, time_period: i32) -> PyResult<Vec<f64>> {
    let roc_wrapped = Roc::new(input, time_period);
    let roc = match roc_wrapped {
        Ok(x) => x,
        Err(err_typ) => match err_typ {
            Error::InvalidParams(msg) => return Err(PyErr::new::<exc::ValueError, _>(py, msg)),
        },
    };
    let out_vec = roc.rocr();
    Ok(out_vec)
}
pub fn rocr_100_py(py: Python, input: Vec<f64>, time_period: i32) -> PyResult<Vec<f64>> {
    let roc_wrapped = Roc::new(input, time_period);
    let roc = match roc_wrapped {
        Ok(x) => x,
        Err(err_typ) => match err_typ {
            Error::InvalidParams(msg) => return Err(PyErr::new::<exc::ValueError, _>(py, msg)),
        },
    };
    let out_vec = roc.rocr100();
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
    use super::super::testcon::*;
    use super::*;

    #[test]
    fn test_roc() {
        let period = Roc::new(Vec::from(VECTOR), 10).unwrap();
        assert_eq!(period.roc(), VECTOR_ROC_10)
    }

    #[test]
    fn test_rocp() {
        let period = Roc::new(Vec::from(VECTOR), 10).unwrap();
        assert_eq!(period.rocp(), VECTOR_ROCP_10)
    }

    #[test]
    fn test_rocr() {
        let period = Roc::new(Vec::from(VECTOR), 10).unwrap();
        assert_eq!(period.rocr(), VECTOR_ROCR_10)
    }

    #[test]
    fn test_rocr100() {
        let period = Roc::new(Vec::from(VECTOR), 10).unwrap();
        assert_eq!(period.rocr100(), VECTOR_ROCR_100_10)
    }
}
