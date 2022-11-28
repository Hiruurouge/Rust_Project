use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::fs::File;
use std::io::BufReader;
use std::fs::OpenOptions;
static  ONLINE:Vec<&String>=Vec::new();
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
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let test:String=stream.peer_addr().unwrap().to_string();
                let client_ip:&str=test.split(":").collect::<Vec<&str>>()[0];
                let file: File = File::open("src/user.txt").expect("error");
                let mut _buf_reader = BufReader::new(file);
                let mut contents = String::new();
                _buf_reader.read_to_string(&mut contents);
                let _vec = contents.split("\n").collect::<Vec<&str>>();
                let mut ok:bool=false;
                for ip in _vec.iter(){
                    let _ip:Vec<&str>=ip.split(":").collect();
                    println!("{}",_ip[0]);
                    println!("{}",client_ip);
                    if  client_ip==_ip[0]
                    {
                        println!("ici");
                        ok=true;
                    }
                }
                if ok==false{register(client_ip.clone())};
                //ONLINE.push(&stream.peer_addr().unwrap().to_string());
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