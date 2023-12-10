fn main() {
    let numbers: [u8; 10] = rand::random();
    let sorted = merge_sort(&numbers);
    println!("{:?}", sorted);
}

fn merge_sort(numbers: &[u8]) -> Vec<u8> {
    if numbers.len() <= 1 {
        return numbers.to_vec();
    }

    let center = numbers.len() / 2;
    let left = merge_sort(&numbers[..center]);
    let right = merge_sort(&numbers[center..]);

    merge(&left, &right)
}

fn merge(left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        result.extend_from_slice(&left[i..]);
    }
    if j < right.len() {
        result.extend_from_slice(&right[j..]);
    }

    result
}
