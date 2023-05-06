use std::fmt::Debug;

use inquire::validator::Validation;
use inquire::CustomType;

mod tests;

pub struct Weight(f64);

pub struct Height(f64);

#[derive(Debug)]
pub struct Bmi {
    bmi: f64,
    conclusion: BMIConclusion,
}

#[derive(Debug)]
enum BMIConclusion {
    Underweight,
    Normal,
    Overweight,
    Obese,
    MorbidObese,
}

#[derive(Debug, PartialEq)]
pub enum BmiError {
    HeightCannotBeZeroOrNegative,
    WeightCannotBeZeroOrNegative,
}

fn main() {
    let weight = Weight(
        CustomType::<f64>::new("Please input your weight in kg: ")
            .with_formatter(&|i| format!("{:}kg", i))
            .with_error_message("Please type a valid number")
            .with_validator(|val: &f64| {
                if *val <= 0.0f64 {
                    Ok(Validation::Invalid(
                        "You should weigh a positive number".into(),
                    ))
                } else {
                    Ok(Validation::Valid)
                }
            })
            .prompt()
            .unwrap_or_else(|err| {
                println!("Error while parsing: {}", err);
                println!("Using standard values for a male");
                85.2
            }),
    );
    let height = Height(
        CustomType::<f64>::new("Please input your height in kg: ")
            .with_formatter(&|i| format!("{:}m", i))
            .with_error_message("Please type a valid number")
            .with_validator(|val: &f64| {
                if *val <= 0.0f64 {
                    Ok(Validation::Invalid(
                        "You should be a positive number high".into(),
                    ))
                } else {
                    Ok(Validation::Valid)
                }
            })
            .prompt()
            .unwrap_or_else(|err| {
                println!("Error while parsing: {}", err);
                println!("Using standard values for a male");
                1.8
            }),
    );
    let bmi = calculate_bmi(height, weight);

    match bmi {
        Ok(bmi) => println!(
            "Your BMI is {} and your are {:?}, congrats?",
            bmi.bmi, bmi.conclusion
        ),
        Err(e) => println!("Error while calculating: {e:?}"),
    }
}

pub fn calculate_bmi(height: Height, weight: Weight) -> Result<Bmi, BmiError> {
    if height.0 <= 0.0 {
        Err(BmiError::HeightCannotBeZeroOrNegative)
    } else if weight.0 <= 0.0 {
        Err(BmiError::WeightCannotBeZeroOrNegative)
    } else {
        let bmi_number = weight.0 / height.0.powf(2f64);

        Ok(Bmi {
            bmi: bmi_number,
            conclusion: match bmi_number {
                number if number < 19.0 => BMIConclusion::Underweight,
                number if (19.0..25.0).contains(&number) => BMIConclusion::Normal,
                number if (25.0..30.0).contains(&number) => BMIConclusion::Overweight,
                number if (30.0..35.0).contains(&number) => BMIConclusion::Obese,
                _ => BMIConclusion::MorbidObese,
            },
        })
    }
}
