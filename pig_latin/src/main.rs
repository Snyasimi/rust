use std::io;


fn main() {
    
    let vowels = vec!["a", "e", "i", "o", "u"];
    
    let mut string = String::new();

    println!("Pig latin :)");
    println!("Enter a word to convert to pig latin");

    io::stdin()
        .read_line(&mut string)
        .expect("Was unable to read line");

    let string = string.trim();
    let first = &string[..1];
    

    if vowels.contains(&first){
        let result = format!("{string}-hay"); 
        println!("{result}");
    } else {

        let substr = &string[1..].trim();    
        let result = format!("{substr}-{first}ay");
        println!("{}",result);
    }



}
