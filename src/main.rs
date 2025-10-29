fn main() {
    let input = [2, 4, 6];
    let mut total = 0;
    for i in &input {
        total += i;
    }
    dbg!(total);

    dbg!(sum(&input));
}

fn sum(arr: &[u32]) -> u32 {
    if arr.iter().count() == 1 {
        return arr[0];
    }
    return arr[0] + sum(&arr[1..]);
}
