fn main() {
    let number = 3;
    if number < 3  || number == 3 {
        println!("it was true.");
    } else {
        println!("it was false.");
    };

    let number = 6;
    if number % 4 == 0 {
        println!("divisible by four");
    } else if number % 3 == 0 {
        println!("divisible by three");
    } else if number % 2 == 0 {
        println!("this will be never reach with number = 6");
    };
}
