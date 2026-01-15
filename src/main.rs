fn main() {
    binary_search();
    linear_search();
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

fn bubble_sort(){

}


