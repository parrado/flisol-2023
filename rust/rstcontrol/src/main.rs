// RST realtime control of emulated double integrator
// Alexander López-Parrado (2023)

use serial2::SerialPort;
use std::{thread, time};
use std::io::prelude::*;
use std::process::{Command, Stdio,ChildStdin};
use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Write, stdout, stdin};

// Sampling period in microseconds
const TS:u64=434164;

// R,S,T polynomials and K gain
const R:[f64;2]=[1.000000000000000  , 0.324493467752342];
const S:[f64;2]=[4.669255625406208 , -3.490658412306608];
const T:[f64;2]=[1.000000000000000 , -0.010000000000000];
const K:f64=1.190502235454142;



fn start_gnuplot()->Result<ChildStdin,std::io::Error>{    
    let process = match Command::new("gnuplot")
                                .args(&["-persist"])
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => {println!("{why}");return Err(why)},
        Ok(process) => return Ok(process.stdin.unwrap()),
    };

    
}

fn plot(t:&Vec<f64>,x:&Vec<f64>,y:&Vec<f64>,r:&Vec<f64>,stdin:&mut ChildStdin){
    // Write a string to the `stdin` of `wc`.
    //
    // `stdin` has type `Option<ChildStdin>`, but since we know this instance
    // must have one, we can directly `unwrap` it.
    stdin.write_all("plot '-' with lines title 'u[n]','-' with lines title 'y[n]','-' with lines title 'r[n]'\n".as_bytes());

    for i in 0..x.len(){    
    stdin.write_all(format!("{} {}\n",t[i],x[i]).as_bytes());
    }
    
    stdin.write_all("e\n".as_bytes());

    for i in 0..x.len(){    
        stdin.write_all(format!("{} {}\n",t[i],y[i]).as_bytes());
        }
        
        stdin.write_all("e\n".as_bytes());
        for i in 0..x.len(){    
          stdin.write_all(format!("{} {}\n",t[i],r[i]).as_bytes());
          }
          
          stdin.write_all("e\n".as_bytes());
      
      
    
}

// Function implementing RST controller
fn rst(refe:f64,out:f64,k:f64,r:&[f64],s:&[f64],t:&[f64],delayt:&mut[f64],delays:&mut[f64],delaye:&mut[f64],n:usize)-> f64{

    // Auxiliary variables
    let mut out_t=0.0;
    let mut out_s=0.0;
    let mut out_r=0.0;
    let mut error;

    delayt[0]=refe;
    delays[0]=out;

    // Reference and output filtering
    for i in (0..n).rev()
    {
      out_t+= t[i]*delayt[i];
      let index =if i==0 {i}else{i-1};
      delayt[i]=delayt[index];

      out_s+= s[i]*delays[i];
      delays[i]=delays[index];
    }

    // Error signal
    error=k*out_t-out_s;

    out_r=error;
    // Error filtering
    for i in (1..n).rev(){      
      out_r-=r[i]*delaye[i];
      delaye[i]=delaye[i-1];
    }

    delaye[1]=out_r/r[0];


    
    return out_r/r[0];
  }

// Function to read plant output
fn read_output(port:&SerialPort)->Result<f64,std::io::Error>{
                let mut buffer1 = [0u8; 1];
                buffer1[0]=1;
                port.write(&buffer1)?;
                let mut buffer2 = [0u8; 8];
                let mut nBytes=0;
                
                loop{
                let read=port.read(&mut buffer2[nBytes..])?;               
                nBytes+=read;    
                if nBytes==8 {
                    break;
                }
                }              

                let value=f64::from_ne_bytes(buffer2);
                

                return Ok(value);
            }

// Function to write control signal
fn write_control(port:&SerialPort,value:f64)->Result<(),std::io::Error>{
                let mut buffer1 = [0u8; 1];
                buffer1[0]=0;
                
                port.write(&buffer1)?;              

                let mut nBytes=0;
                let data=value.to_ne_bytes();
                loop {
                let written=port.write(&data[nBytes..])?;
                nBytes+=written;
                if nBytes==8 {
                    break;
                }
                }    
                return Ok(());
            }
        
        

// Function to serially connect to ESP32
fn connect()->Result<SerialPort,std::io::Error>{

    match serial2::SerialPort::available_ports() {
		Err(e) => {
			eprintln!("Failed to enumerate serial ports: {}", e);
			std::process::exit(1);
		},
		Ok(ports) => {
			eprintln!("Found {} ports", ports.len());
            println!("Connecting to {}",ports[0].display());

            
            let port = SerialPort::open(&ports[0], 115200)?;
            
                return Ok(port);
                        
                    },
	}


}

// Main function
fn main() {

// RST controller reference and delay lines
let mut refe=-1.2;
let mut DELAYS=[0.0f64;2];
let mut DELAYT=[0.0f64;2];
let mut DELAYE=[0.0f64;2];

const N:usize=100;
let mut t=vec![0.0;N];
let mut u_vec=vec![0.0;N];
let mut y_vec=vec![0.0;N];
let mut r_vec=vec![0.0;N];

let stdout = stdout();
let mut stdout = stdout.lock().into_raw_mode().unwrap();
let mut stdin = async_stdin().bytes();


for i in 0..u_vec.len(){
    t[i]=i as f64;
}

let mut gnuplot=start_gnuplot().expect("");

// Connect to ESP32
let port=connect().expect("Failed connecting");
let sampling_time = time::Duration::from_micros(TS);

// Endless loop
loop {
    
    // Read plant output
    let y=read_output(&port).expect("Failed reading");

    // Compute control signal
    let u=rst(refe,y,K,&R,&S,&T,&mut DELAYT,&mut DELAYS,&mut DELAYE,2);
    
    //Write control signal
    write_control(&port, u).expect("Failed writing");

    //println!("{},{}",y,u);

    u_vec.remove(0);
    u_vec.push(u);
    y_vec.remove(0);
    y_vec.push(y);
    r_vec.remove(0);
    r_vec.push(refe);

    plot(&t,&u_vec,&y_vec,&r_vec,&mut gnuplot);
    
    let b = stdin.next();

    match b {
      Some(Ok(c))=>{
        match c{
          b'q'=>break,
          56=>refe=if(refe+0.1)>=2.5 {2.5}else{refe+0.1},
          50=>refe=if(refe-0.1)<=-2.5 {-2.5}else{refe-0.1},
          _=>println!("{c}"),
        }
      },
      Some(Err(e))=>(),
      None => ()
    }
       
    
    // Sleep one sampling period
    thread::sleep(sampling_time);
 
}
 

    }



    
  
