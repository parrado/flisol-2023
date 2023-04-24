use rand::Rng;

fn dotProduct(x: &[f64], y: &[f64])-> Result<f64,String> {

    let mut dot: f64;

    if x.len() != y.len(){
        return Err("Arrays have different size".to_string());
    }

    dot = 0.0;

    for i in 0..x.len(){ // [0,x.len())
        dot += x[i] * y[i];
    }

    return Ok(dot)
}

fn main() {
    const N:usize = 10000000;
    let mut x=vec![0.0;N];
    let mut y=vec![0.0;N];

    let mut rng = rand::thread_rng();


    for i in 0..x.len() {
        x[i]= rng.gen();
    }

    for i in 0..y.len() {     
        y[i]= rng.gen();
    }

    match dotProduct(&x, &y){
        Ok(dot)=>println!("Dot product of x and y is: {}",dot),
        Err(msg)=>println!("Dot product failed {}:",msg)

    }
    
}
