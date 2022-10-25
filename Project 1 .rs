use std::process::Command

fn main() 
{
  let loan:f64 = 520000000.0;
  let rate:f64 = 10.0/100.10;
  let time:f64 = 5.0;
  let mut amount:f64 = 1.0+rate;

amount=1.0 + f64::powf(amount,time);
//println!("{}",f64::powf(2.0,4.0);

amount=amount* loan;

let compound_interest:f64 = amount - loan;


println!("Amount: {}",compound_interest);
let_=Command::new("cmd.exe").arg("/c").arg("pause").status();
}