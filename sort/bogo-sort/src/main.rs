use rand::random;
use rand::seq::SliceRandom;

fn in_order(numbers: &[i32]) -> bool {
    for i in 0..numbers.len() - 1 {
        if numbers[i] > numbers[i + 1] {
            return false;
        }
    }
    return true;
}

fn bogo_sort(numbers: &mut [i32]) -> &[i32] {
    while !in_order(numbers) {
        let mut rng = rand::thread_rng();
        numbers.shuffle(&mut rng);
    }
    return numbers;
}

fn create_random_array() -> [i32; 10] {
    let numbers: [i32; 10] = random();
    return numbers;
}

fn main() {
    let mut numbers: [i32; 10] = create_random_array();
    println!("original: {:?}", numbers);

    let sorted = bogo_sort(&mut numbers);
    println!("sorted: {:?}", sorted);
}
