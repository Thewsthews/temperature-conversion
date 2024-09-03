use std::io;
use std::io::stdin;

fn main() {
    loop {
        println!("Please select the conversion type: ");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Farenheit");

        let mut conversion_type = String::new();
        stdin.read_line(&mut conversion_type)?;
                match io::stdin().read_line(&mut conversion_type){
            Ok(_) => {
                let conversion_type = conversion_type.trim();
                let conversion_type = match conversion_type{
                    "1" => 1,
                    "2" => 2,
                    _ => {
                        println!("Please input 1 or 2 :) ");
                        continue;
                    }
                };

                println!("Please input the temperature: ");
                let mut temperature = String::new();
                stdin().read_line(&mut temperature);

                let temperature: f64 = match temperature.trim().parse(){
                    Ok(num) => num,
                };

                let converted_temperature = if conversion_type == 1{
                    (temperature - 32.) / 1.8
                } else{
                    temperature * 1.8 + 32.
                };
                println!("The conversion temperature is {}", converted_temperature);
            },
            Err(error) => {
                println!("Error reading input: {}", error);
                println!("Please input a valid temperature");
                    continue;
                
            }
        }
    }
}
