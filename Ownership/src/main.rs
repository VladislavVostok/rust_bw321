fn main() {

    {                         
        let s = "hello";   // s is valid from this point forward
        println!("{}", s);   
        // do stuff with s
    } 

    let x = 5;
    let y = x;
    println!("{}",  x);

    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}",  s1);
}
