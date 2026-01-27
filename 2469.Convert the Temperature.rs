impl Solution {
    /// Converts Celsius to Kelvin and Fahrenheit.
    ///
    /// # Intuition
    /// Direct application of temperature conversion formulas.
    ///
    /// # Approach
    /// 1. Kelvin = Celsius + 273.15
    /// 2. Fahrenheit = Celsius * 1.80 + 32.00
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        vec![celsius + 273.15, celsius * 1.80 + 32.00]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let result = Solution::convert_temperature(36.50);
        assert!((result[0] - 309.65).abs() < 1e-5);
        assert!((result[1] - 97.70).abs() < 1e-5);
    }

    #[test]
    fn test_example_2() {
        let result = Solution::convert_temperature(122.11);
        assert!((result[0] - 395.26).abs() < 1e-5);
        assert!((result[1] - 251.798).abs() < 1e-3);
    }

    #[test]
    fn test_zero() {
        let result = Solution::convert_temperature(0.0);
        assert!((result[0] - 273.15).abs() < 1e-5);
        assert!((result[1] - 32.0).abs() < 1e-5);
    }
}
