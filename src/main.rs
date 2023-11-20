use std::collections::HashMap;
use std::io;
use std::process;

// base unit: meters
const LENGTH_CONVERSIONS: [([&str; 2], f64); 7] = [
    (["inch", "in"], 0.0254),
    (["foot", "ft"], 0.3048),
    (["yard", "yd"], 0.9144),
    (["mile", "mi"], 1609.344),
    (["fathom", "ftm"], 1.8288),
    (["cable length", "cbl"], 185.2),
    (["nautical mile", "nmi"], 1853.18),
];
// base unit: grams
const MASS_CONVERSIONS: [([&str; 2], f64); 8] = [
    (["pound", "lb"], 453.59237),
    (["ounce", "oz"], 28.34952),
    (["grain", "gt"], 0.06479891),
    (["dram", "dr"], 1.77184),
    (["stone", "st"], 6350.29),
    (["quarter", "qr"], 12700.58),
    (["hundredweight", "cwt"], 50802.35),
    (["ton", "t"], 1016050.0),
];
// base unit: bars
const PRESSURE_CONVERSIONS: [([&str; 2], f64); 4] = [
    (["pounds per square inch", "psi"], 0.0689476),
    (["feet of water", "ftH2O"], 0.0304808),
    (["atmospheres", "atm"], 1.01325),
    (["pounds per square foot", "psf"], 0.0208854),
];
// base unit: meters per second
const VELOCITY_CONVERSIONS: [([&str; 2], f64); 3] = [
    (["feet per second", "ft/s"], 0.3048),
    (["miles per hour", "mph"], 0.44704),
    (["knots", "kn"], 0.51444),
];
// base unit: meters per second squared
const ACCELERATION_CONVERSIONS: [([&str; 2], f64); 3] = [
    (["feet per second squared", "ft/s²"], 0.3048),
    (["miles per hour per second", "mph/s"], 0.44704),
    (["inches per second squared", "in/s²"], 0.0254),
];
// base unit: L/100km
const MILEAGE_CONVERSIONS: [([&str; 2], f64); 1] = [
    (["miles per gallon", "mpg"], 235.21458329475),
];
// base unit: liter
const VOLUME_CONVERSIONS: [([&str; 2], f64); 12] = [
    (["cubic inch", "in³"], 0.0163871),
    (["cubic foot", "ft³"], 28.3168),
    (["us fluid ounce", "us fl oz"], 0.0295735),
    (["us gallon", "us gal"], 3.78541),
    (["uk fluid ounce", "uk fl oz"], 0.0284131),
    (["uk gallon", "uk gal"], 4.54609),
    (["us pint", "us pt"], 0.473176),
    (["us quart", "us qt"], 0.946353),
    (["uk pint", "uk pt"], 0.568261),
    (["uk quart", "uk qt"], 1.13652),
    (["us cup", "us cup"], 0.24),
    (["us fluid drams", "us fl dr"], 0.00369669),
];
// base unit: square meter
const AREA_CONVERSIONS: [([&str; 2], f64); 5] = [
    (["square inch", "sq in"], 0.00064516),
    (["square foot", "sq ft"], 0.00064516),
    (["square yard", "sq yd"], 0.00064516),
    (["acre", "ac²"], 0.00064516),
    (["square mile", "sq mi"], 0.00064516),
];
// standard metric prefixes
const PREFIXES: [&str; 13] = [
    "T",
    "G",
    "M",
    "k",
    "h",
    "da",
    "",
    "d",
    "c",
    "m",
    "µ",
    "n",
    "p",
];
// doesn't make much sense for the other ones to me. At least I've never needed mm/s²
const PREFIX_UNIT_LIST: [&str; 4] = [
    "len",
    "mas",
    "pre",
    "vol",
];

struct UnitData {
    unit_type: String,
    conversion_factor: f64,
}


