use std::collections::HashMap;

fn always_increase(feed: u32) -> Result<Vec<u32>, Vec<u32>> {
    let mut password = feed;
    let mut successful = false;
    let mut digits = Vec::new();
    let mut splitter = 100000;

    while splitter >= 10 {
        digits.push(password / splitter);
        password %= splitter;
        splitter /= 10;
    }

    digits.push(password);

    for i in 1..6 {
        successful = digits[i - 1] <= digits[i];
        if successful == false {
            break;
        }
    }

    if successful {
        return Ok(digits);
    }
    return Err(digits);
}

fn adjacent_twin(digits: Vec<u32>) -> bool {
    let mut results = HashMap::new();
    let mut x = 0;

    loop {
        let mut occurences = 1;

        if x >= digits.len() {
            break;
        }

        let current = digits[x];

        x += 1;

        if x >= digits.len() {
            break;
        }
        
        if current != digits[x] {
            continue;
        }

        occurences += 1;

        loop {
            x += 1;
            if x >= digits.len() {
                break;
            }

            if current == digits[x] {
                occurences += 1;
            }

            if current != digits[x] {
                x -= 1;
                break;
            }
        }

        results.insert(current, occurences);
    }

    for (_k, v) in &results {
        if *v == 2 {
            return true;
        }
    }
    return false;
}

fn main() {
    let mut possibilities = Vec::new();
    let mut current = 147981;
    let max = 691423;

    while current < max {
        match always_increase(current) {

            Err(_) => {},
            Ok(digits) => {
                if adjacent_twin(digits) {
                    possibilities.push(current);
                }
            }
        }

        current += 1;
    }

    println!("Possible passwords: {:?}", possibilities);
    println!("Number of possible passwords: {:?}", possibilities.len());
}
