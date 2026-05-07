use std::env;
use std::collections::HashMap;
use std::collections::binary_heap::Iter;
use std::fs;
// use std::io;

fn main() {
    fetch_input();
}

fn fetch_input() {
    let args: Vec<String> = env::args().collect();
    
    if args.is_empty() {
        println!("No command provided");
        help();
        return;
    }

    if args.len() > 1 {
        if args.len() > 2 {
            let f_path: String = args[2].clone();
            match args[1].as_str() {
                "-e" => emulate(f_path),
                "-a" => assemble(f_path),
                _ => {
                    println!("Unkown command: {}", args[1]);
                    help();
                }
            }
        } else {
            let f_path: String = String::new();
            match args[1].as_str() {
                "-e" => emulate(f_path),
                "-a" => assemble(f_path),
                _ => {
                    println!("Unkown command: {}", args[1]);
                    help();
                }
            }
        }
    } else {
        help();
    }

}

fn help() {
    println!("!!!Little Man's Computer Collection!!!");
    println!();
    println!("Use example: $ lmcc -e program.bin");
    println!("Options:");
    println!("      -e : Emulate a program,");
    println!("      -a : Assemble a program,");
    println!("          Example: $ lmcc -a program.lmc.");
    println!("      -h : Print this message,");
    println!("      -v : Print version number.");
    println!();
}

// The underscore before "f_path" shows the compiler that 
// the unused variable is intentional.
fn emulate(_f_path: String){ 
    println!("Emulator");
}

fn assemble(_f_path: String){
    println!("Assembler");
    tokenization(_f_path);
}

fn tokenization(f_path: String){
    let source: String;

    if !f_path.is_empty(){
        source = fs::read_to_string(f_path)
             .expect("Error (I think anyway lol)");
    } 
    else {
        source = "ADD R2 R1 10 HALT".to_string();
    }

    println!("Recieved source:");
    println!(":ASM START");
    println!("{source}");
    println!(":ASM END");
    println!();

    /* 
    1:  LocationCounter ← 0
    2:  Get first line of source code.
    3:  while more lines do
    4:   if line has a label then
    5:   SymbolTable.Symbol ← label
    6:   SymbolTable.Location ← LocationCounter
    7:   end if
    8:   Determine number of bytes required by the line when assembled.
    9:   LocationCounter ← LocationCounter + NumberOfBytes
    10:  Get next line of source code.
    11: end while 
    */

    let instr: String = String::new();
    let mut token: Vec<String> = Vec::new();
    let mut _len: usize = instr.len();
    let mut _loc_counter: u32 = 0;

    for word in source.split_whitespace(){
        token.push(word.to_string());
    }
    
    let token_iter: std::slice::Iter<'_, String> = token.iter();

    stream(token_iter);

}

fn stream(token_iter: std::slice::Iter<'_,String>){
    
    // Inputs required to be inserted to the table:
    // - Type
    // - Memory adress
    // - Value
    // - Additional info (data type etc)
    //
    // Struc:
    // Category, Location of more info
    // Address, Location of more info
    // Value, Location of more info
    // More, next

    let mut s_table: HashMap<Vec<i32>, String> = HashMap::new();

    for i in token_iter {
        let category: String = String::new();
        semantic_analysis(s_table.clone(), i, category);
    }

}

fn semantic_analysis(s_table: HashMap<Vec<i32>, String>, i: &String, category: String){

}
    
    // Architecture:
    // LDR RResult <memory ref>
    // STR RResult <memory ref>
    // ADD RResult RAgainst <operand>
    // SUB RResult RAgainst <operand>
    // MOV RResult <operand>
    // CMP RAgainst <operand>
    // B <label>
    // B <condition> <label>
    // HALT
    //
    // AND RResult RAgainst <operand>
    // ORR RResult RAgainst <operand>
    // EOR RResult RAgainst <operand>
    // MVN RResult RAgainst <operand>
    // LSL RResult RAgainst <operand>
    // LSR RResult RAgainst <operand>
    //
    // The static analyzer will look at the first keyword 
    // (ex. ADD) and then will pull in the expected length 
    // of the instruction until HOLT is met. 
    //

    // loop through each instruction
    // compare against arch

