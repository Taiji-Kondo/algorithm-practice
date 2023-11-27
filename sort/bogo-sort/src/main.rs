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

fn main() {
    let mut numbers: [i32; 5] = [3, 1, 5, 2, 4];
    let sorted = bogo_sort(&mut numbers);
    println!("{:?}", sorted);
}
