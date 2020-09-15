/*
This is a HashMap exercise.

In the real world I'd create a database with the following tables (and columns):
department (id, name)
- index by: name
employee (id, name, department_id)
- index by: department_id, name
*/

use std::io;
use std::collections::HashMap;

fn print_usage() {
    println!("USAGE:");
    println!("add NAME to DEPARTMENT");
    println!("list DEPARTMENT");
    println!("list_all");
    println!("quit");
}

fn main() {
    print_usage();
    
    // directory data structure
    let mut dir = HashMap::new();
    
    loop {
        println!();
        
        // get user input
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        // trim newline
        let input = input.trim().to_string();
        
        // parse into words
        let words: Vec<&str> = input.as_str().split_whitespace().collect();
        
        if words.len() < 1 {
            print_usage();
        } else {
            // handle commands
            match words[0] {
                "add" => {
                    dir = do_add(dir, &words);
                },
                "list" => {
                    do_list(&dir, &words);
                }
                "list_all" => {
                    do_list_all(&dir, &words);
                }
                "quit" => {
                    println!("bye");
                    break;
                },
                _ => print_usage(),
            }
        }
    }
}

fn do_add (mut dir: HashMap<String, HashMap<String, ()>>, words: &Vec<&str>) -> HashMap<String, HashMap<String, ()>> {
    if words.len() != 4 || words[2] != "to" {
        print_usage();
        return dir;
    }
    
    let employee = words[1].to_string();
    let department = words[3].to_string();
    
    let dep_employees = dir.entry(department).or_insert(HashMap::new());
    *dep_employees.entry(employee).or_insert(());
    
    println!("done");
    dir
}

fn do_list(dir: &HashMap<String, HashMap<String, ()>>, words: &Vec<&str>) {
    if words.len() != 2 {
        print_usage();
        return;
    }
    
    let department = words[1].to_string();
    list_department(&dir, &department);
}

fn list_department(dir: &HashMap<String, HashMap<String, ()>>, department: &String) {
    match dir.get(department) {
        Some(employees) => {
            // sort
            let mut employees: Vec<&String> = employees.keys().collect();
            employees.sort();
            
            // print
            for e in employees.iter() {
                println!("{}", e);
            }
        },
        None => {
            // 404
            println!{"department \"{}\" not found", &department};
        },
    };
}

fn do_list_all(dir: &HashMap<String, HashMap<String, ()>>, words: &Vec<&str>) {
    if words.len() != 1 {
        print_usage();
        return;
    }
    
    // sort
    let mut departments: Vec<&String> = dir.keys().collect();
    departments.sort();
    
    // print
    for d in departments.iter() {
        println!("department {}:", d);
        list_department(&dir, d);
    }
}
