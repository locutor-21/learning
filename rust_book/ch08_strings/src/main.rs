fn main() {
    // Creating a new String
    let mut s1 = String::new();

    let data = "initial contents";
    let s2 = data.to_string();
    let s3 = "initial contents".to_string();
    let s4 = String::from("initial contents");

    // Updating a String
    let mut s5 = String::from("foo");
    s5.push_str("bar");
    s5.push("!");

    let s6 = String::from("Hello, ");
    let s7 = String::from("world!");
    let s8 = s6 + &s7; // s6 has been moved here and can no longer be used

    let s09 = String::from("tic");
    let s10 = String::from("tac");
    let s11 = String::from("toe");

    let s30 = format!("{}-{}-{}", s09,s10, s11);

    // Indexing into Strings
    let len = String::from("Hola").len();
    for c in "Hola".chars() {
        println!("{}", c);
    }
    for b in "Hola".bytes() {
        println!("{}", b);
    }
}
