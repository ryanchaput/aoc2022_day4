use std::fs;

fn main() {
    //Represents the number of pairs where one range is fully within the other
    let mut _count = 0;

    let file_path = "day4_puzzle.txt";

    let _contents = fs::read_to_string(file_path)
    .expect("Should've read the file.");

    for line in _contents.lines() {
        let ends: Vec<i32> = line.split(&['-', ','])
        .map(|x| x.parse::<i32>().expect("Expected an i32"))
        .collect();
        
        if check_range(ends) {
            _count += 1;
        }
    }

    println!("The count is: {_count}");
}

fn check_range(r: Vec<i32>) -> bool {
    let a1 = r.get(0);
    let a2 =  r.get(1);
    let b1 = r.get(2);
    let b2 = r.get(3);

    let result = (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2);
    
    result
}
