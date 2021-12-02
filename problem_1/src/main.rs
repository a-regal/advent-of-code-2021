use std::fs;

fn read_to_array(filename: &str) -> Vec<i64>{
    let content = fs::read_to_string(filename).expect("Reading error");
    let measurements: Vec<i64> = content.trim().split("\n").map(|l| l.parse().unwrap()).collect();
    return measurements;
}

fn part_1(input: &Vec<i64>) -> i64{
    let mut increases = 0;
    for i in 1..input.len(){
        if input[i] > input[i-1] {
            increases += 1;
        }
    }
    return increases;
}

fn part_2(input: &Vec<i64>) -> i64{
    let mut increases = 0;

    //Same as part 1 but with a 3-window
    for i in 0..input.len()-3{
        let win_1 = input[i] + input[i+1] + input[i+2];
        let win_2 = input[i+1] + input[i+2] + input[i+3];

        if win_2 > win_1{
            increases += 1;
        }
    }
    return increases;
}

fn main() {
    let measures = read_to_array("input.txt");
    let part_a_sol = part_1(&measures);
    let part_b_sol = part_2(&measures);
    println!("{} increases", part_a_sol);
    println!("{} increaes with 3-window", part_b_sol);
}
