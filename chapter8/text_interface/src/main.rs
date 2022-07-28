use std::io;
use std::collections::HashMap;

fn main() -> ! {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    loop{
        let mut user_input = String::new();
        let mut word = String::new();
        let mut word_idx_start = 0;
        let mut word_idx_end = 0;
        let mut counter = 0;
        let mut name = String::new();
        let mut department = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        for letter in user_input.chars(){
            if letter == ' '{
                word = user_input[word_idx_start..word_idx_end].to_string();
                println!("{}",word);
                word_idx_end += 1;
                word_idx_start = word_idx_end;
                match counter {
                    0 => {
                        if word.eq("List"){
                            list_departments(&map);
                            println!("Test");
                            continue
                        }
                    }
                    1 => {
                        name = word;
                    }
                    _ => {
                        println!("Nothing happens.");
                    }
                }
                counter += 1;
            }else{
                word_idx_end += 1;
            }
        }
        if (name.is_empty()){
            continue;
        }
        department = user_input[word_idx_start..word_idx_end-1].to_string();
        map.entry(department)
            .or_default()
            .push(name);
    }
}

fn list_departments(map: &HashMap<String, Vec<String>>){
    for department in map.iter(){
        for name in department.1{
            println!("{} works in {}", name, department.0)
        }
    }
}
