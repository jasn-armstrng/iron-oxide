/// Temperature conversion suite
const ABSOLUTE_ZERO_C: f32 = -273.15;
const ABSOLUTE_ZERO_F: f32 = -459.67;
const ABSOLUTE_ZERO_K: f32 = 0.0;

/// Convert a celsius temperature to fahrenheit.
///
/// # Conversion formula
/// (C * 9.0 / 5.0) + 32.0
///
/// # Return
/// A floating point fahrenheit value that is greater than or equal to the absolute-zero (-459.67).
pub fn celsius_to_fahrenheit(c: f32) -> f32 {
    if c <= ABSOLUTE_ZERO_C {
        ABSOLUTE_ZERO_F
    } else {
        (c * 9.0 / 5.0) + 32.0
    }
}

/// Convert a celsius temperature to kelvin.
///
/// # Conversion formula
/// C + 273.15
///
/// # Return
/// A floating point kelvin value that is greater than or equal to the absolute-zero (0.0).
pub fn celsius_to_kelvin(c: f32) -> f32 {
    if c <= ABSOLUTE_ZERO_C {
        ABSOLUTE_ZERO_K
    } else {
        c + 273.15
    }
}

/// Convert a fahrenheit temperature to celsius.
///
/// # Conversion formula
/// (F - 32.0) * 5.0 / 9.0
///
/// # Return
/// A floating point celsius value that is greater than or equal to the absolute-zero (-273.15).
pub fn fahrenheit_to_celsius(f: f32) -> f32 {
    if f <= ABSOLUTE_ZERO_F {
        ABSOLUTE_ZERO_C
    } else {
        (f - 32.0) * 5.0 / 9.0
    }
}

/// Convert a fahrenheit temperature to kelvin.
///
/// # Conversion formula
/// (F - 32.0) * 5.0 / 9.0 + 273.15
///
/// # Return
/// A floating point kelvin value that is greater than or equal to the absolute-zero (0.0).
pub fn fahrenheit_to_kelvin(f: f32) -> f32 {
    if f <= ABSOLUTE_ZERO_F {
        ABSOLUTE_ZERO_K
    } else {
        (f - 32.0) * 5.0 / 9.0 + 273.15
    }
}

/// Convert a kelvin temperature to celsius.
///
/// # Conversion formula
/// K - 273.15
///
/// # Return
/// A floating point celsius value that is greater than or equal to the absolute-zero (-273.15).
pub fn kelvin_to_celsius(k: f32) -> f32 {
    if k <= ABSOLUTE_ZERO_K {
        ABSOLUTE_ZERO_C
    } else {
        k - 273.15
    }
}

/// Convert a kelvin temperature to fahrenheit.
///
/// # Conversion formula
/// K * 9.0 / 5.0 - 459.67
///
/// # Return
/// A floating point fahrenheit celsius value that is greater than or equal to the absolute-zero (-459.67).
pub fn kelvin_to_fahrenheit(k: f32) -> f32 {
    if k <= ABSOLUTE_ZERO_K {
        ABSOLUTE_ZERO_F
    } else {
        k * 9.0 / 5.0 - 459.67
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
        assert_almost_eq(celsius_to_fahrenheit(ABSOLUTE_ZERO_C), ABSOLUTE_ZERO_F);
        assert_almost_eq(
            celsius_to_fahrenheit(ABSOLUTE_ZERO_C - 23.04),
            ABSOLUTE_ZERO_F,
        );
        assert_almost_eq(celsius_to_fahrenheit(100.0), 212.0);
        assert_almost_eq(celsius_to_fahrenheit(55.0), 131.0);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_almost_eq(fahrenheit_to_celsius(ABSOLUTE_ZERO_F), ABSOLUTE_ZERO_C);
        assert_almost_eq(
            fahrenheit_to_celsius(ABSOLUTE_ZERO_F - 16.0),
            ABSOLUTE_ZERO_C,
        );
        assert_almost_eq(fahrenheit_to_celsius(212.0), 100.0);
        assert_almost_eq(fahrenheit_to_celsius(131.0), 55.0);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        assert_almost_eq(celsius_to_kelvin(ABSOLUTE_ZERO_C), ABSOLUTE_ZERO_K);
        assert_almost_eq(celsius_to_kelvin(ABSOLUTE_ZERO_C - 9.222), ABSOLUTE_ZERO_K);
        assert_almost_eq(celsius_to_kelvin(0.0), 273.15);
        assert_almost_eq(celsius_to_kelvin(-273.15), 0.0);
        assert_almost_eq(celsius_to_kelvin(100.0), 373.15);
    }

    #[test]
    fn test_kelvin_to_celsius() {
        assert_almost_eq(kelvin_to_celsius(ABSOLUTE_ZERO_K), ABSOLUTE_ZERO_C);
        assert_almost_eq(kelvin_to_celsius(ABSOLUTE_ZERO_K - 889.0), ABSOLUTE_ZERO_C);
        assert_almost_eq(kelvin_to_celsius(273.15), 0.0);
        assert_almost_eq(kelvin_to_celsius(0.0), -273.15);
        assert_almost_eq(kelvin_to_celsius(373.15), 100.0);
    }

    #[test]
    fn test_fahrenheit_to_kelvin() {
        assert_almost_eq(fahrenheit_to_kelvin(ABSOLUTE_ZERO_F), ABSOLUTE_ZERO_K);
        assert_almost_eq(
            fahrenheit_to_kelvin(ABSOLUTE_ZERO_F - 10.9),
            ABSOLUTE_ZERO_K,
        );
        assert_almost_eq(fahrenheit_to_kelvin(32.0), 273.15);
        assert_almost_eq(fahrenheit_to_kelvin(-459.67), 0.0);
        assert_almost_eq(fahrenheit_to_kelvin(212.0), 373.15);
    }

    #[test]
    fn test_kelvin_to_fahrenheit() {
        assert_almost_eq(kelvin_to_fahrenheit(ABSOLUTE_ZERO_K), ABSOLUTE_ZERO_F);
        assert_almost_eq(
            kelvin_to_fahrenheit(ABSOLUTE_ZERO_K - 93.3),
            ABSOLUTE_ZERO_F,
        );
        assert_almost_eq(kelvin_to_fahrenheit(273.15), 32.0);
        assert_almost_eq(kelvin_to_fahrenheit(0.0), -459.67);
        assert_almost_eq(kelvin_to_fahrenheit(373.15), 212.0);
    }
}
