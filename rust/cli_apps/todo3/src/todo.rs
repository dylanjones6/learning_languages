use std::fmt;
//use std::fs;
use std::io::{Write, Read, ErrorKind};
//use std::num::ParseIntError;
use std::fs::/*{*/OpenOptions/*, File}*/;
use serde::{Serialize, Deserialize};
use colored::Colorize;

// state == 0 unchecked, state == 1 checked    oooorrrr just delete it entirely, state == 2 deleted
#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub name: String,
    pub checked: bool,
}

impl TodoItem {
    pub fn new(name: String) -> Self {
        TodoItem {
            name,
            checked: false,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn get_state(&self) -> bool {
        self.checked
    }

    pub fn change(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn check(&mut self) {
        self.checked = true;
    }

    pub fn uncheck(&mut self) {
        self.checked = false;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    list: Vec<TodoItem>,
    file_path: String,
    // bak_path: String,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            list: Vec::new(),
            file_path: "/tmp/todo.tmp".to_owned(),
            // bak_path: "/tmp/todo.bak".to_owned(),
        } }

    pub fn push(&mut self, new_item: TodoItem) {
        self.list.push(new_item)
    }

    pub fn pop(&mut self) -> Option<TodoItem, > {
        self.list.pop()
    }

    pub fn remove(&mut self, ind: usize) /*-> Option<TodoItem, >*/ {
        self.list.remove(ind);
    }

    pub fn find(&self, name: &String) -> Option<usize, > {
        let mut i: usize = 0;
        for list_item in &self.list {
            //let name = &list_item.name;
            //println!("{name}");
            if &list_item.name == name {
                return Some(i)
            }
            i += 1;
        }
        return None
    }

    pub fn write(&self) {
        let serialized_list = serde_json::to_string(&self).unwrap();
        //println!("serialized list: {serialized_list}");
        let file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .truncate(true)
                        //.create(true)
                        .open(&self.file_path);
        match file {
            Ok(mut file) => {
                file.write_all(serialized_list.as_bytes()).unwrap();
            }
            Err(e) => {
                panic!("Problem writing to file: {e}");
            }
        }
    }
    
    pub fn read2() -> TodoList {
        let list_in = match OpenOptions::new()
                                .read(true)
                                .open("/tmp/todo.tmp") {
            Ok(mut file) => {
                let mut contents = String::new();
                let _ = file.read_to_string(&mut contents);
                match serde_json::from_str::<TodoList>(&contents) {
                    Ok(list_in) => list_in,
                    Err(error) => {
                        eprintln!("Unable to read file, creating a blank: {error}");
                        let empty_list = TodoList::new();
                        let serialized = serde_json::to_string(&empty_list).unwrap();
                        let list_temp = serde_json::from_str::<TodoList>(&serialized).unwrap();
                        list_temp.write();
                        list_temp
                    }
                }
            }
            // Err(error) => match error.kind() {
            //     ErrorKind::NotFound => {
            //         //eprintln!("");
            //         let empty_list = TodoList::new();
            //         let serialized = serde_json::to_string(&empty_list).unwrap();
            //         serde_json::from_str::<TodoList>(&serialized).unwrap()
            //     }
            //     _ => 

            // }
            Err(_) => {
                let empty_list = TodoList::new();
                let serialized = serde_json::to_string(&empty_list).unwrap();
                let list_temp = serde_json::from_str::<TodoList>(&serialized).unwrap();
                list_temp.write();
                list_temp
            }
        };
        list_in
    }


    pub fn read() -> Option<TodoList, > {
        let file = OpenOptions::new()
                        .read(true)
                        .open("/tmp/todo.tmp");
        let list_in = match file {
            Ok(mut file) => {
                println!("test print 1");
                // let mut contents = String::new();
                // let _ = file.read_to_string(&mut contents);
                let contents = std::fs::read_to_string("/tmp/todo.tmp").unwrap();
                println!("{}", &contents);
                // let contents = contents.trim();
                // println!("here's the thing: {contents}");
                // println!("{:?}", serde_json::from_str::<TodoList>(&contents));
                serde_json::from_str(&contents)
                // println!("testing again");
                // let test = serde_json::from_str(&contents);
                // println!("testing even omre");
                // println!("{:?}", test);
                // println!("stuff");
                // test

            },
            // Err(_) => panic!("Couldn't read old file"),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => /*match File::create("/tmp/todo.tmp")*/ {
                    println!("test print 2");
                    // let empty_contents = String::new();
                    // serde_json::from_str(&empty_contents)
                    let empty_list = TodoList::new();
                    let serialized = serde_json::to_string(&empty_list).unwrap();
                    serde_json::from_str(&serialized)
                }, 
                _ => {
                    println!("test print 3");
                    panic!("Problem creating new todo list");
                },
            }
            // Err(error) => match error.kind() {
            //     ErrorKind::NotFound => match File::create("/tmp/todo.tmp") {
            //         Ok(new_file) => new_file,
            //         Err(e) => panic!("Problem creating new file: {e}"),
            //     },
            //     _ => {
            //         panic!("Problem opening the file!");
            //     }
            // },
        };
        list_in.unwrap()
    }
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut counter = 1;
        // writeln!(f, "{}", self.name);
        // writeln!(f, "------------------");
        for i in &self.list {
            write!(f, "{}  {}\n", counter, i.name).unwrap();
            counter += 1;
        }
        Ok(())
    }
}

pub fn list() {
    let list_in = TodoList::read2();//.unwrap();
    // println!("\n{list_in}")
    let mut counter = 1;
    for list_item in list_in.list {
        match list_item.checked {
            true => {
                println!("{}{}{}", counter.to_string().strikethrough(), "  ".to_string().strikethrough(), list_item.name.strikethrough());
            },
            false => {
                println!("{}  {}", counter, list_item.name);
            },
        }
        // if list_item.checked == true {
        //     println!("{}  {}", counter, list_item.name.strikethrough());
        // } else {
        //     println!("{}  {}", counter, list_item.name);
        // }
        counter += 1;
    }
}

// implement for multiple entries instead of just one
pub fn add(item_name: String) {
    // let new_item = TodoItem::new(item_name);
    let mut list_in = TodoList::read2();//.unwrap();
    if list_in.find(&item_name) == None {
        let new_item = TodoItem::new(item_name);
        list_in.push(new_item);
        list_in.write();
    } else {
        eprintln!("The entry \"{item_name}\" is already in the list, try something else");
    }
}

pub fn check(item_name: String) {
    // gotta change the println! or make a new print method that checks if checked or not
    let mut list_in = TodoList::read2();//.unwrap();
    let ind: Option<usize, > = match item_name.parse::<usize>() {
        Ok(num) => {
            if num <= list_in.list.len() && num > 0 {
                Some(num - 1)
            } else {
                eprintln!("Given index is outside of todo list!");
                None
            }
            //println!("{num}");//num,
            //Some(num)
        }
        Err(not_int) => match not_int.kind() {
            std::num::IntErrorKind::InvalidDigit => match list_in.find(&item_name) {
                Some(num) => {
                    println!("invalid digit matched and index found {num}");
                    Some(num)
                },
                None => {
                    println!("no index found");
                    None
                },
            }
            //std::num::
            _ => {
                println!("some other error: {not_int}");
                None
            }
        }
    };
    match ind {
        Some(ind) => {
            // println!("checking off item");
            list_in.list[ind].checked = true;
            // println!("{}", list_in.list[ind].checked)
        },
        None => {
            eprintln!("Given argument didn't match anything in list");
        }
    }
    list_in.write();
}

pub fn remove() {


}

