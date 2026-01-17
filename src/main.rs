use std::env;
use std::mem;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1].is_empty(){
        help();
        return;
    } else{
        match args[1].as_str(){
            "--lns" => linear_search(),
            "--bys" => binary_search(),
            "--bts" => binary_tree(),
            "--bls" => bubble_sort(),
            _ => {
                println!("Command {} unsuported", args[1]);
                help();
            }
        }
    }
}

fn help(){
    println!("Algorithm practice project");
    println!("Options:");
    println!("Example of use: (./main -ln)");
    println!("      -h    : Prints this message,");
    println!("      --lns : Linear search,");
    println!("      --bys : Binary search,");
    println!("      --bts : Binary tree search,");
    println!("      --bls : Bubble sort.");
    println!();
}

fn binary_search(){
    let array: [i32; 7] = [1, 4, 81, 90, 103, 153, 241];
    let target: i32 = 241;
    let mut last: usize = (array.len() / array[0].
        to_string().len()) - 1;

    let mut found: i32 = 0;
    let mut first: i32 = 0;

    println!("Binary search:");
    println!("target: {target}");
    println!("last pos: {last}");
    println!();

    while (first as usize) <= (last) && (found == 0){
        let midpoint: i32 = (first + last as i32) / 2;

        if array[midpoint as usize] == target{
            found = 1;
            println!("Found at index {midpoint}");
            println!();
        } else {
            if array[midpoint as usize] < target {
                first = midpoint + 1;
            } else {
                last = midpoint as usize - 1;
            }
        }
    }
}

fn linear_search(){
    let array: [i32; 7] = [9, 2, 7, 1934, 13, 200, 1];
    let target: i32 = 200;
    let last: usize = (array.len() / array[0].
        to_string().len()) - 1;

    let mut index: i32 = 0;
    let mut found: i32 = 0;

    println!();
    println!("Linear search:");
    println!("target: {target}");
    println!("end pos: {last}");
    println!();

    while (index as usize) < (last) || (found != 1) {
        if array[index as usize] == target {
            found = 1;
            println!("Found at index {index}");
        }
        index = index + 1;
    }
}

fn binary_tree(){
    // target
    // current_node - I'm assuming the left/right child
    //               are being passed as current_node
    
    let target: i32 = 71;
    let mut current_node: i32 = 8;
    binary_tree_search(target, current_node);
}

fn binary_tree_search(target: i32, mut current_node: i32){
    // I don't know how to represent a bin tree as a data
    // structure, this is severely changing the whole point.
    let array: [i32; 14] = [9, 12, 14, 27, 28, 35, 41, 50, 52, 29, 60, 66, 68, 71]; 
    let last: usize = (array.len() / array[0].
        to_string().len()) - 1;
    
    let mut found: bool = false;
    let current_node_val: i32 = 50;

    if target == current_node_val {
        found = true;
    }

    else if target < current_node_val{
        current_node = current_node - 1;
        if array[current_node as usize] == 0 {
            return binary_tree_search(target, current_node);
        } else {
            found = false;
        }
    }

    current_node = current_node + 1;
    if array[current_node as usize] == 0 {
        return binary_tree_search (target, current_node);
    } else {
        found = false;
    }
}

fn bubble_sort() {
    let mut array: [i32; 7] = [9, 2, 7, 1934, 13, 200, 1];
    let len: usize = array.len();
 
    println!();
    println!("Bubble sort:");
    println!("OG array:");

    for i in 0..len{
        let j: i32 = array[i as usize];
        println!("{j}");
    } println!();

    for index in 0..len {
        let mut swapped: bool = false;
        
        for j in 0..(len-index-1) {
            if array[j as usize] > array[(j + 1) as usize] {
                array.swap(j, j+1);
                swapped = true;
            }
        } if swapped == false {
            break;
        }
    }

    println!("New array:");
    for i in 0..len{
        let j: i32 = array[i as usize];
        println!("{j}");
    }

}





