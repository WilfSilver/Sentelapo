mod node;
mod phrase;
mod field_type;
mod processor;

use crate::processor::Processor;

fn main() {
    let mut processor = Processor::new("impersonation");

    println!("Compiling processor...");
    match processor.init() {
        Ok(_)  => println!("Processor compiled successfully!"),
        Err(e) => panic!("Processor failed to compile: {}", e),
    }

    while true {
        println!("Input:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.pop(); // Remove newline at end
        let result = processor.process(&input);
        println!("Task: '{}'\nConfidence: {}\nLength: {}", result.get_task(), result.get_confidence(), result.len());
        for (field, values) in result.get_fields() {
            println!("{}: {:?}", field, values);
        }
    }
}