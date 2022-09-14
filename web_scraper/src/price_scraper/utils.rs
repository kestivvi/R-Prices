use std::num::ParseFloatError;

pub fn string_to_float(s: &str) -> error_stack::Result<f64, ParseFloatError> {
    let mut first_dot = true;
    s.chars()
        .into_iter()
        .map(|c| if c == ',' { '.' } else { c })
        // TODO: Try doing it with Iterator::scan()
        .filter(|&c| {
            let first_occurence_of_dot = c == '.' && first_dot;
            if first_occurence_of_dot {
                first_dot = false;
            }
            c.is_numeric() || first_occurence_of_dot
        })
        .collect::<String>()
        .parse::<f64>()
        .map_err(|error| {
            error_stack::report!(error)
                .attach_printable(format!("Cannot parse string to float. String: {}", s))
        })
}
