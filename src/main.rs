fn main() {
    println!("Hello, world!");

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a_iter = a.iter();

    let b: Vec<i32> = a_iter.map(|x: &i32| x * 2).collect();

    println!("my array: {:?}", b)
}
