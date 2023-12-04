use rand::random;

fn main() {
    let mut numbers: [u8; 10] = random();
    println!("{:?}", insertion_sort(&mut numbers));
}

fn insertion_sort(numbers: &mut [u8]) -> &[u8] {
    for i in 1..numbers.len() {
        let temp: u8 = numbers[i];
        let mut j: usize = i - 1;
        while j >= 0 && numbers[j] > temp {
            numbers[j + 1] = numbers[j];
            j -= 1;
        }
        numbers[j + 1] = temp;
    }
    numbers
}