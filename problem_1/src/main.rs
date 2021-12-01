use std::fs;

pub fn read_to_array(filename: &str) -> Vec<i64>{
    let content = fs::read_to_string(filename).expect("Reading error");
    let measurements: Vec<i64> = content.trim().split("\n").map(|l| l.parse().unwrap()).collect();
    return measurements;
}


fn main() {
    let measures = read_to_array("input.txt");
    
    let mut increases = 0;
    for i in 1..measures.len(){
        if measures[i] > measures[i-1] {
            increases += 1;
        }
    }
   
    println!("{} increases", increases)
}
