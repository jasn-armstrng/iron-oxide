pub fn temperature_converter(value: f32, from: char, to: char) -> f32 {
    // Utility:
    //  - Convert between temperature units
    // Inputs:
    //  - value: The floating point value to be converted
    //  - from: The unit the value is to be converted from. Case insensitive char - c/C, f/F, k/K for Celsius, Fahrenheit, and Kelvin respectively
    //  - to:  he unit the value is to be converted to. ...
    // Returns:
    //  - A floating point value that is greater than or equal to the absolute-zero value for that resulting unit

    // If `from` eq `to`, return value
    if from == to {
        return value;
    }

    // Match and return conversion for value
    match (from.to_ascii_lowercase(), to.to_ascii_lowercase()) {
        // Celsius to Fahrenheit
        ('c', 'f') => {
            // Compare conversion with absolute-zero before return
            let result = (value * 9.0 / 5.0) + 32.0;
            if result < -459.67 {
                -459.67
            } else {
                result
            }
        }
        // Fahrenheit to Celsius
        ('f', 'c') => {
            // Compare conversion with absolute-zero before return
            let result = (value - 32.0) * 5.0 / 9.0;
            if result < -273.15 {
                -273.15
            } else {
                result
            }
        }
        // Fahrenheit to Kelvin
        ('f', 'k') => {
            let result = (value - 32.0) * 5.0 / 9.0 + 273.15;
            if result < 0.0 {
                0.0
            } else {
                result
            }
        }
        // Kelvin to Fahrenheit
        ('k', 'f') => {
            let result = value * 9.0 / 5.0 - 459.67;
            if result < -459.67 {
                -459.67
            } else {
                result
            }
        }
        // Celsius to Kelvin
        ('c', 'k') => {
            let result = value + 273.15;
            if result < 0.0 {
                0.0
            } else {
                result
            }
        }
        // Kelvin to Celsius
        ('k', 'c') => {
            let result = value - 273.15;
            if result < -273.15 {
                -273.15
            } else {
                result
            }
        }
        // Return an absurd value for non-matches
        _ => -999999.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 0.01;

    fn assert_almost_eq(a: f32, b: f32) {
        let relative_epsilon = 0.0001; // 0.01% tolerance
        let diff = (a - b).abs();
        let max = a.abs().max(b.abs());

        assert!(
            diff < max * relative_epsilon || diff < EPSILON,
            "Expected {} to be approximately equal to {}",
            a,
            b
        );
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_almost_eq(temperature_converter(0.0, 'c', 'f'), 32.0);
        assert_almost_eq(temperature_converter(100.0, 'C', 'F'), 212.0);
        assert_almost_eq(temperature_converter(-40.0, 'c', 'F'), -40.0);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_almost_eq(temperature_converter(32.0, 'f', 'c'), 0.0);
        assert_almost_eq(temperature_converter(212.0, 'F', 'C'), 100.0);
        assert_almost_eq(temperature_converter(-40.0, 'F', 'c'), -40.0);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        assert_almost_eq(temperature_converter(0.0, 'c', 'k'), 273.15);
        assert_almost_eq(temperature_converter(-273.15, 'C', 'K'), 0.0);
        assert_almost_eq(temperature_converter(100.0, 'c', 'K'), 373.15);
    }

    #[test]
    fn test_kelvin_to_celsius() {
        assert_almost_eq(temperature_converter(273.15, 'k', 'c'), 0.0);
        assert_almost_eq(temperature_converter(0.0, 'K', 'C'), -273.15);
        assert_almost_eq(temperature_converter(373.15, 'K', 'c'), 100.0);
    }

    #[test]
    fn test_fahrenheit_to_kelvin() {
        assert_almost_eq(temperature_converter(32.0, 'f', 'k'), 273.15);
        assert_almost_eq(temperature_converter(-459.67, 'F', 'K'), 0.0);
        assert_almost_eq(temperature_converter(212.0, 'f', 'K'), 373.15);
    }

    #[test]
    fn test_kelvin_to_fahrenheit() {
        assert_almost_eq(temperature_converter(273.15, 'k', 'f'), 32.0);
        assert_almost_eq(temperature_converter(0.0, 'K', 'F'), -459.67);
        assert_almost_eq(temperature_converter(373.15, 'K', 'F'), 212.0);
    }

    #[test]
    fn test_same_unit_conversion() {
        assert_almost_eq(temperature_converter(42.0, 'c', 'c'), 42.0);
        assert_almost_eq(temperature_converter(42.0, 'f', 'f'), 42.0);
        assert_almost_eq(temperature_converter(42.0, 'k', 'k'), 42.0);
    }

    #[test]
    fn test_case_insensitivity() {
        // Test that function handles uppercase and lowercase units the same
        let temp = 42.0;
        assert_almost_eq(
            temperature_converter(temp, 'c', 'f'),
            temperature_converter(temp, 'C', 'F'),
        );
        assert_almost_eq(
            temperature_converter(temp, 'f', 'k'),
            temperature_converter(temp, 'F', 'K'),
        );
        assert_almost_eq(
            temperature_converter(temp, 'k', 'c'),
            temperature_converter(temp, 'K', 'C'),
        );
    }
}
