use std::io;

fn main() {
    println!("Do you want to convert to (c)elsius or (f)ahrenheit");

    let mut temperature_to = String::new();
    loop{
        io::stdin()
            .read_line(&mut temperature_to)
            .expect("Failed to read line");
        
        if temperature_to.starts_with('c') || temperature_to.starts_with('f') {
            break;
        }
        temperature_to.clear();
    }

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperatrue: f64 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let temperature_converted: f64;

    if temperature_to.starts_with('c'){
        temperature_converted = (temperatrue - 32.0) * (5.0/9.0);
    }else{
        temperature_converted = (temperatrue * (9.0/5.0)) + 32.0;
    }

    println!("Temperatur {} was converted to {}", temperatrue, temperature_converted);
    
}
