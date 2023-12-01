use rand::random;

fn main() {
    let mut numbers: [u8; 10] = random();
    println!("{:?}", cocktail_sort(&mut numbers));
}

fn cocktail_sort(numbers: &mut [u8]) -> &[u8] {
    let mut is_swapped: bool = true;
    let mut start = 0;
    let mut end = numbers.len() - 1;

    while is_swapped {
        is_swapped = false;
        for i in start..end {
            if numbers[i] > numbers[i + 1] {
                (numbers[i], numbers[i + 1]) = (numbers[i + 1], numbers[i]);
                is_swapped = true;
            }
        }

        if !is_swapped { break; };

        is_swapped = false;
        end = end - 1;

        for i in (start..end).rev() {
            if numbers[i] > numbers[i + 1] {
                (numbers[i], numbers[i + 1]) = (numbers[i + 1], numbers[i]);
                is_swapped = true;
            }
        }

        if !is_swapped { break; };
        start = start + 1;
    }
    return numbers;
}