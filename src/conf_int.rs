use crate::utils::*;
use js_sys::Array;
use statrs::distribution::{ContinuousCDF, FisherSnedecor, Normal, StudentsT};
use wasm_bindgen::prelude::*;

/// Calculates a one-sample Z-interval (confidence interval for population mean)
/// using the standard normal distribution.
///
/// # Arguments
/// * `column` - A JavaScript array of numerical values representing the sample
/// * `alpha` - The significance level (e.g., 0.05 for a 95% confidence interval)
///
/// # Returns
/// A JavaScript array containing:
/// - Lower bound of the confidence interval
/// - Upper bound of the confidence interval
#[wasm_bindgen]
pub fn one_samp_z_interval(column: &JsValue, alpha: &JsValue) -> JsValue {
    let alpha = alpha.as_f64().unwrap_or(0.05);
    let data = js_array_to_vector(column);

    if data.is_empty() {
        return JsValue::NULL;
    }

    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (data.len() - 1) as f64;
    let std_dev = variance.sqrt();

    let z_score = Normal::new(0.0, 1.0)
        .unwrap()
        .inverse_cdf(1.0 - alpha / 2.0);
    let moe = z_score * (std_dev / (data.len() as f64).sqrt());

    let lower = mean - moe;
    let upper = mean + moe;

    let arr = Array::new();
    arr.set(0, JsValue::from_f64(lower));
    arr.set(1, JsValue::from_f64(upper));

    arr.into()
}

/// Calculates a two-sample Z-interval (confidence interval for difference between two
/// population means)
/// using the standard normal distribution.
///
/// # Arguments
/// * `column1` - A JavaScript array of numerical values for the first sample
/// * `column2` - A JavaScript array of numerical values for the second sample
/// * `alpha` - The significance level (e.g., 0.05 for a 95% confidence interval)
///
/// # Returns
/// A JavaScript array containing:
/// - Lower bound of the confidence interval
/// - Upper bound of the confidence interval
#[wasm_bindgen]
pub fn two_samp_z_interval(column1: &JsValue, column2: &JsValue, alpha: &JsValue) -> JsValue {
    let alpha = alpha.as_f64().unwrap_or(0.05);
    let data1 = js_array_to_vector(column1);
    let data2 = js_array_to_vector(column2);

    if data1.is_empty() || data2.is_empty() {
        return JsValue::NULL;
    }

    let mean1 = data1.iter().sum::<f64>() / data1.len() as f64;
    let mean2 = data2.iter().sum::<f64>() / data2.len() as f64;

    let var1 = data1.iter().map(|x| (x - mean1).powi(2)).sum::<f64>() / (data1.len() - 1) as f64;
    let var2 = data2.iter().map(|x| (x - mean2).powi(2)).sum::<f64>() / (data2.len() - 1) as f64;

    let std_dev1 = var1.sqrt();
    let std_dev2 = var2.sqrt();

    let z_score = Normal::new(0.0, 1.0)
        .unwrap()
        .inverse_cdf(1.0 - alpha / 2.0);

    let pooled_se =
        ((std_dev1.powi(2) / data1.len() as f64) + (std_dev2.powi(2) / data2.len() as f64)).sqrt();

    let diff_mean = mean1 - mean2;
    let margin_of_error = z_score * pooled_se;

    let lower = diff_mean - margin_of_error;
    let upper = diff_mean + margin_of_error;

    let arr = Array::new();
    arr.set(0, JsValue::from_f64(lower));
    arr.set(1, JsValue::from_f64(upper));

    arr.into()
}

/// Calculates a one-sample T-interval (confidence interval for population mean) using
/// the Student's t-distribution.
///
/// # Arguments
/// * `column` - A JavaScript array of numerical values representing the sample
/// * `alpha` - The significance level (e.g., 0.05 for a 95% confidence interval)
///
/// # Returns
/// A JavaScript array containing:
/// - Lower bound of the confidence interval
/// - Upper bound of the confidence interval
#[wasm_bindgen]
pub fn one_samp_t_interval(column: &JsValue, alpha: &JsValue) -> JsValue {
    let alpha = alpha.as_f64().unwrap_or(0.05);
    let data = js_array_to_vector(column);

    if data.is_empty() {
        return JsValue::NULL;
    }

    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (data.len() - 1) as f64;
    let std_dev = variance.sqrt();

    // Degrees of freedom
    let df = (data.len() - 1) as f64;

    // Use t-distribution instead of normal distribution
    let t_dist = StudentsT::new(0.0, 1.0, df).unwrap();
    let t_score = t_dist.inverse_cdf(1.0 - alpha / 2.0);

    let margin_of_error = t_score * (std_dev / (data.len() as f64).sqrt());

    let lower = mean - margin_of_error;
    let upper = mean + margin_of_error;

    let arr = Array::new();
    arr.set(0, JsValue::from_f64(lower));
    arr.set(1, JsValue::from_f64(upper));

    arr.into()
}

