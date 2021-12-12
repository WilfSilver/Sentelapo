use crate::phrase::Phrase;
use crate::node::NodeResult;

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    nodes: Vec<Box<Self>>,
    phrase: Rc<RefCell<dyn Phrase>>,
    task: String,
}

impl Node {
    pub fn new(phrase: Rc<RefCell<dyn Phrase>>, task: &String) -> Node {
        Node {
            nodes: Vec::new(),
            phrase: phrase,
            task: task.clone(),
        }
    }

    pub fn new_from_phrases(phrases: &[Rc<RefCell<dyn Phrase>>], task: &String) -> Result<Node, String>{
        match phrases.get(0) {
            Some(phrase) => {
                if phrases.len() > 2 {
                    match Node::new_from_phrases(&phrases[1..], task) {
                        Ok(node) => Ok(Node {
                                        nodes:  vec![Box::new(node)],
                                        phrase: phrase.clone(),
                                        task:   String::new(),
                                    }),
                        Err(s)   => Err(s),
                    }
                } else if phrases.len() == 2 {
                    Ok(Node {
                        nodes:  vec![Box::new(Node::new(phrases[1].clone(), task))],
                        phrase: phrase.clone(),
                        task:   String::new(),
                    })
                } else {
                    Ok(Node {
                        nodes:  Vec::new(),
                        phrase: phrase.clone(),
                        task:   task.clone(),
                    })
                }
            }
            None => Err("No phrases found in slice ref when creating node".to_string()),
        }
    }

    pub fn input(&self, input: &[&str]) -> NodeResult {
        let my_result = self.phrase.borrow().matches(input);

        if my_result.len() == 0 {
            return NodeResult::new(&String::new(), 0.0, 0);
        } else if my_result.len() == input.len() && !self.task.is_empty()  {
            println!("Found: {}", self.task);
            return NodeResult::new_from_phrase(&self.task, &my_result);
        }

        let next_input: &[&str] = &input[my_result.len()..];

        let mut result_conf = 0.0;
        let mut result_length = 0;

        if !self.task.is_empty() {
            result_conf   = my_result.get_confidence();
            result_length = my_result.len();
        }

        let mut changed = false;
        // Stores all the tasks that return that are equal
        let mut potential_results: Vec<NodeResult> = vec![NodeResult::new(&self.task, result_conf, result_length)];

        // Saves a bit of processing later
        if self.nodes.len() == 0 {
            return potential_results.remove(0);
        }

        let mut task_count: HashMap<String, u32> = HashMap::new();

        for node in &self.nodes {
            let r = node.input(next_input);

            if r.get_score() > potential_results[0].get_score() {
                changed = true;
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

        if result_index != 0 {
            changed = true;
        }

        let mut result = potential_results.remove(result_index);

        if changed {
            result.mult_confidence(my_result.get_confidence());
            result.add_length(my_result.len());
            result.add_fields(my_result.get_fields());
        }

        return result;
    }

    pub fn add_nodes(&mut self, phrases: &[Rc<RefCell<dyn Phrase>>], task: &String) -> Result<bool, String> {
        match phrases.get(0) {
            Some(v) => {
                if Rc::ptr_eq(&self.phrase, v) {
                    if phrases.len() == 1 {
                        if !self.task.is_empty() && self.task.ne(task) {
                            return Err(format!("Cannot compile because '{}' and '{}' has same input", self.task, task));
                        }
                        self.task = task.clone();
                        return Ok(true);
                    }

                    for node in &mut self.nodes {
                        match node.add_nodes(&phrases[1..], task) {
                            Ok(has_matched) => {
                                if has_matched {
                                    return Ok(true);
                                }
                            }
                            Err(e) => return Err(e),
                        }
                    }

                    match Node::new_from_phrases(&phrases[1..], task) {
                        Ok(node) => self.nodes.push(Box::new(node)),
                        Err(e)   => return Err(e),
                    }

                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }
            None => return Err("Given phrases of length 0".to_string()),
        }
    }

    pub fn get_dump_string(&self) -> String {
        let mut output = format!("Task '{}' | Phrase '{}' | Nodes |", self.task, self.phrase.borrow().get_name());
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