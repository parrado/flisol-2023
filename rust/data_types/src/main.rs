
fn main() {
    
    // Tuples example
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    let five_hundred=tup.0;

    println!("The value of y is: {y}");
    println!("The value of five_hundred is: {five_hundred}");

    // Array examples
    let a = [1, 2, 3, 4, 5];
    let b:[f64;5] = [1.0, 2.0, 3.0, 4.0, 5.0];

    let mut x = [3u32; 5];

    x[0]=0x80000000;

    println!("{:?}",x);

    let mut y=[0.0;5];
    let z= ["Hola","Mundo","Cruel"];

    y[0]=25.0;

    // Test out of bound index
    let w = f64::cos(y[4]).to_string()+z[2];

    // What is shown?
    println!("Result is: {w}");


}
