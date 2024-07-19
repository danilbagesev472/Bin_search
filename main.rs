use std::cmp::Ordering;

fn main() {
    let arr: [i32; 5] = [13, 3, 5, 6, 125];
    let res = bin_search(&arr, 5);
    match res {
        Some((val, inx)) => println!("value - {val}\t index - {inx}"),
        None => println!("Not found"),
    }
}
fn bin_search(arr: &[i32], value: i32) -> Option<(i32, usize)> {
    let mut low: usize = 0;
    let mut up: usize = arr.len() - 1;
    let mut i: usize = 0;

    while low <= up {
        i += 1;

        let mid: usize = (up + low) / 2;
        let mid_val = arr[mid];

        match mid_val.cmp(&value) {
            Ordering::Equal => return Some((mid_val, mid)),
            Ordering::Greater => low = mid + 1,
            Ordering::Less => up = mid - 1,
        }

        println!("step {}", i)
    }
    None
}
