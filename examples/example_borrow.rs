

fn print_vec(vec: &Vec<u8>){
    for i in vec {
        println!("{}", i);
    }
}

fn main() {
    let nums: Vec<u8> = Vec::new(); 
    print_vec(&nums);

    let size = nums.len();
    println!("{}", size);
}