use std::mem;

fn main() {
    binary_search();
    linear_search();
    // binary_tree_sort();
    bubble_sort();
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

fn binary_tree_sort(){
    let array: [i32; 14] = [9, 12, 14, 27, 28, 35, 41, 50, 52, 29, 60, 66, 68, 71]; 
    let target: i32 = 71;
    let last: usize = (array.len() / array[0].
        to_string().len()) - 1;
    
    let mut found: bool = false;
    let current_node: i32 = 50;
    let current_node_pos: i32 = 8;

    while found == false {
        if target == current_node {
            found = true;
        }
        else if target < current_node{
            /*
             * if left child exists
            if array[].contains(){

            }
            */
        }
    }
}

fn bubble_sort() {
    let array: [i32; 7] = [9, 2, 7, 1934, 13, 200, 1];
    let last: usize = (array.len() / array[0].
        to_string().len()) - 1;
    let index: i32 = 0;
    let j: i32 = 0;
    let mut swapped: bool = false;
    println!();
    println!("Bubble sort:");

    while swapped == false {
        let len: i32 = (last as i32)-2;
        for index in 0..len {
            for j in 0..len-index-2 {
                if array[j as usize] > array[(j + 1) as usize] {
                    let mut x = array[(index + 1) as usize];
                    let mut y = array[index as usize];
                    println!("OG X: {x}");
                    println!("OG Y: {y}");
                    mem::swap(&mut x, &mut y);
                    println!("Swapped X: {x}");
                    println!("Swapped Y: {y}");
                    swapped = true;
                }
            }
        }
    }
}





