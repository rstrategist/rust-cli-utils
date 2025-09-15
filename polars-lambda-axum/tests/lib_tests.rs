// tests/lib_tests.rs

use polars_lambda_axum::{calculate, http_handler::polars_demo_compute};

#[test]
fn test_calculate_filters_and_groups() {
    let df = calculate(4.8).expect("calculate should succeed");

    // The DataFrame should have grouped by "species"
    assert!(df.get_column_names().contains(&"species"));
    assert!(df.get_column_names().contains(&"sepal_length_sum"));

    // Filter > 4.8 should exclude the lowest rows
    let sepal_lengths = df.column("sepal_length_sum").unwrap();
    assert!(sepal_lengths.len() > 0);

    // Ensure we actually filtered (i.e., sums are above some minimum)
    let min_sum = sepal_lengths.i64().unwrap().min().unwrap();
    assert!(min_sum > 10);
}

#[test]
fn test_polars_demo_compute_json_output() {
    let json_str = polars_demo_compute().expect("should succeed");
    assert!(json_str.contains("\"a\""));
    assert!(json_str.contains("\"b\""));
    assert!(json_str.contains("\"sum\"") == false); // we didn’t add sum here
}
