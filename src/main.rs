use colored::Colorize;
use std::io;

fn main() {
   let mut vector= Vec::new();
   loop {

    println!("Please input number to which we count the prime numbers");

    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("Failed to read line");

    let number: i64 = match number.trim().parse() {
       Ok(num) => num,
       Err(_) => continue,
        
    };

    for count in 1..number {
        let mut _done = true;
        if count % 2 != 0 {
            
            if _done {
                let stop = ((count as f64).sqrt() + 1.0) as i64;
                    for i in 3..stop {
                        if i % 2 != 0 {
                            if count % i == 0 {
                                _done = false;
                                break;
                            }
                
                        }
                    }
                }
                if _done {
                   //print!("{} ", count);
                   vector.push(count);
                }
            }
        }
        let mut i = 1;
        for  mut i in i..vector.len(){
           
        if vector[i] - vector[i - 1] == 2{
            print!("{}, {}, ", vector[i - 1].to_string().green(), vector[i].to_string().green());
           i = i + 1;
           }
           else {
           print!("{}, ", vector[i - 1]);
           }
        }
break;} 
}     