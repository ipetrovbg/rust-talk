fn main() {
    let data = vec![1, 2, 3];
    let doubled: Vec<i32> = data.iter().map(|x| x * 2).collect();
    let sum: i32 = doubled.iter().sum();
    println!("sum: {}", sum);
}
