use rand::random;

fn main() {
    let mut numbers: [u8; 10] = random();
    println!("{:?}", selection_sort(&mut numbers));
}

fn selection_sort(numbers: &mut [u8; 10]) -> &[u8; 10] {
    let numbers_len: usize = numbers.len();

    for i in 0..numbers_len {
        let mut min_index: usize = i;
        for j in i + 1..numbers_len {
            if  numbers[min_index] > numbers[j] {
                min_index = j;
            }
        }
        (numbers[i], numbers[min_index]) = (numbers[min_index], numbers[i]);
    }
    numbers
}