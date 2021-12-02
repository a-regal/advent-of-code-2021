use std::fs;

fn read_to_array(filename: &str) -> (Vec<String>, Vec<i64>){
    let content = fs::read_to_string(filename).expect("Reading error");
    
    //Containers for the input data
    let mut direction = Vec::new();
    let mut steps = Vec::new();
    
    //Processing the inputs
    for line in content.trim().split("\n"){
        let mut parts = line.split(' ');
        let d = parts.next().unwrap().to_string();
        let s = parts.next().unwrap().parse().unwrap();

        direction.push(d);
        steps.push(s);
    }
    return (direction, steps);
}

fn part_1(direction: &Vec<String>, steps: &Vec<i64>) -> i64{
    let mut horizontal = 0;
    let mut vertical = 0;
    for i in 0..direction.len(){
        match direction[i].as_ref(){
            "forward" => horizontal += steps[i],
            "up" => vertical -= steps[i],
            "down" => vertical += steps[i],
            _ => println!("Unknown directive")
        }
    }

    return horizontal * vertical
}

fn part_2(direction: &Vec<String>, steps: &Vec<i64>) -> i64{
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;
    for i in 0..direction.len(){
        match direction[i].as_ref(){
            "forward" => {
                horizontal += steps[i]; 
                vertical += aim * steps[i]
            },
            "up" => aim -= steps[i],
            "down" => aim += steps[i],
            _ => println!("Unknown directive")
        }
    }

    return horizontal * vertical
}

fn main() {
    let (direction, steps) = read_to_array("input.txt");
    let part_a_sol = part_1(&direction, &steps);
    let part_b_sol = part_2(&direction, &steps);
    println!("Position = {}", part_a_sol);
    println!("Position with aim = {}", part_b_sol);
}
