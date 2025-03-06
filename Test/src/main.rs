use std::io;
use rand::{
    rng, Rng
};
use std::cmp::Ordering;


fn main() {
    let fib: i32 = Fib(6);
    println!("{}", fib);
    
    let cel = FarengeitToCelsiya(64);
    println!("{}", cel);

    // println!("Guess the number!");
    // println!("Please input your guess.");
    
    // let secret_number: u32 = rng().random_range(1..=100);


    let x: (i32, f64, bool) = (32, 34.0, true);

    println!("{}", x.0);
    println!("{}", x.2);

    let a: [i32; 5]= [1, 2, 3, 4, 5];



    for i in (0..a.len()).rev(){
        println!("{}", a[i]);
    }
    



    let first = a[0];
    let second = a[1];

    let months: [&str; 12]= ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    
    let mut res : f64 = another_function(23.6, 34.0);
    let mut res2 : f64 = another_function2(23.6, 34.0);



    // loop {

    //     let mut guess: String = String::new();
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = match guess.trim().parse() 
    //     {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     println!("You secret numver: {}", secret_number);

    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => { println!("You win!"); break; },
    //     }
    // }



}

fn another_function(a:f64, b:f64) -> f64 {
    a + b
}

fn another_function2(a:f64, b:f64) -> f64 {
    return a + b;
}

fn FarengeitToCelsiya(far: i32) -> f64{
    (far as f64 - 32.0) * (5.0 / 9.0)
}

fn Fib(n: i32) -> i32{

    if n < 0{
        panic!("Завязывай глумится!");
    }

    if n == 0{
       return 0;
    } 
    else if n <= 2{
        return 1;
    }
    

    return Fib(n-1) + Fib(n-2);
}

//

/*

jhojhjkhjklh

*/