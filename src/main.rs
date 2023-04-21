use rand::Rng;
use std::time::Instant;
use std::thread;

const NUM_OF_BOOLS: usize = 1000;
const NOB_DIVIDED: usize = NUM_OF_BOOLS / 4;

fn create_2d_vector() -> Vec<Vec<Vec<bool>>> {
    let mut array:Vec<Vec<Vec<bool>>> = Vec::with_capacity(NUM_OF_BOOLS);
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_OF_BOOLS {
        array.push(Vec::with_capacity(NUM_OF_BOOLS));
    }
    for x in &mut array {
        for _ in 0..NUM_OF_BOOLS {
            x.push(Vec::with_capacity(NUM_OF_BOOLS))
        }
        for y in x {
            for _ in 0..NUM_OF_BOOLS {
                y.push(rng.gen_bool(0.5));
            }
        }
    }
    array
}

enum SortedOptions {
    Trues,
    Falses,
}

struct Sorted {
    trues: Vec<bool>,
    falses: Vec<bool>,
}

impl Sorted {
    fn add_to_vec(&self, sorted_option: SortedOptions, vector: &mut Vec<bool>) {
        match sorted_option {
            SortedOptions::Falses => {
                for x in &self.falses {
                    vector.push(*x);
                }
            }
            SortedOptions::Trues => {
                for x in &self.trues {
                    vector.push(*x);
                }
            }
        }
    }
}

fn parse_3d_vector(vector: &mut [Vec<Vec<bool>>]) -> Sorted {
    let mut trues: Vec<bool> = vec![];
    let mut falses: Vec<bool> = vec![];

    for x in vector {
        for y in x {
            for z in y {
                // Check which vector to enter the values into 
                if z == &true {
                    trues.push(z.clone());
                } else {
                    falses.push(z.clone());
                }
            }
        }
    }

    Sorted {
        trues,
        falses,
    }
}

fn main() {
    let mut vector = create_2d_vector();

    let (first, rest) = vector.split_at_mut(NOB_DIVIDED);
    let (second, rest_2) = rest.split_at_mut(NOB_DIVIDED);
    let (third, fourth) = rest_2.split_at_mut(NOB_DIVIDED);

    // Create an Instant to track time
    let before = Instant::now();

    let mut sorted_1: Sorted = Sorted{trues: vec![], falses: vec![]};
    let mut sorted_2: Sorted = Sorted{trues: vec![], falses: vec![]};
    let mut sorted_3: Sorted = Sorted{trues: vec![], falses: vec![]};
    let mut sorted_4: Sorted = Sorted{trues: vec![], falses: vec![]};

    thread::scope(|s| {
        s.spawn( || {
            sorted_1 = parse_3d_vector(first);
        });
        s.spawn( || {
            sorted_2 = parse_3d_vector(second);
        });
        s.spawn( || {
            sorted_3 = parse_3d_vector(third);
        });
        s.spawn( || {
            sorted_4 = parse_3d_vector(fourth);
        });
    });

    println!("Time Elapsed: {:?}", before.elapsed());

    let mut falses: Vec<bool> = vec![];
    let mut trues: Vec<bool> = vec![];

    sorted_1.add_to_vec(SortedOptions::Falses, &mut falses);
    sorted_1.add_to_vec(SortedOptions::Trues, &mut trues);
    sorted_2.add_to_vec(SortedOptions::Falses, &mut falses);
    sorted_2.add_to_vec(SortedOptions::Trues, &mut trues);
    sorted_3.add_to_vec(SortedOptions::Falses, &mut falses);
    sorted_3.add_to_vec(SortedOptions::Trues, &mut trues);
    sorted_4.add_to_vec(SortedOptions::Falses, &mut falses);
    sorted_4.add_to_vec(SortedOptions::Trues, &mut trues);
}