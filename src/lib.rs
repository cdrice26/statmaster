use js_sys;
use js_sys::{Object, Reflect};
use log::{info, Level};
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
        "greater" => dist.cdf(1.0 / f),
        "less" => dist.cdf(f),
        "two-sided" => 2.0 * (1.0 - dist.cdf(f)),
        _ => panic!("Invalid test type"),
    };

    let _ = Reflect::set(&obj, &JsValue::from_str("f"), &JsValue::from_f64(f));
    let _ = Reflect::set(&obj, &JsValue::from_str("p"), &JsValue::from_f64(p));

    obj.into()
}
