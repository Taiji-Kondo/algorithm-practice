use rand::random;

fn main() {
    let mut numbers: [u8; 10] = random();
    println!("{:?}", bubble_sort(&mut numbers));
}

fn bubble_sort(numbers: &mut [u8]) -> &[u8] {
    let numbers_len = numbers.len();
    for i in 0..numbers_len {
        for j in 0..numbers_len - i - 1  {
            if numbers[j] > numbers[j + 1] {
                (numbers[j], numbers[j + 1]) = (numbers[j + 1], numbers[j]);
            }
        }
    }
    return numbers;
}