/// Calculates a two-sample T-interval (confidence interval for difference between
/// two population means)
/// using Welch's t-test approximation.
///
/// # Arguments
/// * `column1` - A JavaScript array of numerical values for the first sample
/// * `column2` - A JavaScript array of numerical values for the second sample
/// * `alpha` - The significance level (e.g., 0.05 for a 95% confidence interval)
///
/// # Returns
/// A JavaScript array containing:
/// - Lower bound of the confidence interval
/// - Upper bound of the confidence interval
#[wasm_bindgen]
pub fn two_samp_t_interval(column1: &JsValue, column2: &JsValue, alpha: &JsValue) -> JsValue {
    let alpha = alpha.as_f64().unwrap_or(0.05);
    let data1 = js_array_to_vector(column1);
    let data2 = js_array_to_vector(column2);

    if data1.is_empty() || data2.is_empty() {
        return JsValue::NULL;
    }

    let mean1 = data1.iter().sum::<f64>() / data1.len() as f64;
    let mean2 = data2.iter().sum::<f64>() / data2.len() as f64;

    let var1 = data1.iter().map(|x| (x - mean1).powi(2)).sum::<f64>() / (data1.len() - 1) as f64;
    let var2 = data2.iter().map(|x| (x - mean2).powi(2)).sum::<f64>() / (data2.len() - 1) as f64;

    let std_dev1 = var1.sqrt();
    let std_dev2 = var2.sqrt();

    // Welch's t-test degrees of freedom approximation
    let se1 = std_dev1.powi(2) / data1.len() as f64;
    let se2 = std_dev2.powi(2) / data2.len() as f64;
    let df = (se1 + se2).powi(2)
        / ((se1.powi(2) / ((data1.len() - 1) as f64)) + (se2.powi(2) / ((data2.len() - 1) as f64)));

    // Use t-distribution
    let t_dist = StudentsT::new(0.0, 1.0, df).unwrap();
    let t_score = t_dist.inverse_cdf(1.0 - alpha / 2.0);

    let pooled_se = (se1 + se2).sqrt();
    let diff_mean = mean1 - mean2;
    let margin_of_error = t_score * pooled_se;

    let lower = diff_mean - margin_of_error;
    let upper = diff_mean + margin_of_error;

    let arr = Array::new();
    arr.set(0, JsValue::from_f64(lower));
    arr.set(1, JsValue::from_f64(upper));

    arr.into()
}

