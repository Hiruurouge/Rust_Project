/*
// check if in the command you have white space
fn string_contain_whitespace(command: &str) -> bool{
    command.contains(char::is_whitespace)
}

//return arguments 

/* chech if the parameter contains whitespace and return tab*/
fn get_arguments(s: &str) -> Vec<String>{
    let mut tab:Vec<String> = Vec::new();
    if string_contain_whitespace(s){
        let bytes = s.as_bytes();
        let mut c = 0;
        for (i, &item) in bytes.iter().enumerate(){
            if item == b' ' || item == b'\0'{
                tab.push(s[c..i].to_string());
                c = i+1;
            }        
        } 
        tab.push(s[c..].to_string());
        return tab;
    }
    tab.push(s.to_string());
    tab
}


fn execute_commands(command: &str, arg: Vec<String>) -> Resultat {
    
    //  runs the program and returns the output
    let output = if arg.is_empty(){
        Command::new(command) //rajouter .to_string() pour la convertion en STRING
                .output()
                .expect("failed to execute process")
    }else{
        Command::new(command) //rajouter .to_string() pour la convertion en STRING
                .args(arg)
                .output()
                .expect("failed to execute process")
    };
    
    
    let s = output.status.to_string();
    let o = String::from_utf8_lossy(&output.stdout).to_string();
    let e = String::from_utf8_lossy(&output.stderr).to_string();   
    let results = create_resultat(s, o, e);
    

    /*
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    */
    //assert!(output.status.success());  
    results
} 

fn main(){
    //let mut tab = get_arguments(tab[i]);
    //let c = tab[0].clone();
    //tab.remove(0);
}

*/