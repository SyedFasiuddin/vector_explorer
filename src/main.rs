fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let len: i32 = v.len() as i32; // return usize
    println!("{len}"); // 3

    let last_elem: Option<i32> = v.pop();
    match last_elem {
        Some(x) => println!("Last element is: {x}"),
        None => println!("cannot pop")
    }
}
