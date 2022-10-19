// use std::io;

//Answer Fixed 2

fn main(){
    let param1: u32 = "10".parse().expect("Not a number!");
    let param2: u32 = "3".parse().expect("Not a number!");
    println!("param1 = {}",param1);
    println!("param2 = {}",param2);
    let remainder = param1 % param2;
    println!("output = {}", remainder);
    //10 tidak habis di bagi dengan 3, menyisakan angka 1 sebagai hasil bagi
}

// #### Error Program calling let x & let y ####//

// fn main(){
    // println!("Input parameter first : ");
    // let x ;{
    // let mut a = String::new();
    // io::stdin().read_line(&mut a).expect("failed to read from stdin");
    // 
    // let mut trim = a.trim();
    // match trim.parse::<u32>(){
        // Ok(x) => println!("param1 = {x}", x=&a),
        // Err(..) => {println!("this was not an integer: {}", trim);}
        // };
        // x = &a;
        // println!("param1 = {}",x);
    // }
// 
    // println!("Input second parameter : ");
    // let y ; {
    // let mut b = String::new();
    // io::stdin().read_line(&mut b).expect("failed to read from stdin");
    // 
    // let mut trim = b.trim();
    // match trim.parse::<u32>(){
        // Ok(y) => println!("param2 = {y}", y=&b),
        // Err(..) => {println!("this was not an integer: {}", trim);}
        // };
        // y = &b;
        // println!("param2 = {}",y);
    // }
// 
    // let remainder = x % y;
    // println!("output = {}",remainder);
// }


//###################### DRAFT #######################//
// fn main(){
    // let reader = std::io::stdin();
    // let mut line = String::new();

    // let a = reader.read_line(&mut line).unwrap();
    // println!("Line {}", line);
    // println!("Num bytes read = {}", a);
// }