/// Calculates a two-sample variance interval
/// (confidence interval for the ratio of population variances).
///
/// # Arguments
/// * `column1` - A JavaScript array of numerical values for the first sample
/// * `column2` - A JavaScript array of numerical values for the second sample
/// * `alpha` - The significance level (e.g., 0.05 for a 95% confidence interval)
///
/// # Returns
/// A JavaScript array containing:
/// - Lower bound of the confidence interval
/// - Upper bound of the confidence interval
#[wasm_bindgen]
pub fn two_samp_var_interval(column1: &JsValue, column2: &JsValue, alpha: &JsValue) -> JsValue {
    let alpha = alpha.as_f64().unwrap_or(0.05);
    let data1 = js_array_to_vector(column1);
    let data2 = js_array_to_vector(column2);

    if data1.is_empty() || data2.is_empty() {
        return JsValue::NULL;
    }

    let mean1 = data1.iter().sum::<f64>() / data1.len() as f64;
    let mean2 = data2.iter().sum::<f64>() / data2.len() as f64;

    let var1 = data1.iter().map(|x| (x - mean1).powi(2)).sum::<f64>() / (data1.len() - 1) as f64;
    let var2 = data2.iter().map(|x| (x - mean2).powi(2)).sum::<f64>() / (data2.len() - 1) as f64;

    // Compute F-distribution parameters
    let df1 = data1.len() - 1;
    let df2 = data2.len() - 1;

    // Compute the F-statistic (larger variance / smaller variance)
    let f_statistic = var1.max(var2) / var1.min(var2);

    // Compute critical F-values for confidence interval
    let f_dist = FisherSnedecor::new(df1 as f64, df2 as f64).unwrap();
    let f_lower = f_dist.inverse_cdf(alpha / 2.0);
    let f_upper = f_dist.inverse_cdf(1.0 - alpha / 2.0);

    // Compute confidence interval for variance ratio
    let lower = f_statistic / f_upper;
    let upper = f_statistic / f_lower;

    let arr = Array::new();
    arr.set(0, JsValue::from_f64(lower));
    arr.set(1, JsValue::from_f64(upper));

    arr.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::vec_to_jsvalue;
    use wasm_bindgen_test::*;

    #[allow(unused)]
    #[wasm_bindgen_test]
    fn test_one_samp_z_interval() {
        let data = vec_to_jsvalue(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let alpha = JsValue::from_f64(0.05);

        let result = one_samp_z_interval(&data, &alpha);
        let result_arr: Array = result.into();

        assert_eq!(
            result_arr.length(),
            2,
            "Should return lower and upper bounds"
        );

        let lower_bound: f64 = result_arr.get(0).as_f64().unwrap();
        let upper_bound: f64 = result_arr.get(1).as_f64().unwrap();

        assert!(
            lower_bound < upper_bound,
            "Lower bound should be less than upper bound"
        );

        assert!(
            (lower_bound - 1.6141).abs() < 0.01,
            "Lower bound should be 1.6141, not {}",
            lower_bound
        );

        assert!(
            (upper_bound - 4.3859).abs() < 0.01,
            "Upper bound should be 4.3859, not {}",
            upper_bound
        );
    }

    #[allow(unused)]
    #[wasm_bindgen_test]
    fn test_two_samp_z_interval() {
        let data1 = vec_to_jsvalue(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let data2 = vec_to_jsvalue(vec![2.0, 3.0, 4.0, 5.0, 6.0]);
        let alpha = JsValue::from_f64(0.05);

        let result = two_samp_z_interval(&data1, &data2, &alpha);
        let result_arr: Array = result.into();

        assert_eq!(
            result_arr.length(),
            2,
            "Should return lower and upper bounds"
        );

        let lower_bound: f64 = result_arr.get(0).as_f64().unwrap();
        let upper_bound: f64 = result_arr.get(1).as_f64().unwrap();

        assert!(
            lower_bound < upper_bound,
            "Lower bound should be less than upper bound"
        );

        assert!(
            (lower_bound + 2.959964).abs() < 0.01,
            "Lower bound should be 2.959964, not {}",
            lower_bound
        );

        assert!(
            (upper_bound - 0.959964).abs() < 0.01,
            "Upper bound should be 0.959964, not {}",
            upper_bound
        )
    }

    #[allow(unused)]
    #[wasm_bindgen_test]
    fn test_one_samp_t_interval() {
        let data = vec_to_jsvalue(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let alpha = JsValue::from_f64(0.05);

        let result = one_samp_t_interval(&data, &alpha);
        let result_arr: Array = result.into();

        assert_eq!(
            result_arr.length(),
            2,
            "Should return lower and upper bounds"
        );

        let lower_bound: f64 = result_arr.get(0).as_f64().unwrap();
        let upper_bound: f64 = result_arr.get(1).as_f64().unwrap();

        assert!(
            lower_bound < upper_bound,
            "Lower bound should be less than upper bound"
        );

        assert!(
            (lower_bound - 1.036757).abs() < 0.01,
            "Lower bound should be 1.036757, not {}",
            lower_bound
        );

        assert!(
            (upper_bound - 4.963243).abs() < 0.01,
            "Upper bound should be 4.963243, not {}",
            upper_bound
        )
    }

    #[allow(unused)]
    #[wasm_bindgen_test]
    fn test_two_samp_t_interval() {
        let data1 = vec_to_jsvalue(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let data2 = vec_to_jsvalue(vec![2.0, 3.0, 4.0, 5.0, 6.0]);
        let alpha = JsValue::from_f64(0.05);

        let result = two_samp_t_interval(&data1, &data2, &alpha);
        let result_arr: Array = result.into();

        assert_eq!(
            result_arr.length(),
            2,
            "Should return lower and upper bounds"
        );

        let lower_bound: f64 = result_arr.get(0).as_f64().unwrap();
        let upper_bound: f64 = result_arr.get(1).as_f64().unwrap();

        assert!(
            lower_bound < upper_bound,
            "Lower bound should be less than upper bound"
        );

        assert!(
            (lower_bound + 3.306004).abs() < 0.01,
            "Lower bound should be -3.306004, not {}",
            lower_bound
        );

        assert!(
            (upper_bound - 1.306004).abs() < 0.01,
            "Upper bound should be 1.306004, not {}",
            upper_bound
        )
    }

    #[allow(unused)]
    #[wasm_bindgen_test]
    fn test_two_samp_var_interval() {
        let data1 = vec_to_jsvalue(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let data2 = vec_to_jsvalue(vec![2.0, 3.0, 4.0, 5.0, 6.0]);
        let alpha = JsValue::from_f64(0.05);

        let result = two_samp_var_interval(&data1, &data2, &alpha);
        let result_arr: Array = result.into();

        assert_eq!(
            result_arr.length(),
            2,
            "Should return lower and upper bounds"
        );

        let lower_bound: f64 = result_arr.get(0).as_f64().unwrap();
        let upper_bound: f64 = result_arr.get(1).as_f64().unwrap();

        assert!(
            lower_bound < upper_bound,
            "Lower bound should be less than upper bound"
        );

        assert!(
            (lower_bound - 0.1041175).abs() < 0.01,
            "Lower bound should be 0.1041175, not {}",
            lower_bound
        );

        assert!(
            (upper_bound - 9.60453).abs() < 0.01,
            "Upper bound should be 9.60453, not {}",
            upper_bound
        )
    }
}
