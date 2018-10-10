fn main() {
    let number = 2;
    let fib = fibonnaci(number);
    println!("The fibonnaci of {} is {}", number, fib)
}

fn fibonnaci(x: u32) -> u32 {
    let mut sum = 0;
    for i in 1..x+1{
        sum += i
    }
    sum
}
