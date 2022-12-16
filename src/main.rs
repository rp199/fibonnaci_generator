use std::io;

fn main() {
    println!("Fibonnaci Sequence Generator");
    loop {
        let how_many_numbers = read_how_many();
        let sequence = generate_fibonnaci_sequence(how_many_numbers);
        println!("Sequence: {:?}", sequence)
    }

}

fn generate_fibonnaci_sequence(how_many: usize) -> Vec<usize>{
    let mut sequence = Vec::with_capacity(how_many);
    match how_many {
        0 => {},
        1 => sequence.push(0),
        _ => {
            sequence.push(0);
            sequence.push(1);
            for idx in 2..how_many {
                let number = sequence[idx - 1] + sequence[idx - 2];
                sequence.push(number)
            }
        }
    };
    sequence
}

fn read_how_many() -> usize {

    loop {
        let mut idx = String::new();
        println!("How many fibonnaci numbers?");
        io::stdin()
            .read_line(&mut idx)
            .expect("Failed to read line.");

        let idx: usize = match idx.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => {
                println!("Invalid number");
                continue
            }
        };
        return idx;
    }

}