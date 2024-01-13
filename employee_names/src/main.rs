use std::io;
use std::collections::HashMap;

fn main() {
    
    let mut cont = 0;
    let mut strings: Vec<String> = Vec::new();
    let mut  employees = HashMap::new();

    println!("Employee Names");
    println!("Enter epmloyee to add to a dept,\n Use \"Add \'Employee_name\' to \'Department\' \"\n");
    println!("{:?}",employees);

    loop {


    let mut input = String::new();

    cont +=1;

    if cont > 5 {break;}
        println!("Enter employee and dept");

    
        io::stdin()
            .read_line(&mut input)
            .expect("Could not real input");
    
        let input = input.trim();

        strings.clear();

        for word in input.split_whitespace(){

            strings.push(word.to_string());
        }

        
        
            employees.insert(strings[1].clone(),strings[3].clone());
        
    }

    println!("{:?}",employees);
}
