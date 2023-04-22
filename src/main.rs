use std::fmt::Debug;
use std::io::Write;
use std::str::FromStr;

struct Weight(f64);

struct Height(f64);

#[derive(Debug)]
struct Bmi {
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

    let weight = Weight(f64::from_str(buffer_weight.trim()).unwrap_or_else(|err| {
        println!("Error while parsing: {}", err);
        println!("Using standard values for a male");
        85.2
    }));
    println!("Weight is {}", weight.0);

    print!("Please input your height in meters: ");

    let _ = std::io::stdout().flush();

    let mut buffer_height = String::new();
    stdin
        .read_line(&mut buffer_height)
        .unwrap_or_else(|err| panic!("Error while parsing: {}", err));

    let height = Height(f64::from_str(buffer_height.trim()).unwrap_or_else(|err| {
        println!("Error while parsing: {}", err);
        println!("Using standard values for a male");
        1.8
    }));
    println!("Weight is {}", height.0);

    let bmi = calculate_bmi(height, weight);

    println!(
        "Your BMI is {} and your are {:?}, congrats?",
        bmi.bmi, bmi.conclusion
    );
}

fn calculate_bmi(height: Height, weight: Weight) -> Bmi {
    let bmi_number = weight.0 / height.0.powf(2f64);

    Bmi {
        bmi: bmi_number,
        conclusion: match bmi_number {
            number if number < 19.0 => BMIConclusion::Underweight,
            number if (19.0..25.0).contains(&number) => BMIConclusion::Normal,
            number if (25.0..30.0).contains(&number) => BMIConclusion::Overweight,
            number if (30.0..35.0).contains(&number) => BMIConclusion::Obese,
            _ => BMIConclusion::MorbidObese,
        },
    }
}

#[cfg(test)]
mod test {
    use super::calculate_bmi;
    use super::BMIConclusion;
    use super::Bmi;
    use super::Height;
    use super::Weight;

    #[test]
    fn test_division_by_zero() {
        let bmi = calculate_bmi(Height(78.0), Weight(0.0));

        println!("{:?}", bmi)
    }
}
