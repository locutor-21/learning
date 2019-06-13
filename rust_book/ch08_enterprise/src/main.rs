#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;
use std::collections::HashMap;

pub fn add_employee(company: &mut HashMap<String, Vec<String>>) {
    let mut department = String::new();
    let mut employee = String::new();

    println!("Which department?");
    
    if !company.is_empty() {
        print!("Options are:");
        for key in company.keys(){
            print!(" {}", key);
        }
        println!("");
    }

    io::stdin().read_line(&mut department)
        .expect("Failed to read command.");
    let department = department.trim().to_string();

    println!(
        "What is the name of the employee?"
    );
    io::stdin().read_line(&mut employee)
        .expect("Failed to read command.");
    let employee = employee.trim().to_string();

    let department_name = department.clone();

    let department = company.entry(department).or_default();
    department.push(employee);

    println!("Done! {} now has the following structure:\n{:?}", department_name, department);
    
    println!("");
}

pub fn list_department(company: &mut HashMap<String, Vec<String>>) {

    let mut department = String::new();

    println!("Which department?");
    
    if !company.is_empty() {
        print!("Options are:");
        for key in company.keys(){
            print!(" {}", key);
        }
        println!("");
    }

    io::stdin().read_line(&mut department)
        .expect("Failed to read command.");
    let department = department.trim().to_string();

    if company.contains_key(&department) {
        let mut names = company.get(&department).unwrap().clone();
        names.sort();
        for name in names {
            print!("{} ", name);
        }
        println!("");
    }
    println!("");
}

pub fn list_company(company: &mut HashMap<String, Vec<String>>) {

    if company.is_empty(){
        println!("Company has no departments.");
    } else{
        for (department, employees) in company {
            let mut names = employees.clone();
            names.sort();
            
            print!("{}:", department.to_uppercase());
            for name in names {
                print!(" {}", name);
            }
            println!("");
        }
    }
    println!("");
}

fn main() {
    
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        
        println!(
            "Choose a number:
    1) Add employee to a Department
    2) List people in a Department
    3) List all people in the Company, by Department
    4) Exit"
        );

        let mut action = String::new();
        io::stdin().read_line(&mut action)
            .expect("Failed to read command.");

        let action: u8 = match action.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match action {
            1 => add_employee(&mut company),
            2 => list_department(&mut company), 
            3 => list_company(&mut company),
            4 => break,
            _ => continue,
        }
    }
}
