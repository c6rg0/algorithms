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
            "-ts" => two_sum(),
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
    
    let current_node_val: i32 = 50;

    if target == current_node_val {
        let found: bool = true;
    }




    else if target < current_node_val{
        current_node = current_node - 1;
        if array[current_node as usize] == 0 {
            return binary_tree_search(target, current_node);
        } else {
            let found: bool = true;
        }
    }

    current_node = current_node + 1;
    if array[current_node as usize] == 0 {
        return binary_tree_search (target, current_node);
    } else {
        let found: bool = true;
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

struct Solution {
    return_array: Vec<i32>,
    return_palidrome: i32,
}

impl Solution{
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut i: i32 = 0;
        let nums_size = nums.len();
        let mut return_array: Vec<i32> = Vec::new();
        while i < nums_size as i32
        {
            let mut j: i32 = 0;
            while j < nums_size as i32
            {
                if i != j
                {
                    if nums[i as usize] + nums[j as usize] == target
                    {
                        println!("{i}{j}");
                        return_array.push(i);
                        return_array.push(j);
                    }
                }
                j = j + 1;
            }
            i = i + 1;
        }
        return_array.truncate(2);
        return_array
    }

    pub fn is_palindrome(x: i32) -> bool {
        /* most optimal solution, note: return would use less memory but for less speed
        x.to_string().chars().rev().eq(x.to_string().chars())
        */

        // my solution...
        let judgement: bool;
        let mut x_len: usize = x.to_string().len();
        if x < 0{
            x_len = x_len - 1;
        }

        let mut x_vec: Vec<i32> = Vec::new(); 
        println!("x = {x}");
        let mut x = x;
        let mut i = 0;

        while i < x_len as i32
        {
            x_vec.push(x % 10);
            i = i + 1;
            x = x / 10
        }
        x_vec.reverse();

        let first: i32;
        if x_len == 1
        {
            first = x_vec[0];
        }
        else
        {
            first = x_vec[1];
        }

    let last: i32 = x_vec[x_len - 1];

    if first > -1
    {
        if first == last{
            let mut x_vec_r = x_vec.to_vec();
            x_vec_r.reverse();
            if x_vec_r == x_vec
            {
                judgement = true;
            }
            else
            {
                judgement = false;
            }
        }
        else
        {
            judgement = false;
        }
    }
    else
    {
        judgement = false;
    }

    judgement
    }

    pub fn roman_to_int(s: String) -> i32 {
        let mut collected: i32 = 0; 
        for index in 0..s.len() {
            match s.chars().as_str() {
                "I" => collected = collected + 1,
                "V" => collected = collected + 5,
                "X" => collected = collected + 10,
                "L" => collected = collected + 50,
                "C" => collected = collected + 100,
                "D" => collected = collected + 500,
                "M" => collected = collected + 1000,
                _ => ()
            }
            println!("{collected}");
        }
        collected
    }
}




