fn main() {
  let a = 99; //set variable
  if a >= 99 { //set and then check condition
    println!("Big Number!");
  } else {
    println!("Small number.");
  }
  ifelse1();
  ifelse2();
}

//Nest if...else

fn ifelse1() {
  let a = 201;
  if a > 99 {
    if a > 200 {
      println!("Huge number!");
    } else {
      println!("Big Number!");
    }
  }
  else {
    println!("Small number");
  }
}

fn ifelse2() {
  let a = 99;
  if a > 200 {
    println!("Huge Number!");
  } else if a > 99 {
    println!("Big Number!");
  } else {
    println!("Small number.");
  }
}