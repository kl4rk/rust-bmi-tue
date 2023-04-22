use std::fmt::Debug;
use std::io::Write;
use std::str::FromStr;

struct Weight {
    weight: f64,
}

struct Height {
    height: f64,
}

#[derive(Debug)]
struct BMI {
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

fn main() {
    let stdin = std::io::stdin();
    print!("Please input your weight in kg: ");

    let _ = std::io::stdout().flush();

    let mut buffer_weight = String::new();
    stdin
        .read_line(&mut buffer_weight)
        .unwrap_or_else(|err| panic!("Error while parsing: {}", err));

    let weight = Weight {
        weight: f64::from_str(&buffer_weight.trim()).unwrap_or_else(|err| {
            println!("Error while parsing: {}", err);
            println!("Using standard values for a male");
            85.2
        }),
    };
    println!("Weight is {}", weight.weight);

    print!("Please input your height in meters: ");

    let _ = std::io::stdout().flush();

    let mut buffer_height = String::new();
    stdin
        .read_line(&mut buffer_height)
        .unwrap_or_else(|err| panic!("Error while parsing: {}", err));

    let height = Height {
        height: f64::from_str(&buffer_height.trim()).unwrap_or_else(|err| {
            println!("Error while parsing: {}", err);
            println!("Using standard values for a male");
            1.8
        }),
    };
    println!("Weight is {}", height.height);

    let bmi = calculate_bmi(height, weight);

    println!(
        "Your BMI is {} and your are {:?}, congrats?",
        bmi.bmi, bmi.conclusion
    );
}

fn calculate_bmi(height: Height, weight: Weight) -> BMI {
    let bmi_number = weight.weight / height.height.powf(2f64);

    BMI {
        bmi: bmi_number,
        conclusion: match bmi_number {
            number if number < 19.0 => BMIConclusion::Underweight,
            number if number >= 19.0 && number < 25.0 => BMIConclusion::Normal,
            number if number >= 25.0 && number < 30.0 => BMIConclusion::Overweight,
            number if number >= 30.0 && number < 35.0 => BMIConclusion::Obese,
            _ => BMIConclusion::MorbidObese,
        },
    }
}

#[cfg(test)]
mod test {
    use super::calculate_bmi;
    use super::BMI;
    use super::Weight;
    use super::Height;
    use super::BMIConclusion;

    #[test]
    fn test_division_by_zero() {
        let bmi = calculate_bmi(Height {
            height: 78.0
        }, Weight {
            weight: 0.0
        });

        println!("{:?}", bmi)
    }
}
