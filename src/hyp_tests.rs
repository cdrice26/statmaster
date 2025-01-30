use crate::utils::*;
use js_sys::Object;
use js_sys::Reflect;
use statrs::distribution::ContinuousCDF;
use statrs::distribution::FisherSnedecor;
use statrs::distribution::Normal;
use statrs::distribution::StudentsT;
use statrs::statistics::Statistics;
use wasm_bindgen::prelude::*;
use web_sys::console;

/// Performs a one-sample z-test on a column of data represented as a JavaScript array.
///
/// # Arguments
///
/// * `column` - A reference to a JsValue representing a JavaScript array of f64 numbers.
/// * `tails` - A reference to a JsValue indicating the type of test ("two-sided", "less", or "greater").
/// * `mu0` - A reference to a JsValue representing the null hypothesis mean.
///
/// # Returns
///
/// A reference to a JsValue representing the p-value and z-statistic of the one-sample z-test.
#[wasm_bindgen]
pub fn one_samp_z_test(column: &JsValue, tails: &JsValue, mu0: &JsValue) -> JsValue {
    let tails = tails.as_string().unwrap(); // can be "two-sided", "less" or "greater"
    let mu0 = mu0.as_f64().unwrap();

    let obj = Object::new();

    if tails != "two-sided" && tails != "less" && tails != "greater" {
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("error"),
            &JsValue::from_str("Invalid test type"),
        );
        return obj.into();
    }

    let data = js_array_to_vector(column);

    let n = data.len() as f64;
    let mean = data.iter().sum::<f64>() / n;
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
    let std_dev = variance.sqrt();

    // Standard error of the mean
    let std_error = std_dev / n.sqrt();

    // z-statistic
    let z = (mean - mu0) / std_error;

    let dist = Normal::new(0.0, 1.0).unwrap();

    let p = match tails.as_str() {
        "two-sided" => 2.0 * (1.0 - dist.cdf(z)),
        "less" => dist.cdf(z),
        "greater" => 1.0 - dist.cdf(z),
        _ => {
            let _ = Reflect::set(
                &obj,
                &JsValue::from_str("error"),
                &JsValue::from_str("Invalid test type"),
            );
            return obj.into();
        }
    };

    let obj = Object::new();
    let _ = Reflect::set(&obj, &JsValue::from_str("z"), &JsValue::from_f64(z));
    let _ = Reflect::set(&obj, &JsValue::from_str("p"), &JsValue::from_f64(p));

    obj.into()
}

/// Performs a one-sample t-test on a column of data represented as a JavaScript array.
///
/// # Arguments
///
/// * `column` - A reference to a JsValue representing a JavaScript array of f64 numbers.
/// * `tails` - A reference to a JsValue indicating the type of test ("two-sided", "less", or "greater").
/// * `mu0` - A reference to a JsValue representing the null hypothesis mean.
///
/// # Returns
///
/// * A JsValue representing the test statistic and p-value of the one-sample t-test.
#[wasm_bindgen]
pub fn one_samp_t_test(column: &JsValue, tails: &JsValue, mu0: &JsValue) -> JsValue {
    let tails = tails.as_string().unwrap(); // can be "two-sided", "less" or "greater"
    let mu0 = mu0.as_f64().unwrap();

    let obj = Object::new();

    if tails != "two-sided" && tails != "less" && tails != "greater" {
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("error"),
            &JsValue::from_str("Invalid test type"),
        );
        return obj.into();
    }

    let data = js_array_to_vector(column);

    let n = data.len() as f64;
    let mean = data.iter().sum::<f64>() / n;
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
    let std_dev = variance.sqrt();

    // Standard error of the mean
    let std_error = std_dev / n.sqrt();

    // t-statistic
    let t = (mean - mu0) / std_error;

    // Degrees of freedom
    let df = n - 1.0;

    // Create t-distribution
    let t_dist = StudentsT::new(0.0, 1.0, df).unwrap();

    // Calculate p-value using t-distribution
    let p = match tails.as_str() {
        "two-sided" => 2.0 * (1.0 - t_dist.cdf(t.abs())),
        "less" => t_dist.cdf(t),
        "greater" => 1.0 - t_dist.cdf(t),
        _ => 0.0,
    };

    let obj = Object::new();
    let _ = Reflect::set(&obj, &JsValue::from_str("t"), &JsValue::from_f64(t));
    let _ = Reflect::set(&obj, &JsValue::from_str("p"), &JsValue::from_f64(p));

    obj.into()
}

