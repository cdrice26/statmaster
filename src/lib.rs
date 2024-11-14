use js_sys;
use js_sys::{Object, Reflect};
use log::{info, log_enabled, Level};
use statrs::distribution::ContinuousCDF;
use statrs::distribution::FisherSnedecor;
use statrs::statistics::Statistics;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

/// Converts a JavaScript array (JsValue) to a Rust vector of f64.
///
/// # Arguments
///
/// * `js_array` - A reference to a JsValue representing a JavaScript array.
///
/// # Returns
///
/// * A Vec<f64> containing the converted elements of the JavaScript array.
fn js_array_to_vector(js_array: &JsValue) -> Vec<f64> {
    // Convert the JsValue to a Vec<f64>
    let array: Vec<JsValue> = js_sys::Array::from(js_array).to_vec(); // Convert to Vec<JsValue>

    array
        .into_iter()
        .filter_map(|value| value.as_f64()) // Filter and map to f64
        .collect() // Collect into Vec<f64>
}

/// Converts a nested JavaScript array (JsValue) to a Rust vector of JsValue.
///
/// # Arguments
///
/// * `js_array` - A reference to a JsValue representing a nested JavaScript array.
///
/// # Returns
///
/// * A Vec<JsValue> containing the converted elements of the nested JavaScript array.
fn js_nested_array_to_vector(js_array: &JsValue) -> Vec<JsValue> {
    // Convert the JsValue to a Vec<JsValue>
    js_sys::Array::from(js_array).to_vec() // Convert to Vec<JsValue>
}

/// Performs a variance test between two columns of data represented as JavaScript arrays.
///
/// # Arguments
///
/// * `column1` - A reference to a JsValue representing the first JavaScript array.
/// * `column2` - A reference to a JsValue representing the second JavaScript array.
/// * `tails` - A reference to a JsValue indicating the type of test ("two-sided", "less", or "greater").
///
/// # Returns
///
/// * A JsValue object containing the test results, including "n1", "n2", "s1", "s2", "f", and "p".
#[wasm_bindgen]
pub fn variance_test(column1: &JsValue, column2: &JsValue, tails: &JsValue) -> JsValue {
    let tails = tails.as_string().unwrap(); // can be "two-sided", "less" or "greater"

    let obj = Object::new();

    if tails != "two-sided" && tails != "less" && tails != "greater" {
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("error"),
            &JsValue::from_str("Invalid test type"),
        );
        return obj.into();
    }

    let c1 = js_array_to_vector(column1);
    let c2 = js_array_to_vector(column2);

    let n1 = c1.len() as f64;
    let n2 = c2.len() as f64;

    let s1 = c1.std_dev();
    let s2 = c2.std_dev();

    // Check for division by zero
    if s2 == 0.0 {
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("error"),
            &JsValue::from_str("Division by zero"),
        );
        return obj.into();
    }

    let f = s1.powi(2) / s2.powi(2);

    let _ = Reflect::set(&obj, &JsValue::from_str("n1"), &JsValue::from_f64(n1));
    let _ = Reflect::set(&obj, &JsValue::from_str("n2"), &JsValue::from_f64(n2));
    let _ = Reflect::set(&obj, &JsValue::from_str("s1"), &JsValue::from_f64(s1));
    let _ = Reflect::set(&obj, &JsValue::from_str("s2"), &JsValue::from_f64(s2));

    // Set f and p in the object
    let dist = FisherSnedecor::new(n1 - 1.0, n2 - 1.0).unwrap();
    let p = match tails.as_str() {
        "greater" => 1.0 - dist.cdf(f),
        "less" => dist.cdf(f),
        "two-sided" => 2.0 * f64::min(1.0 - dist.cdf(f), dist.cdf(f)),
        _ => panic!("Invalid test type"),
    };

    let _ = Reflect::set(&obj, &JsValue::from_str("f"), &JsValue::from_f64(f));
    let _ = Reflect::set(&obj, &JsValue::from_str("p"), &JsValue::from_f64(p));

    obj.into()
}

