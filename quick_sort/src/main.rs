use rand::random;

fn main() {
    let mut numbers: [u8; 10] = random();
    println!("{:?}", quick_sort(&mut numbers));
}

fn partition(numbers: &mut [u8], low: u8, high: u8) -> u8 {
    let mut i: i8 = low as i8 - 1;
    let pivot = numbers[high as usize];
    
    for j in low..high {
        if numbers[j as usize] <= pivot {
            i += 1;
            (numbers[i as usize], numbers[j as usize]) = (numbers[j as usize], numbers[i as usize]);
        }
    }
    (numbers[(i + 1) as usize], numbers[high as usize]) = (numbers[high as usize], numbers[(i + 1) as usize]);
    (i + 1).try_into().unwrap()
}

fn quick_sort(numbers: &mut [u8]) -> &[u8] {
    fn _quick_sort(numbers: &mut [u8], low: u8, high: u8) {
        if low < high {
            let partition_index = partition(numbers, low, high);
            _quick_sort(numbers, low, partition_index - 1);
            _quick_sort(numbers, partition_index  + 1, high);
        }
    }

    _quick_sort(numbers, 0, (numbers.len() - 1).try_into().unwrap());
    numbers
}