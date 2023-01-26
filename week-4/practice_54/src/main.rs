use std::io;

fn main() {
    let mut input1 = String::new();
    
    println!("Enter your number");
    io::stdin().read_line(&mut input1).expect("Not a vaild string");
    let mut num:i32 = input1.trim().parse().expect("Not a vaild number");

    while num > 15 {
        println!("inside the loop number is {}",num);
        num+=1;
    }
    println!("outside the loop number is {}",num);

}   

    
 
