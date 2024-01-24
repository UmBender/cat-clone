use std::env;
use std::fs::File;
use std::io::Read;

struct Cat {
    show_path: bool,
    show_tabs: bool,
    open: bool,
    data: String,
    path: String,
}

impl Cat {
    fn new(path: &String, show_path: bool, show_tabs: bool) -> Self {
        let return_data = Self {
            show_path,
            show_tabs,
            open: false,
            data: String::new(),
            path: path.clone(),
        };
        return return_data;
    }

    fn show(&self) {
        match self.open {
            true => {
                if self.show_path {
                    println!("PATH: \"{}\"", self.path.clone());
                }
                if self.show_tabs {
                    for (i, j) in self.data.split('\n').enumerate() {
                        println!("{:05} | {}", i + 1, j);
                    }
                } else {
                    for i in self.data.split('\n') {
                        println!("{}", i);
                    }
                }
            }
            false => {}
        }
    }

    fn sync(&mut self) {
        let mut file_path = match File::open(self.path.clone()) {
            Ok(i) => i,
            Err(_) => {
                self.open = false;
                return;
            }
        };
        match file_path.read_to_string(&mut self.data) {
            Ok(_) => self.open = true,
            Err(_) => {
                self.open = false;
                return;
            }
        }
    }

    fn new_sync(path: &String, show_path: bool, show_tabs: bool) -> Self {
        let mut obj = Self::new(path, show_path, show_tabs);
        obj.sync();
        return obj;
    }
}

fn main() -> std::io::Result<()> {
    if env::args().len() < 2 {
        println!("cat: fatal error: no input files");
        return Ok(());
    }
    let mut elements: Vec<String> = Vec::new();
    let mut tabs = false;
    let mut path = false;
    for (i, j) in env::args().enumerate() {
        if i == 0 {
            continue;
        }
        match j.as_str() {
            "--table" => tabs = true,
            "--path" => path = true,
            string => elements.push(String::from(string)),
        }
    }
    for i in elements {
        let teste = Cat::new_sync(&String::from(i.clone()), path, tabs);
        teste.show();
    }

    Ok(())
}