/// Performs a two-sample t-test.
///     
/// # Arguments
///
/// * `column1` - A reference to a JsValue representing the first JavaScript array.
/// * `column2` - A reference to a JsValue representing the second JavaScript array.
/// * `delta0` - A reference to a JsValue representing the hypothesized difference in means.
/// * `tails` - A reference to a JsValue indicating the type of test ("two-sided", "less", or "greater").
///
/// # Returns
///
/// * A JsValue representing the test statistic and p-value of the two-sample t-test.
#[wasm_bindgen]
pub fn two_samp_t_test(
    column1: &JsValue,
    column2: &JsValue,
    delta0: &JsValue,
    tails: &JsValue,
) -> JsValue {
    let d0 = delta0.as_f64().unwrap();
    let tails = tails.as_string().unwrap();

    let c1 = js_array_to_vector(column1);
    let c2 = js_array_to_vector(column2);

    let n1 = c1.len() as f64;
    let n2 = c2.len() as f64;

    let mean1 = c1.iter().sum::<f64>() / n1;
    let mean2 = c2.iter().sum::<f64>() / n2;

    let s1 = c1.iter().map(|x| (x - mean1).powi(2)).sum::<f64>() / (n1 - 1.0);
    let s2 = c2.iter().map(|x| (x - mean2).powi(2)).sum::<f64>() / (n2 - 1.0);

    let t = (mean1 - mean2 - d0) / f64::sqrt(s1 / n1 + s2 / n2);

    let df = (s1 / n1 + s2 / n2).powi(2)
        / ((s1 / n1).powi(2) / (n1 - 1.0) + (s2 / n2).powi(2) / (n2 - 1.0));

    let dist = StudentsT::new(0.0, 1.0, df).unwrap();

    let p = match tails.as_str() {
        "two-sided" => 2.0 * (1.0 - dist.cdf(t.abs())),
        "less" => dist.cdf(t),
        "greater" => 1.0 - dist.cdf(t),
        _ => 0.0,
    };

    let obj = Object::new();
    let _ = Reflect::set(&obj, &JsValue::from_str("t"), &JsValue::from_f64(t));
    let _ = Reflect::set(&obj, &JsValue::from_str("p"), &JsValue::from_f64(p));

    obj.into()
}

