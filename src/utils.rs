use js_sys;
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
pub fn js_array_to_vector(js_array: &JsValue) -> Vec<f64> {
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
pub fn js_nested_array_to_vector(js_array: &JsValue) -> Vec<JsValue> {
    // Convert the JsValue to a Vec<JsValue>
    js_sys::Array::from(js_array).to_vec() // Convert to Vec<JsValue>
}

/// Converts a vector of f64 to a JsValue representing a JavaScript array.
///
/// # Arguments
///
/// * `vec` - A vector of f64 numbers to be converted.
///
/// # Returns
///
/// * A JsValue representing the JavaScript array.
#[allow(unused)]
pub fn vec_to_jsvalue(vec: Vec<f64>) -> JsValue {
    // Create a JavaScript array from the Vec
    let js_array = js_sys::Array::new();
    for item in vec {
        js_array.push(&JsValue::from(item));
    }
    js_array.into() // Convert the js_sys::Array to JsValue
}

/// Converts a vector of vectors of f64 to a JsValue representing a nested JavaScript array.
///
/// # Arguments
///
/// * `vec` - A vector of vectors of f64 numbers to be converted.
///
/// # Returns
///
/// * A JsValue representing the nested JavaScript array.
#[allow(unused)]
pub fn nested_vec_to_jsvalue(vec: Vec<Vec<f64>>) -> JsValue {
    // Create a JavaScript array from the Vec
    let js_array = js_sys::Array::new();
    for item in vec {
        js_array.push(&vec_to_jsvalue(item));
    }
    js_array.into() // Convert the js_sys::Array to JsValue
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

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
}
