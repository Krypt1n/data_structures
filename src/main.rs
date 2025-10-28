fn binary_search(arr: &[u32], item: u32) -> Option<u32> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let guess: u32 = arr[mid];

        if guess == item {
            return Some(mid as u32);
        } else if guess > item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    None
}

fn main() {
    let item: u32 = 12;
    dbg!(binary_search(&[1, 3, 5, 7, 9], item));
}
