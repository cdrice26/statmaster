use js_sys;
use js_sys::{Object, Reflect};
use log::Level;
use statrs::distribution::ContinuousCDF;
use statrs::distribution::FisherSnedecor;
use statrs::statistics::Statistics;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn js_array_to_vector(js_array: &JsValue) -> Vec<f64> {
    // Convert the JsValue to a Vec<f64>
    let array: Vec<f64> = js_sys::Array::from(js_array)
        .iter()
        .filter_map(|value| value.as_f64()) // Convert each element to f64
        .collect();

    array
}

#[wasm_bindgen]
pub fn variance_test(column1: &JsValue, column2: &JsValue, tails: &JsValue) -> JsValue {
    let tails = tails.as_string().unwrap(); // can be "two-sided", "less" or "greater"
    let _ = console_log::init_with_level(Level::Debug);

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

#[cfg(test)]
mod tests {
    use super::*;
    use js_sys::*;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;

    fn vec_to_jsvalue(vec: Vec<f64>) -> JsValue {
        // Create a JavaScript array from the Vec
        let js_array = js_sys::Array::new();
        for item in vec {
            js_array.push(&JsValue::from(item));
        }
        js_array.into() // Convert the js_sys::Array to JsValue
    }

    #[wasm_bindgen_test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[wasm_bindgen_test]
    fn test_js_array_to_vector() {
        let js_array = vec_to_jsvalue(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let js_array_2 = vec_to_jsvalue(vec![2.0, 3.0, 4.0, 5.0, 6.0]);

        let result = js_array_to_vector(&js_array);
        let result_2 = js_array_to_vector(&js_array_2);

        assert_eq!(result, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(result_2, vec![2.0, 3.0, 4.0, 5.0, 6.0]);
    }

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
}
