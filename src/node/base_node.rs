use crate::node::Node;
use crate::node::NodeResult;
use crate::phrase::Phrase;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct BaseNode {
    nodes: Vec<Box<Node>>,
}

impl BaseNode {
    pub fn new() -> BaseNode {
        BaseNode {
            nodes: Vec::new(),
        }
    }

    pub fn input(&self, input: &[&str]) -> NodeResult {
        // TODO: Put this in its own function
        // Stores all the tasks that return that are equal
        let mut potential_results: Vec<NodeResult> = vec![NodeResult::new(&String::new(), 0.0, 0)];

        // Saves a bit of processing later
        if self.nodes.len() == 0 {
            return potential_results.remove(0);
        }

        let mut task_count: HashMap<String, u32> = HashMap::new();

        for node in &self.nodes {
            let r = node.input(input);

            if r.get_score() > potential_results[0].get_score() {
                task_count.clear();
                potential_results.clear();
                task_count.insert(r.get_task().clone(), 1);
                potential_results.push(r);
                // potential_results = vec![r];
            } else if r.get_score() == potential_results[0].get_score() {
                match task_count.get_mut(r.get_task()) {
                    Some(count) => *count += 1,
                    None        => { task_count.insert(r.get_task().clone(), 1); }
                }
                potential_results.push(r);
            }
        }

        // TODO: This should influence confidence
        let mut max_task = String::new();
        let mut max_count = 0;
        for (task, count) in task_count {
            if count > max_count {
                max_task = task;
                max_count = count;
            }
        }

        let mut first = true;
        let mut result_index: usize = 0;
        for i in 0..potential_results.len() {
            if potential_results[i].get_task().eq(&max_task) {
                if first || potential_results[i].len() > potential_results[result_index].len() {
                    result_index = i;
                    first = false;
                }
            }
        }

        return potential_results.remove(result_index);
    }

    pub fn add_nodes(&mut self, phrases: &[Rc<RefCell<dyn Phrase>>], task: &String) -> Result<bool, String> {
        if phrases.len() == 0 {
            return Err("0 length phrase given".to_string());
        }

        for node in &mut self.nodes {
            match node.add_nodes(&phrases, task) {
                Ok(has_matched) => {
                    if has_matched {
                        return Ok(true);
                    }
                }
                Err(e) => return Err(e),
            }
        }

        if phrases.len() > 1 {
            match Node::new_from_phrases(&phrases, task) {
                Ok(node) => self.nodes.push(Box::new(node)),
                Err(e)   => return Err(e),
            }
        } else {
            self.nodes.push(Box::new(Node::new(phrases.get(0).unwrap().clone(), task)));
        }

        return Ok(true);
    }

    pub fn get_dump_string(&self) -> String {
        let mut output = "Base Node |".to_string();
        let spaces = output.len() - 1;

        for node in &self.nodes {
            // Add given number of spaces at the beginning of each new line from the string
            let node_out = node.get_dump_string()
                            .split("\n")
                            .map(|x| " ".repeat(spaces) + "| " + x)
                            .collect::<Vec<String>>()
                            .join("\n");
            output += "\n";
            output += &node_out;
        }

        return output;
    }
}