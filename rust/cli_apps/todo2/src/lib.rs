use std::fs::{self, File, OpenOptions};
use std::io::{Write, BufRead, BufReader};
use std::fmt;

pub struct TodoItem {
    pub name: String,
    state: u8,
    // description: Option<String>,
    // due_date: Option<String>,
    // secret: String,
}

// TodoItem states: 0 == unchecked, 1 == checked, 2 == removed
impl TodoItem {
    pub fn new(name: String) -> Self {
        TodoItem {
            name,
            state: 0,
            // description: None,
            // due_date: None,
        }
    }
    pub fn get_name(&self) -> Option<&str, > {
        Some(&self.name)
    }
    pub fn get_state(&self) -> Option<&u8, > {
        Some(&self.state)
    }
}

pub struct Todo<'a> {
    name: &'a str,
    list: Vec<TodoItem>,
    file_path: String, //&'a str,
    bak_path: String, //&'a str,
}

/// To do list struct
impl Todo<'_> {
    /// Creates new instance of a to-do list
    ///
    /// # Example use
    ///
    /// ```
    /// let new_todo = Todo::new("New list");
    /// ```
    pub fn new(/*name: String*/) -> Self {

        let name = "";
        let name_upper = &name.to_uppercase();
        let file_path = format!("/tmp/{}_todo.txt", &name_upper);
        let bak_path = format!("/tmp/{}_todo.bak", &name_upper);
        Todo {
            name: &name,
            list: Vec::new(),
            file_path: String::from(&file_path),
            bak_path: String::from(&bak_path.to_string()),
        }
    }

    pub fn push(&mut self, new: TodoItem) {
        self.list.push(new)
    }

    pub fn pop(&mut self) -> Option<TodoItem, > {
        self.list.pop()
    }

    pub fn print_items(&self) {
        for i in &self.list {
            println!("{} ", i.name);
        }
    }

    pub fn print(&self) {
        //print!("\n");
        println!("{}", self.name);
        println!("------------------------");
        for i in &self.list {
            print!("-");
            println!("{}", i.name);
        }
    }

    pub fn save(&self) {
        let name = &self.name; // use if we're going to do named lists
        let name_upper = &name.to_uppercase();
        let file_path = format!("/tmp/{}_todo.txt", &name_upper);
        let bak_path = format!("/tmp/{}_todo.bak", &name_upper);
        let temp_todo = Todo::new();

        if temp_todo.check_file_exists() {
            let _ = fs::rename(&file_path, &bak_path);
        }
        
        let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(&file_path)
                    .unwrap();

        for i in &self.list {
            // println!("{},{}", i.name, i.state);
            writeln!(file, "{},{}", i.name, i.state).expect("Line couldn't be added to file")
        }
        //let mut f = File::create(&file_path);
    }

    pub fn read(&self) -> Self {
        let file = File::open(&self.file_path).unwrap();
        let reader = BufReader::new(file);
        let mut read_todo = Todo::new();
        for line in reader.lines() {
            // println!("{:?}", &line);
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(',').collect();
                let read_todo_item = TodoItem {
                    name: String::from(parts[0]),
                    state: parts[1].parse().expect("Could not parse read value"),
                };
                read_todo.push(read_todo_item);
                // println!("{:?}", parts);
            }
            // let parts = line.split(",");
            // println!("{:?}", parts);
            // if block to then split along commas and stuff 
        }
        // println!("{}", read_todo);
        read_todo
    }

    pub fn check_file_exists(&self) -> bool {
        let file = File::open(&self.file_path);
        match file {
            Ok(_) => {
                // println!("found todo.txt file!")
                true
            },
            Err(_) => false,
        }
    }
}

impl fmt::Display for Todo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut counter = 1;
        // writeln!(f, "{}", self.name);
        // writeln!(f, "------------------");
        for i in &self.list {
            write!(f, "{}  {}\n", counter, i.name);
            counter += 1;
        }
        Ok(())
    }
}

// commands for single list!
//
// add
// check
//     (empty call shows list)

pub fn list() {
    let mut temp = Todo::new();
    temp = temp.read();
    println!("{}", &temp);
}

pub fn add(args: &Vec<String>) {
    let mut temp_todo = Todo::new();
    if temp_todo.check_file_exists() {
        temp_todo = temp_todo.read()
    }
    println!("printing items in add: ");
    for item in args {
        println!("{}", item);
        let temp_item = TodoItem::new(item.to_string());
        temp_todo.push(temp_item);
    }
    temp_todo.save();
    list();
}

pub fn delete(args: &Vec<String>) {
    let mut temp_todo = Todo::new();

}

pub fn check(args: &Vec<String>) {


}
