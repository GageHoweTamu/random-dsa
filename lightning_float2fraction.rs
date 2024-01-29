// this has probably been done before, but i reinvented it
// approximates a float as a fraction, given an "accuracy_score" like 0.00001
//                                                              (expected/actual will have less than 0.00001 error)
// // https://github.com/GageHoweTamu/decimal-fraction-approximator

fn approximate_float(input_value: f64, accuracy_score: f64) -> (i64, i64, i64) {

    let (whole_part, fractional_part) = separate_parts(input_value);

    let mut denominator = 1;
    let mut numerator = (fractional_part * denominator as f64).round() as i64;

    // Main approximation loop
    while (fractional_part - (numerator as f64 / denominator as f64)).abs() > accuracy_score {
        denominator += 1;
        numerator = (fractional_part * denominator as f64).round() as i64;
    }

    (whole_part, numerator, denominator)
}
