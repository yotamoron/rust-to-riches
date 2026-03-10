/*
 * Temperature Converter Create a command-line utility that prompts the 
 * user to enter a temperature and its unit (Celsius or Fahrenheit). 
 * Read the input, parse the string into a floating-point number, perform the 
 * correct mathematical conversion, and print the result. 
 * This will get you comfortable with basic I/O and shadowing.
*/

use std::io;

#[derive(Debug)]
enum TemperatureUnit {
    Fahrenheit,
    Celsius
}

struct Temperature {
    temp: f64,
    unit: TemperatureUnit
}

fn read(msg: &str, expect: &str) -> String {
    let mut val = String::new();

    println!("{msg}");
    io::stdin()
        .read_line(&mut val)
        .expect(expect);

    val
}

fn temp() -> f64 {
    let temp = read("What is the temperature?", "Failed to read temperature");
    let temp: f64 = match temp.trim().parse() {
        Err(e) => panic!("Failed parsing {temp}: {e}"),
        Ok(val) => val
    };

    temp
}

fn unit() -> TemperatureUnit {
    let unit = read("What is the unit (f|c):", "Failed to read unit");
    
    match unit.trim() {
        "f" => TemperatureUnit::Fahrenheit,
        "c" => TemperatureUnit::Celsius,
        _ => panic!("Unknown temprature {unit}, allowed values are f or c")
    }
}

fn convert(from_user: Temperature) -> Temperature {
    match from_user.unit {
        TemperatureUnit::Celsius => Temperature { 
            temp: from_user.temp * (9.0/5.0) + 32.0, 
            unit: TemperatureUnit::Fahrenheit 
        },
        TemperatureUnit::Fahrenheit => Temperature { 
            temp: (from_user.temp - 32.0) * (5.0/9.0), 
            unit: TemperatureUnit::Celsius
        }
    }

}

fn main() {
    let from_user = Temperature {
        temp: temp(),
        unit: unit()
    };
    let converted = convert(from_user);

    println!("Converted. Temp: {}, Unit: {:?}", converted.temp, converted.unit);
}
