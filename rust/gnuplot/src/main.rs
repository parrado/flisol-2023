use gnuplot::{Figure, Caption, Color};
use std::{thread, time};
use rand::Rng;
use std::io::prelude::*;
use std::process::{Command, Stdio,ChildStdin};

fn start_gnuplot()->Result<ChildStdin,std::io::Error>{    
    let process = match Command::new("gnuplot")
                                .args(&["-persist"])
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => {println!("kkk{why}");return Err(why)},
        Ok(process) => return Ok(process.stdin.unwrap()),
    };

    
}

fn plot(t:&Vec<f64>,x:&Vec<f64>,y:&Vec<f64>,stdin:&mut ChildStdin){
    // Write a string to the `stdin` of `wc`.
    //
    // `stdin` has type `Option<ChildStdin>`, but since we know this instance
    // must have one, we can directly `unwrap` it.
    stdin.write_all("plot '-' with lines title 'y[n]','-' with lines title 'u[n]'\n".as_bytes());

    for i in 0..x.len(){    
    stdin.write_all(format!("{} {}\n",t[i],x[i]).as_bytes());
    }
    
    stdin.write_all("e\n".as_bytes());

    for i in 0..x.len(){    
        stdin.write_all(format!("{} {}\n",t[i],y[i]).as_bytes());
        }
        
        stdin.write_all("e\n".as_bytes());
    
    
}


fn main(){

const N:usize=100;


let mut t=vec![0.0;N];
let mut u=vec![0.0;N];
let mut y=vec![0.0;N];

for i in 0..u.len(){
    t[i]=i as f64;
}

let mut gnuplot=start_gnuplot().expect("");
let mut rng = rand::thread_rng();


loop{
    let value:f64= rng.gen();
    u.remove(0);
    u.push(value);


    let value:f64= rng.gen();
    y.remove(0);
    y.push(value);

    plot(&t,&u,&y,&mut gnuplot);
  
    thread::sleep(time::Duration::from_millis(100));
}



}