fn main() {
    let v: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    println!("result is {:?}", my_sum(&v[..]));
    let v: Vec<u32> = vec![1, u32::MAX];
    println!("result is {:?}", my_sum(&v[..]));
}

fn my_sum(list: &[u32]) -> Option<u32> {
    let mut sum = 0_u64;
    for &item in list {
        sum = sum + item as u64;
        if sum > u32::MAX as u64 {
            return None;
        }
    }
    Some(sum as u32)
}
