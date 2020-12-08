use std::collections::HashMap;
use std::io;

fn main() {
    let mut empls: HashMap<String, String> = HashMap::new(); //Relation between Employee and Department
    loop {
        println!(
            "Menu:
			1-Add employee to department.
			2-Add departament to company.
			3-Retrive all employees in a department.
			4-Retrive all employees and their department.
			Press any other value to exit."
        );
        let mut op = String::new(); //options in the menu
        let mut dep = String::new(); //Name of department
        let mut emp = String::new(); //Name of employee
        io::stdin()
            .read_line(&mut op)
            .expect("Failed to read line.");
        let op: u32 = match op.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Press a number (1-4) or any other to exit.");
                continue;
            }
        };
        match op {
            1 => add_empl(&mut empls, &mut emp, &mut dep),
            2 => add_dep(&mut empls, &mut dep),
            3 => retrive_by_dep(&empls, &mut dep),
            4 => retrive_all(&empls),
            _ => break,
        };
    }
}
fn add_empl(empls: &mut HashMap<String, String>, emp: &mut String, dep: &mut String) {
    println!("Inform employee name:");
    io::stdin().read_line(emp).expect("Employee");
    println!("Inform employee department:");
    io::stdin().read_line(dep).expect("Department");

    empls.insert(emp.to_string(), dep.to_string());
}
fn add_dep(empls: &mut HashMap<String, String>, dep: &mut String) {
    println!("Inform department name:");
    io::stdin().read_line(dep).expect("Department");
    for (_key, val) in empls.iter() {
        if val == dep {
            println!("Department {}already existent.", dep);
            break;
        }
    }
}
fn retrive_by_dep(empls: &HashMap<String, String>, dep: &mut String) {
    println!("Inform department name:");
    io::stdin().read_line(dep).expect("Department");
    let mut emps: Vec<String> = Vec::new();
    for (key, val) in empls.iter() {
        if val == dep {
            emps.push(key.to_string());
        }
    }
    emps.sort();

    if emps.is_empty() {
        println!("Is Empty or don't exist.");
    } else {
        for e in emps.iter() {
            println!("Employee: {}", e);
        }
    }
}
fn retrive_all(empls: &HashMap<String, String>) {
    let mut emps: Vec<String> = Vec::new();
    for (key, _val) in empls.iter() {
        emps.push(key.to_string());
    }
    emps.sort();
    for e in emps {
        let val: String = match empls.get(&e) {
            Some(val) => val.to_string(),
            None => "".to_string(),
        };
        println!("Employee: {} Department: {} ", e, val);
    }
}
