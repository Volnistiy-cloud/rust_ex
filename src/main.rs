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

    for count in 2..number {
        let mut _done = true;
            
            if _done {
                let stop = ((count as f64).sqrt() + 1.0) as i64;
                    for i in 2..stop {
                            if count % i == 0 {
                                _done = false;
                                break;
                            }
                    }
                }
                if _done {
                   //print!("{} ", count);
                   vector.push(count);
                }
        }
<<<<<<< HEAD
   
        for i in 1..vector.len(){
           
        //if vector[i] - vector[i - 1] == 2{
        //    print!("{}, {}, ", vector[i - 1].to_string().green(), vector[i].to_string().green());
          //continue;
          // }
=======

        for i in 1..vector.len(){
           
        if vector[i] - vector[i - 1] == 2{
            print!("{}, {}, ", vector[i - 1].to_string().green(), vector[i].to_string().green());
           }
           else {
>>>>>>> bb31abb06cf2fd568b5d4857835bf8b4d9fee071
           print!("{}, ", vector[i - 1]);
       
       
        }
        //print!("{}, ", vector[i - 1]);
break;} 
}     
