use crate::field_type;
use crate::field_type::FieldType;
use crate::field_type::Custom;
use crate::node::BaseNode;
use crate::node::NodeResult;
use crate::phrase::Phrase;
use crate::phrase::Field;
use crate::phrase::GroupPhrase;
use crate::phrase::WordPhrase;
use crate::phrase::MetaPhrase;

use regex::Regex;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Processor {
    base_node:     BaseNode,
    compiling:     Vec<String>, // Vector of strings describing what is currently being compiled, e.g. "phrase:greeting", "field_type:company"
    config_path:   String,
    field_to_type: HashMap<String, String>,
    field_types:   HashMap<String, Rc<RefCell<dyn FieldType>>>,
    fields:        HashMap<String, Rc<RefCell<Field>>>,
    phrases:       HashMap<String, Rc<RefCell<dyn Phrase>>>,
}

impl Processor {
    pub fn new(config: &str) -> Processor {
        Processor {
            base_node:     BaseNode::new(),
            compiling:     Vec::new(),
            config_path:   format!("./config/{}", config),
            field_to_type: HashMap::new(),
            field_types:   HashMap::new(),
            fields:        HashMap::new(),
            phrases:       HashMap::new(),
        }
    }

    pub fn init(&mut self) -> Result<bool, String> {
        println!("Compiling field_types...");
        let contents = std::fs::read_to_string(self.config_path.clone() + "/field_types.var").expect("");

        for line in contents.split("\n") {
            let pair: Vec<&str> = line.split(":").collect();
            if pair.len() != 2 {
                return Err("Invalid definition of field in 'field_types.var' file".to_string());
            }
            self.field_to_type.insert(pair[0].to_string(), pair[1].to_string());
        }

        println!("Compiling Phrases...");
        let paths = std::fs::read_dir(self.config_path.clone() + "/phrases").expect("Could not open config phrases directory");
        for file in paths {
            let path = file.unwrap().path();
            if path.extension().unwrap() == "td" {
                let phrase: String = path.file_stem().unwrap().to_str().unwrap().to_string();
                self.get_phrase(&phrase)?; // This will compile the phrases if needed (as the could already be compiled)
            } else {
                println!("Skipping '{}'", path.display());
            }
        }

        println!("Compiling tasks");
        self.compile_nodes()?;

        // For debugging purposes
        let mut file = std::fs::File::create("node_data.txt").expect("create failed");
        file.write_all(self.base_node.get_dump_string().as_bytes()).expect("write failed");

        return Ok(true);
    }

    pub fn process(&self, input: &str) -> NodeResult {
        self.base_node.input(&input.split(' ').collect::<Vec<&str>>()[..])
    }

    fn compile(&mut self, phrases: &Vec<&str>, name: &String) -> Result<Rc<RefCell<dyn Phrase>>, String> {
        let mut group_ref = RefCell::new(GroupPhrase::new(name, Vec::new()));

        let group = group_ref.get_mut();

        for phrase in phrases {
            match self.compile_line(phrase) {
                Ok(line) => {
                    if line.len() == 1 {
                        group.push(line[0].clone());
                    } else {
                        group.push(Rc::new(RefCell::new(MetaPhrase::new(&String::new(), line))));
                    }
                }
                Err(e)   => return Err(e),
            }
        }

        return Ok(Rc::new(group_ref));
    }

    fn compile_line(&mut self, line: &str) -> Result<Vec<Rc<RefCell<dyn Phrase>>>, String> {
        // Should be safe to unwrap here as the regex is predefined
        let regex = Regex::new(r"\{[a-zA-Z0-9\-_]+\}").unwrap();

        let mut offset = 0;
        let mut rest: &str = line;
        let mut line_vec: Vec<Rc<RefCell<dyn Phrase>>> = Vec::new();

        for m in regex.find_iter(line) {
            // Removes any spaces at the beginning
            if rest.get(0..1) == Some(" ") {
                rest = &rest[1..];
                offset += 1;
            }

            // If there are characters before the match starts, then add them to the group
            if m.start() - offset > 1 {
                let mut content = rest[..m.start() - offset].to_string();

                // Removes trailing spaces
                if content.chars().last() == Some(' ') {
                    content.pop();
                }

                let temp: Rc<RefCell<dyn Phrase>> = Rc::new(RefCell::new(WordPhrase::new(&content, content.clone())));
                line_vec.push(temp.clone());
            }

            // Get name by removing the first and last characters
            let name = rest[m.start() + 1 - offset .. m.end() - 1 - offset].to_string();

            // This could be a field or a phrase, so check if it is in list of fields and add appropriate phrase
            if self.field_to_type.contains_key(&name) {
                match self.get_field_phrase(&name) {
                    Ok(field) => line_vec.push(field as Rc<RefCell<dyn Phrase>>),
                    Err(e)    => return Err(e),
                }
            } else {
                match self.get_phrase(&name) {
                    Ok(phrase) => line_vec.push(phrase),
                    Err(e)     => return Err(e),
                }
            }

            rest   = &rest[m.end() - offset..];
            offset = m.end();
        }

        if offset != line.len() {
            let temp: Rc<RefCell<dyn Phrase>> = Rc::new(RefCell::new(WordPhrase::new(&rest.to_string(), rest.to_string())));
            line_vec.push(temp.clone());
        }

        if line_vec.len() == 0 {
            return Err("Could not find any phrases in line".to_string());
        }

        return Ok(line_vec);
    }

