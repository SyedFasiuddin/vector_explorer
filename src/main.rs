fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let len: i32 = v.len() as i32; // return usize
    println!("{len}"); // 3

    let last_elem: Option<i32> = v.pop();
    match last_elem {
        Some(x) => println!("Last element is: {x}"),
        None => println!("cannot pop")
    }

    let len: usize = v.len();
    println!("{len}"); // 2

    v.push(3);
    v.push(4);
    for i in &v { // i: &i32
        println!("{i}");
    }

    let last_elem: Option<&i32> = v.last();
    match last_elem {
        Some(x) => println!("Last element is: {x}"),
        None => println!("Empty vector")
    }

    let it: std::slice::Iter<i32> = v.iter();
    println!("{:?}", it);
}

#[cfg(test)]
mod tests {

    #[test]
    fn new_vec_with_macro() {
        let v = vec![1, 2, 3];
        assert_eq!([1, 2, 3].to_vec(), v);
    }

    #[test]
    fn new_vec() {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        assert_eq!(vec![1, 2, 3], v);
        assert_eq!([1, 2, 3].to_vec(), v);
    }

}
