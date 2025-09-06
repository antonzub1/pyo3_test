use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

#[pyfunction]
fn mean(values: Vec<f64>) -> PyResult<f64> {
    if values.is_empty() {
        return Err(PyValueError::new_err("Cannot compute mean value of empty array"));
    }
    
    let mut sum = 0.0;
    let mut has_pos_inf = false;
    let mut has_neg_inf = false;
    
    for value in values.iter() {
        if value.is_nan() {
            // Early return if there is NAN
            return Ok(f64::NAN);
        } else if value.is_infinite() {
            if value.is_sign_positive() {
                has_pos_inf = true;
            } else {
                has_neg_inf = true;
            }
        } else {
            sum += value;
        }
    }
    
    // +inf + -inf == NAN according to IEEE 754
    if has_pos_inf && has_neg_inf {
        return Ok(f64::NAN);
    }
    
    if has_pos_inf && !has_neg_inf {
        return Ok(f64::INFINITY);
    }
    
    if has_neg_inf && !has_pos_inf {
        return Ok(f64::NEG_INFINITY);
    }
    
    Ok(sum / values.len() as f64)
}

/// Compute the median of a float array
#[pyfunction]
fn median(mut values: Vec<f64>) -> PyResult<f64> {
    if values.is_empty() {
        return Err(PyValueError::new_err("Cannot compute median value of empty array"));
    }

    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    
    // Check for NaN values first
    if values.iter().any(|&x| x.is_nan()) {
        return Ok(f64::NAN);
    }
    
    let len = values.len();
    
    if len % 2 == 1 {
        Ok(values[len / 2])
    } else {
        // Return mean of two middle values if total count is even
        let mid1 = values[len / 2 - 1];
        let mid2 = values[len / 2];
        
        if mid1.is_infinite() || mid2.is_infinite() {
            if mid1 == mid2 {
                Ok(mid1) 
            // +inf + -inf == NAN according to IEEE 754
            } else if mid1.is_infinite() && mid2.is_infinite() {
                Ok(f64::NAN) 
            } else {
                // Return inf if one of values is infinity:w
                if mid1.is_infinite() {
                    Ok(mid1)
                } else {
                    Ok(mid2)
                }
            }
        } else {
            Ok((mid1 + mid2) / 2.0)
        }
    }
}

#[pymodule(name="stat_functions")]
fn stat_functions(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mean, m)?)?;
    m.add_function(wrap_pyfunction!(median, m)?)?;
    Ok(())
}
