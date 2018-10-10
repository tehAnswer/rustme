fn main() {
    let mut counter = 0;
    let counter = loop {
        println!("{}", counter);
        if counter == 10 {
            break counter * 2;
        }
        counter = counter + 1;
    };

    assert_eq!(counter, 20);

    let mut counter = 4;

    while counter != 0 {
        println!("woof.");
        counter -= 1;
    }

    let elements = [10, 20, 30, 40];

    for i in elements.iter() {
        println!("{}", i);
    }

    for i in (1..4).rev() {
        println!("{}!!!", i);
    }
    println!("Engage.");
}