    fn compile_phrase(&mut self, name: &String) -> Result<Rc<RefCell<dyn Phrase>>, String> {
        println!("Compiling phrase: {}", name);

        let compiling_name = "phrase:".to_string() + name;
        if self.compiling.contains(&compiling_name) {
            return Err(format!("Cannot compile '{}' because it is already compiling", name));
        }
        self.compiling.push(compiling_name.clone());

        let file_path = self.config_path.clone() + format!("/phrases/{}.td", name).as_str();
        let contents = std::fs::read_to_string(&file_path).expect("");

        if contents == "" {
            return Err(format!("Could not read '{}'",  file_path));
        }

        let result = self.compile(&contents.split("\n").collect(), name);

        if self.compiling.pop() != Some(compiling_name) {
            return Err(format!("Cannot remove '{}' as it is not last in compiling list", name));
        }

        return result;
    }

    fn compile_field_type(&mut self, name: &String) -> Result<Rc<RefCell<dyn FieldType>>, String> {
        println!("Compiling Field Type: {}", name);

        let compiling_name = "field_type:".to_string() + name;
        if self.compiling.contains(&compiling_name) {
            return Err(format!("Cannot compile '{}' because it is already compiling", name));
        }
        self.compiling.push(compiling_name.clone());

        let mut field_type_ref = RefCell::new(Custom::new());
        let field_type = field_type_ref.get_mut();

        let file_path = self.config_path.clone() + format!("/field_types/{}.td", name).as_str();
        let contents = std::fs::read_to_string(&file_path).expect("");

        if contents == "" {
            return Err(format!("Could not read '{}'",  file_path));
        }

        for line in contents.split("\n") {
            let phrases: Vec<&str> = line.split(":").collect();

            match phrases.get(0) {
                Some(syn_name) => {
                    match self.compile(&phrases, name) {
                        Ok(compiled) => field_type.insert_synonym(syn_name.to_string(), compiled),
                        Err(e)       => return Err(e),
                    }
                }
                None => continue,
            }
        }

        if self.compiling.pop() != Some(compiling_name) {
            return Err(format!("Cannot remove '{}' as it is not last in compiling list", name));
        }

        return Ok(Rc::new(field_type_ref));
    }

    fn compile_nodes(&mut self) -> Result<bool, String> {
        let paths = std::fs::read_dir(self.config_path.clone() + "/tasks").expect("Could not open config task directory");

        for file in paths {
            let path = file.unwrap().path();
            if path.extension().unwrap() == "task" {
                let task: String = path.file_stem().unwrap().to_str().unwrap().to_string();
                let contents = std::fs::read_to_string(path).expect("Could not read contents of task file");

                let lines: Vec<&str> = contents.split("\n").collect();

                println!("Compiling Task: {}", task);
                if lines.len() > 0 && lines[0] != "" {
                    for line in &lines {
                        match self.compile_line(line) {
                            Ok(line_vec) => { self.base_node.add_nodes(&line_vec[..], &task)?; }
                            Err(e) => return Err(e),
                        }
                    }
                }
            } else {
                println!("Skipping '{}'", path.display());
            }
        }

        return Ok(true);
    }

    fn create_field(&mut self, name: &String) -> Result<Rc<RefCell<Field>>, String> {
        let field_type: String;
        match self.field_to_type.get(name) {
            Some(ft_name) => {
                match self.field_types.get(ft_name) {
                    Some(ft) => {
                        let field = Rc::new(RefCell::new(Field::new(&name, ft.clone())));
                        self.fields.insert(name.clone(), field.clone());
                        return Ok(field);
                    }
                    None     => field_type = ft_name.clone(), // We cannot have an immutable reference with mutable reference
                }
            }
            None => return Err(format!("Could not find field type for given type '{}'", name)),
        }

        match self.get_field_type(&field_type) {
            Ok(ft) => {
                let field = Rc::new(RefCell::new(Field::new(&name, ft.clone())));
                self.fields.insert(name.clone(), field.clone());
                Ok(field)
            }
            Err(e) => Err(e),
        }
    }

    fn get_field_phrase(&mut self, name: &String) -> Result<Rc<RefCell<Field>>, String> {
        match self.fields.get(name) {
            Some(field) => Ok(field.clone()),
            None        => self.create_field(name),
        }
    }

    fn get_field_type(&mut self, name: &String) -> Result<Rc<RefCell<dyn FieldType>>, String> {
        match self.field_types.get(name) {
            Some(field_type) => Ok(field_type.clone()),
            None => {
                match field_type::get_default_type(name) {
                    Some(field_type) => Ok(field_type.clone()),
                    None             => {
                        match self.compile_field_type(name) {
                            Ok(field_type) => {
                                self.field_types.insert(name.clone(), field_type.clone());
                                Ok(field_type)
                            }
                            Err(e) => Err(e),
                        }
                    }
                }
            }
        }
    }

    fn get_phrase(&mut self, name: &String) -> Result<Rc<RefCell<dyn Phrase>>, String> {
        match self.phrases.get(name) {
            Some(phrase) => Ok(phrase.clone()),
            None => {
                match self.compile_phrase(name) {
                    Ok(phrase) => {
                        self.phrases.insert(name.clone(), phrase.clone());
                        Ok(phrase)
                    }
                    Err(e)     => Err(e),
                }
            }
        }
    }
}