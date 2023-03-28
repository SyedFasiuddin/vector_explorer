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

    #[test]
    fn insert_into_vector() {
        let mut v = Vec::new();

        // untill something is being pushed into the vector the compiler doesn't know what
        // type of elements will be stores inside vector `v` so it will complain saying to add
        // a type errno E0282
        v.push(1);

 // 1  error[E0282]: type annotations needed for `Vec<T>`
 //   --> src/main.rs:52:13
 //    |
 // 52 |         let mut v = Vec::new();
 //    |             ^^^^^
 //    |
 // help: consider giving `v` an explicit type, where the type for type parameter `T` is specified
 //    |
 // 52 |         let mut v: Vec<T> = Vec::new();
 //    |                  ++++++++
 //
 // For more information about this error, try `rustc --explain E0282`.
 // error: could not compile `vector_explorer` due to previous error

        v.push(10);
        assert_eq!(vec![1, 10], v);
    }

    #[test]
    fn remove_last_element_from_vec() {
        let mut v = vec![10, 20, 30];

        assert_eq!(v.pop(), Some(30));
        assert_eq!(vec![10, 20], v);

        v.pop();
        v.pop();
        assert_eq!(v, Vec::new());
    }

    #[test]
    fn remove_element_from_any_position() {
        let mut v = vec![1, 2, 3, 4, 5];

        // this is o(n) operation, as everything on right will be shifted
        assert_eq!(v.remove(2), 3); // remove will panic if index (argument) is out of bound
        assert_eq!(v.len(), 4);
        assert_eq!(vec![1, 2, 4, 5], v);
    }

    #[test]
    fn insert_element_at_any_position() {
        let mut v = vec![1, 2, 3, 4, 5];

        // will panic if idx > len
        v.insert(1, 10); // idx, val
        assert_eq!(vec![1, 10, 2, 3, 4, 5], v);

        // I assume that this operation also takes o(n) time, as everything is shifted to make
        // place for the element, if under the hood linked lists are used then may be not.
        // Howevery time complexity is not mentioned in the docs
    }

}
