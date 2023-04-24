
use rand::Rng;

fn main() {
    test_if();

    println!("{}",decoder(3));

    test_match("Success");

    test_sort();
}



fn test_if(){
    let mut rng = rand::thread_rng();
    let a:f64= rng.gen();
    let b:f64= rng.gen();

    let mut max1 = a;
    if a < b {
        max1 = b;
    }

    let max2: f64;

    if a > b {
        max2 = a
    } else  {
        max2 = b
    }


    let max3 = if a > b {a} else {b};
    println!("{max1}, {max2}, {max3}")
}

fn decoder(x: u8)-> u8{
    let r: u8;
    r = match x{
        0       => 0b0001,
        1       => 0b0010,
        2       => 0b0100,
        3       => 0b1000,
        _    => 0
    };
    return  r;
}

fn test_match(m: &str){
   match m {
    "Error" => {
        println!("You screwed it!!");
    }
    "Success" => {
        println!("You nailed it!!");
    }
    &_ => println!("Invalid message")
   
       
   }


}


fn test_sort(){
    let mut y = [0u128;10];
    let mut rng = rand::thread_rng();    
   

    for i in 0..y.len(){ // Test out of bound index =y.len()
        y[i]= rng.gen();
    }

    println!("Unsorted array");

    for item in y{
        println!("{item},");
    }

    y.sort();

    println!("\nSorted array");
    for item in y{
        println!("{item},");
    }
}