/// Performs a matched pairs t-test.
///
/// # Arguments
///
/// * `column1` - A reference to a JsValue representing the first JavaScript array.
/// * `column2` - A reference to a JsValue representing the second JavaScript array.
/// * `delta0` - A reference to a JsValue representing the difference between the two means.
/// * `tails` - A reference to a JsValue indicating the type of test ("two-sided", "less", or "greater").
///
/// # Returns
///
/// * A JsValue object containing the test statistic t and p-value p.
#[wasm_bindgen]
pub fn matched_pairs_t_test(
    column1: &JsValue,
    column2: &JsValue,
    delta0: &JsValue,
    tails: &JsValue,
) -> JsValue {
    let data = subtract_jsvalue_arrays(column1, column2);

    one_samp_t_test(&data, tails, delta0)
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
/// * A JsValue object containing the test statistic f and p-value p.
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

/// Computes the F-statistic and p-value for a linear regression test.
///
/// # Arguments
///
/// * `x` - A reference to a JsValue representing the independent variable.
/// * `y` - A reference to a JsValue representing the dependent variable.
///
/// # Returns
///
/// * An object with two properties: `f` and `p`, the F-statistic and p-value,
///   respectively.
#[wasm_bindgen]
pub fn regression_test(x: &JsValue, y: &JsValue) -> JsValue {
    let x_vec = js_array_to_vector(x);
    let y_vec = js_array_to_vector(y);

    let n = x_vec.len() as f64;

    let sxy = x_vec
        .iter()
        .zip(y_vec.iter())
        .map(|(x, y)| x * y)
        .sum::<f64>()
        - (1.0 / n) * x_vec.iter().sum::<f64>() * y_vec.iter().sum::<f64>();
    let sxx = x_vec.iter().map(|x| x.powi(2)).sum::<f64>()
        - (1.0 / n) * (x_vec.iter().sum::<f64>()).powi(2);
    let syy = y_vec.iter().map(|y| y.powi(2)).sum::<f64>()
        - (1.0 / n) * (y_vec.iter().sum::<f64>()).powi(2);

    let tss = syy;
    let ssr = sxy.powi(2) / sxx;
    let sse = tss - ssr;

    let df_tr = 1.0;
    let df_e = n - 2.0;

    let ms_tr = ssr / df_tr;
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
    use wasm_bindgen_test::*;

    use super::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[allow(unused)]
    #[wasm_bindgen_test]
    fn test_one_samp_z_test() {
        let column1 = vec_to_jsvalue(vec![1.0, 2.0, 3.0, 4.0, 5.0]);

        let result1 = one_samp_z_test(
            &column1,
            &JsValue::from_str("two-sided"),
            &JsValue::from_f64(0.0),
        );
        let result2 = one_samp_z_test(
            &column1,
            &JsValue::from_str("greater"),
            &JsValue::from_f64(0.0),
        );
        let result3 = one_samp_z_test(
            &column1,
            &JsValue::from_str("less"),
            &JsValue::from_f64(0.0),
        );

        let p1 = Reflect::get(&result1, &JsValue::from_str("p")).unwrap();
        let p2 = Reflect::get(&result2, &JsValue::from_str("p")).unwrap();
        let p3 = Reflect::get(&result3, &JsValue::from_str("p")).unwrap();

        assert!((p1.as_f64().unwrap() - 0.00002209).abs() < 0.01);
        assert!((p2.as_f64().unwrap() - 0.00001105).abs() < 0.01);
        assert!((p3.as_f64().unwrap() - 1.0).abs() < 0.01);
    }

    #[allow(unused)]
    #[wasm_bindgen_test]
    fn test_one_samp_t_test() {
        let column1 = vec_to_jsvalue(vec![1.0, 2.0, 3.0, 4.0, 5.0]);

        let result1 = one_samp_t_test(
            &column1,
            &JsValue::from_str("two-sided"),
            &JsValue::from_f64(0.0),
        );
        let result2 = one_samp_t_test(
            &column1,
            &JsValue::from_str("greater"),
            &JsValue::from_f64(0.0),
        );
        let result3 = one_samp_t_test(
            &column1,
            &JsValue::from_str("less"),
            &JsValue::from_f64(0.0),
        );

        let p1 = Reflect::get(&result1, &JsValue::from_str("p")).unwrap();
        let p2 = Reflect::get(&result2, &JsValue::from_str("p")).unwrap();
        let p3 = Reflect::get(&result3, &JsValue::from_str("p")).unwrap();

        assert!((p1.as_f64().unwrap() - 0.01324).abs() < 0.01);
        assert!((p2.as_f64().unwrap() - 0.006618).abs() < 0.01);
        assert!((p3.as_f64().unwrap() - 0.9934).abs() < 0.01);
    }

    #[allow(unused)]
    #[wasm_bindgen_test]
    fn test_two_samp_t_test() {
        let column1 = vec_to_jsvalue(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let column2 = vec_to_jsvalue(vec![2.0, 3.0, 4.0, 5.0, 6.0]);

        let result1 = two_samp_t_test(
            &column1,
            &column2,
            &JsValue::from_f64(0.0),
            &JsValue::from_str("two-sided"),
        );
        let result2 = two_samp_t_test(
            &column1,
            &column2,
            &JsValue::from_f64(0.0),
            &JsValue::from_str("greater"),
        );
        let result3 = two_samp_t_test(
            &column1,
            &column2,
            &JsValue::from_f64(0.0),
            &JsValue::from_str("less"),
        );

        let p1 = Reflect::get(&result1, &JsValue::from_str("p")).unwrap();
        let p2 = Reflect::get(&result2, &JsValue::from_str("p")).unwrap();
        let p3 = Reflect::get(&result3, &JsValue::from_str("p")).unwrap();

        assert!((p1.as_f64().unwrap() - 0.3466).abs() < 0.01);
        assert!((p2.as_f64().unwrap() - 0.8267).abs() < 0.01);
        assert!((p3.as_f64().unwrap() - 0.1733).abs() < 0.01);
    }

    #[allow(unused)]
    #[wasm_bindgen_test]
    fn test_matched_pairs_t_test() {
        let column1 = vec_to_jsvalue(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let column2 = vec_to_jsvalue(vec![2.0, 5.0, 4.0, 7.0, 9.0]);

        let result1 = matched_pairs_t_test(
            &column1,
            &column2,
            &JsValue::from_f64(0.0),
            &JsValue::from_str("two-sided"),
        );
        let result2 = matched_pairs_t_test(
            &column1,
            &column2,
            &JsValue::from_f64(0.0),
            &JsValue::from_str("greater"),
        );
        let result3 = matched_pairs_t_test(
            &column1,
            &column2,
            &JsValue::from_f64(0.0),
            &JsValue::from_str("less"),
        );

        let p1 = Reflect::get(&result1, &JsValue::from_str("p")).unwrap();
        let p2 = Reflect::get(&result2, &JsValue::from_str("p")).unwrap();
        let p3 = Reflect::get(&result3, &JsValue::from_str("p")).unwrap();

        assert!((p1.as_f64().unwrap() - 0.01613).abs() < 0.01);
        assert!((p2.as_f64().unwrap() - 0.9919).abs() < 0.01);
        assert!((p3.as_f64().unwrap() - 0.008065).abs() < 0.01);
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

        assert!((f.as_f64().unwrap() - 1.0).abs() < 0.01);
        assert!((p.as_f64().unwrap() - 0.3465).abs() < 0.01);
    }

    #[allow(unused)]
    #[wasm_bindgen_test]
    fn test_regression_test() {
        let column1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let column2 = vec![2.0, 30.0, 4.0, 50.0, 6.0];

        let x = vec_to_jsvalue(column1);
        let y = vec_to_jsvalue(column2);

        let result = regression_test(&x, &y);

        let p = Reflect::get(&result, &JsValue::from_str("p")).unwrap();
        let f = Reflect::get(&result, &JsValue::from_str("f")).unwrap();

        assert!((f.as_f64().unwrap() - 0.1396).abs() < 0.01);
        assert!((p.as_f64().unwrap() - 0.7335).abs() < 0.01);
    }
}
