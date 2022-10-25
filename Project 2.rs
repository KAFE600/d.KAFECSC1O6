use std::process::Command;

fn main()
{
  let toshiba = (2.0,450000.00);
  let mac = (1.0,1500000.00);
  let hp = (3.0,750000.00);
  let dell = (3.0,2850000.00);
  let acer = (1.0,250000.00);
  let sum:f64 = acer.1 + dell.1 + mac.1 + toshiba.1;
  let quant:f64 = acer.0 + dell.0 + hp.0 +mac.0 +toshiba.0;
  println("{}",quant);
  println("Sum is: {}",sum);
  println("Average is:{}",sum/quant);

  Let_=Command::new("cmd.exe").arg("/c").arg("pause").status();
  }