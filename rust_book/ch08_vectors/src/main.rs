fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1,2,3];

    // Updating a Vector
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    
    // Reading Elements of Vectors
    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[2];
    println!("The third element is {}", third);
    
    // let does_not_exist = &v4[100]
    let does_not_exist = v4.get(100); // Returns None

    // Iterating over the Values in a Vector
    let v5 = vec![100, 32, 57];
    let mut v6 = vec![100, 32, 57];

    for i in &v5 {
        println!("{}", i);
    }

    for i in &mut v6 {
        *i += 50;
    }

    // Using an Enum to Store Multiple Types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

} // <- the vectors go out of scope and are freed here
