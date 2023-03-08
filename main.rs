use std::io;

fn main() {
    println!("welcome to temperature converter!");
    println!("to convert from °C to °F press 1");
    println!("to convert from °F to °C press 2");
    println!("to convert from °C to K press 3");
    println!("to convert from K to °C press 4");
    println!("to convert from K to °F press 5");
    println!("to convert from °F to K press 6");
    println!("enter choice: ");
    let mut inp = String::new();
    let mut inp_con: f32;
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("error!!");
    let ch2: i8 = choice.trim().parse().unwrap();
    if ch2 == 1 {
        println!("enter temp in °C: ");
        io::stdin().read_line(&mut inp).expect("error!!");
        inp_con = inp.trim().parse().unwrap();
        println!("{}°C is equal to {}°F",inp_con,c_t_f(inp_con));
    }
    if ch2 == 2 {
        println!("enter temperature in °F");
        io::stdin().read_line(&mut inp).expect("error!!");
        inp_con = inp.trim().parse().unwrap();
        println!("{}°F is equal to {}°C",inp_con,f_t_c(inp_con));
    }
    if ch2 == 3 {
        println!("enter temperature in °C");
        io::stdin().read_line(&mut inp).expect("error!!");
        inp_con = inp.trim().parse().unwrap();
        println!("{}°C is equal to {}K",inp_con,c_t_k(inp_con));
    }
    if ch2 == 4 {
        println!("enter temperature in K");
        io::stdin().read_line(&mut inp).expect("error!!");
        inp_con = inp.trim().parse().unwrap();
        println!("{}K is equal to {}°C",inp_con,k_t_c(inp_con));
    }
    if ch2 ==5 {
        println!("enter temperature in K");
        io::stdin().read_line(&mut inp).expect("error!!");
        inp_con = inp.trim().parse().unwrap();
        println!("{}K is equal to {}°F",inp_con,k_t_f(inp_con));
    }
    if ch2 ==2 {
        println!("enter temperature in °F");
        io::stdin().read_line(&mut inp).expect("error!!");
        inp_con = inp.trim().parse().unwrap();
        println!("{}°F is equal to {}K",inp_con,f_t_k(inp_con));
    }
}

fn f_t_c(fahrenheit: f32) -> f32{
    return 5.0*((fahrenheit-32.0)/9.0);
}

fn c_t_f(celcius: f32) -> f32{
    return ((celcius/5f32)*9f32)+32f32;
}

fn c_t_k(celcius: f32) -> f32{
    return celcius+273.15;
}

fn k_t_c(kelvin: f32) -> f32{
    return kelvin-273.15;
}

fn f_t_k(fahrenheit: f32) -> f32{
    let cel = f_t_c(fahrenheit);
    return c_t_k(cel);
}

fn k_t_f(kelvin: f32) -> f32{
    let cel = k_t_c(kelvin);
    return c_t_f(cel);
}
