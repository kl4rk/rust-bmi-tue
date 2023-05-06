#[cfg(test)]
mod test {
    use crate::{calculate_bmi, BmiError, Height, Weight};
    use float_eq::assert_float_eq;
    #[test]
    fn test_height_zero_err() {
        let bmi_err = calculate_bmi(Height(0.0), Weight(78.0)).unwrap_err();
        assert_eq!(bmi_err, BmiError::HeightCannotBeZeroOrNegative);
    }
    #[test]
    fn test_weight_zero_err() {
        let bmi_err = calculate_bmi(Height(1.0), Weight(0.0)).unwrap_err();
        assert_eq!(bmi_err, BmiError::WeightCannotBeZeroOrNegative);
    }

    #[test]
    fn test_result() {
        let bmi = calculate_bmi(Height(1.84), Weight(78.0)).unwrap();
        assert_float_eq!(bmi.bmi, 23.03, abs <= 0.15);
    }
}
