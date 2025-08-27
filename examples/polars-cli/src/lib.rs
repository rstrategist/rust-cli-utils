/*
 * This program reads the Iris dataset from a CSV file located at "src/data/iris.csv".
 * It assumes the CSV has headers.
 *
 * 1. Filtering:
 *    It filters the rows where the column 'sepal_length' is greater than 5.0.
 *
 * 2. Grouping:
 *    The program then groups the data by the 'species' column.
 *
 * 3. Aggregation:
 *    For each group (species in this case), it calculates the sum and mean of the other columns.
 *
 * Output Explanation:
 *    The resulting DataFrame has a shape of (3, 9), indicating that it has 3 rows and 9 columns.
 *    The rows represent the three species of Iris flowers: 'setosa', 'versicolor', and 'virginica'.
 *    The columns represent:
 *    - 'species': The name of the Iris species
 *    - 'sepal_length_sum': The sum of 'sepal_length' values for each species
 *    - 'sepal_width_sum': The sum of 'sepal_width' values for each species
 *    - 'petal_length_sum': The sum of 'petal_length' values for each species
 *    - 'petal_width_sum': The sum of 'petal_width' values for each species
 *    - 'sepal_length_mean': The mean of 'sepal_length' values for each species
 *    - 'sepal_width_mean': The mean of 'sepal_width' values for each species
 *    - 'petal_length_mean': The mean of 'petal_length' values for each species
 *    - 'petal_width_mean': The mean of 'petal_width' values for each species
 *
 *    For example, for 'virginica', the sum and mean of 'sepal_length' values are shown,
 *    as well as for the other columns.
 */

// Import necessary modules from the `polars` crate
use polars::prelude::*;

// Define the main function that returns a Result type.
// If everything is Ok, it returns `()`, otherwise it returns a `PolarsError`.
pub fn calculate() -> Result<DataFrame, PolarsError> {
    // Read the CSV file located at "src/data/iris.csv"
    // Assume the CSV has headers.
    let df = LazyCsvReader::new("data/iris.csv")
        .has_header(true)
        .finish()?
        // Filter the rows where the column "sepal_length" is greater than 5.0
        .filter(col("sepal_length").gt(lit(5.0)))
        // Group the DataFrame by the "species" column
        .groupby(vec![col("species")])
        // Aggregate the groups by summing and averaging the specified columns for each group
        .agg(&[
            col("sepal_length").sum().alias("sepal_length_sum"),
            col("sepal_width").sum().alias("sepal_width_sum"),
            col("petal_length").sum().alias("petal_length_sum"),
            col("petal_width").sum().alias("petal_width_sum"),
            col("sepal_length").mean().alias("sepal_length_mean"),
            col("sepal_width").mean().alias("sepal_width_mean"),
            col("petal_length").mean().alias("petal_length_mean"),
            col("petal_width").mean().alias("petal_width_mean"),
        ])
        // Trigger computation and collect the result into a DataFrame
        .collect()?;

    // Print the resulting DataFrame
    println!("{}", df);

    // Return Ok to signify that the program has executed successfully
    Ok(df)
}
