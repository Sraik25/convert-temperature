use std::io;

fn main() {
    println!("-----------Convert temperature--------");

    println!("--------------------------------------");

    println!("Please input temperature");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    println!("Please type temperature");

    let mut type_temperature = String::new();

    io::stdin()
        .read_line(&mut type_temperature)
        .expect("Failed to read line");

    let temperature: u32 = temperature.trim().parse().expect("Please type a number");

    let mut convert_temperature = 0;

    if type_temperature.trim() == "F" {
        convert_temperature = (temperature * 9 / 5) + 32;
    } else if type_temperature.trim() == "C" {
        convert_temperature = (temperature - 32) * 5 / 9;
    }

    println!("\nYour temperature is: {}", convert_temperature);
}