fn convert(input: f64, from: &str, unit_info: &HashMap<String, UnitData>) -> (f64, String) {
    let mut result: f64 = 0.0;
    let mut unit_type = String::new();
    let mut unit_prefix = String::new();

    if let Some(unit_data) = unit_info.get(from) {
        if unit_data.unit_type != "mil" {
            result = input * unit_data.conversion_factor;
        } else {
            result = unit_data.conversion_factor/input;
        }
        if unit_data.unit_type == "len" {
            unit_type = String::from("m");
        } else if unit_data.unit_type == "mas" {
            unit_type = String::from("g");
        } else if unit_data.unit_type == "pre" {
            unit_type = String::from("bar");
        } else if unit_data.unit_type == "vel" {
            if result < 1.0 {
                unit_type = String::from("m/s");
            } else {
                result = result*3.6;
                unit_type = String::from("km/h");
            }
        } else if unit_data.unit_type == "acc" {
            unit_type = String::from("m/s²");
        } else if unit_data.unit_type == "mil" {
            unit_type = String::from("L/100km");
        } else if unit_data.unit_type == "vol" {
            unit_type = String::from("L");
        } else if unit_data.unit_type == "are" {
            unit_type = String::from("m²");
        }
        if PREFIX_UNIT_LIST.contains(&unit_data.unit_type.as_str()) {
            let mut count: usize = 6;
            let mut min_max: (usize, usize) = (3, 10);
            if unit_data.unit_type == "vol" {
                 min_max = (6, 10);
            }
            while result > 10.0 {
                if count > min_max.0{
                    result = result/10.0;
                    count -= 1;
                    unit_prefix = PREFIXES[count].to_string();
                } else{
                    break;
                }
            }
            while result < 1.0 {
                if count < min_max.1{
                    result = result*10.0;
                    count += 1;
                    unit_prefix = PREFIXES[count].to_string();
                } else {
                    break;
                }
            }
        }
    }
    unit_prefix.push_str(&unit_type);
    (result, unit_prefix)
}


fn map() -> HashMap<String, UnitData> {
    let mut unit_info: HashMap<String, UnitData> = HashMap::new();

    for unit in LENGTH_CONVERSIONS {
        for name in unit.0 {
            unit_info.insert(name.to_string(), UnitData{unit_type: String::from("len"), conversion_factor: unit.1});
        }
    }
    for unit in MASS_CONVERSIONS {
        for name in unit.0 {
            unit_info.insert(name.to_string(), UnitData{unit_type: String::from("mas"), conversion_factor: unit.1});
        }
    }
    for unit in PRESSURE_CONVERSIONS {
        for name in unit.0 {
            unit_info.insert(name.to_string(), UnitData{unit_type: String::from("pre"), conversion_factor: unit.1});
        }
    }
    for unit in VELOCITY_CONVERSIONS {
        for name in unit.0 {
            unit_info.insert(name.to_string(), UnitData{unit_type: String::from("vel"), conversion_factor: unit.1});
        }
    }
    for unit in ACCELERATION_CONVERSIONS {
        for name in unit.0 {
            unit_info.insert(name.to_string(), UnitData{unit_type: String::from("acc"), conversion_factor: unit.1});
        }
    }
    for unit in MILEAGE_CONVERSIONS {
        for name in unit.0 {
            unit_info.insert(name.to_string(), UnitData{unit_type: String::from("mil"), conversion_factor: unit.1});
        }
    }
    for unit in VOLUME_CONVERSIONS {
        for name in unit.0 {
            unit_info.insert(name.to_string(), UnitData{unit_type: String::from("vol"), conversion_factor: unit.1});
        }
    }
    for unit in AREA_CONVERSIONS {
        for name in unit.0 {
            unit_info.insert(name.to_string(), UnitData{unit_type: String::from("are"), conversion_factor: unit.1});
        }
    }
    unit_info
}


fn parse_input(input: &str, unit_info: &HashMap<String, UnitData>) -> Option<(f64, String)> {
    // Find the index at which the first alphabetical character or whitespace occurs
    let split_index = input.find(|c: char| char::is_alphabetic(c) || char::is_whitespace(c)).unwrap_or(input.len());

    // Split the input into numeric and unit parts
    let (numeric, unit) = input.split_at(split_index);

    // Trim leading and trailing whitespace from the unit part
    let mut unit = unit.trim();
    if unit == "feet" {
        unit = "foot";
    } else if unit == "cubic feet" {
        unit = "cubic foot";
    }

    // Create a string to hold the resulting unit name
    let mut unit_name = String::from(unit);

    // Try to find a matching unit name in the unit_info HashMap
    while !unit_info.contains_key(&unit_name) && unit_name.len() > 0 {
        unit_name.pop();
    }

    // Attempt to parse the numeric part into an f64
    if let Ok(numeric_value) = numeric.parse::<f64>() {
        Some((numeric_value, unit_name))
    } else {
        None
    }
}


fn main() {
    let unit_info: HashMap<String, UnitData> = map();
    loop {
        println!("You have:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Whoops. Something went wrong.");
        if input == "quit\n" {
            process::exit(0);
        }
        if let Some((value, unit)) = parse_input(&input, &unit_info) {
            let converted = convert(value, &unit, &unit_info);
            println!("Which roughly equals:");
            println!("{:.3}{}", converted.0, converted.1);
        } else {
            println!("Invalid input format");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conversion() {
        let unit_info: HashMap<String, UnitData> = map();
        assert_eq!(convert(10.0, "in", &unit_info), (0.254, String::from("m")));
        assert_eq!(convert(10.0, "foot", &unit_info), (3.048, String::from("m")));
        assert_eq!(convert(10.0, "lb", &unit_info), (4535.9237, String::from("g")));
    }
}