/// Computes the F-statistic and p-value for a one-way ANOVA test.
///
/// # Arguments
///
/// * `data` - A JavaScript array of arrays, where each subarray represents a
///   group of data.
///
/// # Returns
///
/// * An object with two properties: `f` and `p`, the F-statistic and p-value,
///   respectively.
#[wasm_bindgen]
pub fn anova_1way_test(data: &JsValue) -> JsValue {
    let columns = js_nested_array_to_vector(data);
    let test_data: Vec<Vec<f64>> = columns
        .iter()
        .map(|item| js_array_to_vector(item))
        .collect();

    let n = test_data[0].len() as f64;
    let k = test_data.len() as f64;

    let mu_i = test_data.iter().map(|col| col.mean()).collect::<Vec<f64>>();
    let mu_t = mu_i.iter().sum::<f64>() / k;

    let sstr = n * mu_i.iter().map(|mi| (mi - mu_t).powi(2)).sum::<f64>();
    let tss = test_data
        .iter()
        .map(|col| col.iter().map(|x| (x - mu_t).powi(2)).sum::<f64>())
        .sum::<f64>();
    let sse = tss - sstr;

    let df_tr = k - 1.0;
    let df_e = (n * k) - k;

    let ms_tr = sstr / df_tr;
    let ms_e = sse / df_e;

    let f = ms_tr / ms_e;

    let dist = FisherSnedecor::new(df_tr, df_e).unwrap();
    let p = 1.0 - dist.cdf(f);

    let obj = Object::new();
    let _ = Reflect::set(&obj, &JsValue::from_str("f"), &JsValue::from_f64(f));
    let _ = Reflect::set(&obj, &JsValue::from_str("p"), &JsValue::from_f64(p));
    obj.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use js_sys::*;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    /// Converts a vector of f64 to a JsValue representing a JavaScript array.
    ///
    /// # Arguments
    ///
    /// * `vec` - A vector of f64 numbers to be converted.
    ///
    /// # Returns
    ///
    /// * A JsValue representing the JavaScript array.
    fn vec_to_jsvalue(vec: Vec<f64>) -> JsValue {
        // Create a JavaScript array from the Vec
        let js_array = js_sys::Array::new();
        for item in vec {
            js_array.push(&JsValue::from(item));
        }
        js_array.into() // Convert the js_sys::Array to JsValue
    }

    fn nested_vec_to_jsvalue(vec: Vec<Vec<f64>>) -> JsValue {
        // Create a JavaScript array from the Vec
        let js_array = js_sys::Array::new();
        for item in vec {
            js_array.push(&vec_to_jsvalue(item));
        }
        js_array.into() // Convert the js_sys::Array to JsValue
    }

    #[allow(unused)]
    #[wasm_bindgen_test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[allow(unused)]
    #[wasm_bindgen_test]
    fn test_js_array_to_vector() {
        let js_array = vec_to_jsvalue(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let js_array_2 = vec_to_jsvalue(vec![2.0, 3.0, 4.0, 5.0, 6.0]);

        let result = js_array_to_vector(&js_array);
        let result_2 = js_array_to_vector(&js_array_2);

        assert_eq!(result, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(result_2, vec![2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[allow(unused)]
    #[wasm_bindgen_test]
    fn test_variance_test() {
        let column1 = vec_to_jsvalue(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let column2 = vec_to_jsvalue(vec![2.0, 3.0, 4.0, 5.0, 6.0]);

        let result1 = variance_test(&column1, &column2, &JsValue::from_str("less"));
        let result2 = variance_test(&column1, &column2, &JsValue::from_str("greater"));
        let result3 = variance_test(&column1, &column2, &JsValue::from_str("two-sided"));

        let p1 = Reflect::get(&result1, &JsValue::from_str("p")).unwrap();
        let p2 = Reflect::get(&result2, &JsValue::from_str("p")).unwrap();
        let p3 = Reflect::get(&result3, &JsValue::from_str("p")).unwrap();

        assert!((p1.as_f64().unwrap() - 0.5).abs() < 0.01);
        assert!((p2.as_f64().unwrap() - 0.5).abs() < 0.01);
        assert!((p3.as_f64().unwrap() - 1.0).abs() < 0.01);
    }

    #[allow(unused)]
    #[wasm_bindgen_test]
    fn test_anova_1way_test() {
        let column1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let column2 = vec![2.0, 3.0, 4.0, 5.0, 6.0];
        let data = vec![column1, column2];
        let data_js = nested_vec_to_jsvalue(data);

        let result = anova_1way_test(&data_js);

        let p = Reflect::get(&result, &JsValue::from_str("p")).unwrap();
        let f = Reflect::get(&result, &JsValue::from_str("f")).unwrap();

        assert!(f.as_f64().unwrap().abs() - 1.0 < 0.01);
        assert!(p.as_f64().unwrap().abs() - 0.3465 < 0.01);
    }
}
