use std::collections::HashMap;

fn main() {
    

    let mut integers = vec![1,5,3,23,2,1,4];
    //let mut map = HashMap::new();
    //USING BUBBLESORT
    for _i in 0..integers.len(){

        //push elements into array
        //
        


        for j in 0..integers.len() - 1{

            if integers[j] > integers[j+1]{
    
                let temp = integers[j];
                integers[j] = integers[j+1];
                integers[j+1] = temp;
            }

        }

        
    }


let  mut new = HashMap::new();


    for i in 0..integers.len(){

        let cont = new.entry(integers[i]).or_insert(0);

        *cont+=1;
    }

println!("Printing vector");

for i in 0..integers.len(){println!("{}",integers[i]);}
    println!("The Median is {}",integers[integers.len() / 2]);

    println!("How many times each number occurs in the vector");
    println!("{:?}",new);

}
