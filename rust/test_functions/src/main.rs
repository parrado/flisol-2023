fn my_awesome_function(x:f64,y:f64)->(f64,f64){
    

    let phasor=(f64::sqrt(x*x+y*y),y.atan2(x));

    return phasor;
}



fn main() {
    let phasor=my_awesome_function(-1.0,-1.0);
    println!("Magnitude: {},Phase: {}",phasor.0,phasor.1);
}
