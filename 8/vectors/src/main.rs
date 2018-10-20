fn main() {
    let head = String::from("xd");
    create_vector(&head);
    println!("Head: {}", head);

    let v = vec![1, 2, 3, 4, 5];
    print_element(&v, 2);
    print_element(&v, 100);

    for i in &mut v.clone() {
        *i += 50;
        println!("{}", i);
    }

    for i in &v {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut row: Vec<SpreadsheetCell> = Vec::new();
    row.push(SpreadsheetCell::Int(3));
    row.push(SpreadsheetCell::Text(String::from("blue")));
    row.push(SpreadsheetCell::Float(10.12));

    if let Some(SpreadsheetCell::Int(x)) = row.get(0) {
        println!("Row: {}", x);
    }
}

fn print_element(vector: &Vec<i32>, index: usize) {
    match vector.get(index) {
        Some(x) => {
            println!("Element @ Index: {}", x);
        },
        None => {
            println!("Unreachable element at index: {}", index);
        }
    }
}

fn create_vector(head: &String) {
    let vector = vec![head.clone(), String::from("dd")];
    println!("Vec: {}", vector.first().unwrap());
}
