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
    fn new_empty_vec_with_0_capacity() {
        let v1: Vec<i32> = Vec::new();
        let v2: Vec<i32> = vec![];

        assert_eq!(v1.len(), 0);
        assert_eq!(v2.len(), 0);

        assert!(v1.is_empty());
        assert!(v2.is_empty());

        assert!(v1.capacity() == 0);
        assert!(v2.capacity() == 0);
    }

    #[test]
    fn new_empty_vec_with_some_capacity() {
        let v: Vec<i32> = Vec::with_capacity(10);
        // will panic if capacity exceeds isize::MAX

        assert_eq!(v.len(), 0);
        assert!(v.is_empty());
        assert!(v.capacity() >= 10);
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

    #[test]
    fn concatenate_two_vec() {
        let mut v1 = vec![1, 2, 3];
        let mut v2 = vec![4, 5, 6];
        v1.append(&mut v2);

        assert_eq!(v1, vec![1, 2, 3, 4, 5, 6]); // everything from other vector is moved in
        assert_eq!(v2, vec![]); // everything is moved out
    }

    #[test]
    fn get_len_of_vec() {
        let v = vec![1, 2, 3];
        assert_eq!(v.len(), 3);
    }

    #[test]
    fn check_if_vec_is_empty() {
        let mut v: Vec<i32> = Vec::new();
        // since not inseting any elements we get error E0282
        // refer test `insert_into_vector`
        assert_eq!(v.is_empty(), true);

        v.push(1);
        assert_eq!(v.is_empty(), false);

        // in C++ we do something like
        // v.size() > 0
        // we can do it here as well but clippy will correct us
    }

    #[test]
    fn get_capacity_of_vec() {
        let mut v = vec![1, 2, 3];
        assert_eq!(v.capacity(), 3);

        v.pop();
        v.pop();
        v.pop();
        assert_eq!(v.capacity(), 3);
        assert_eq!(v.is_empty(), true);
    }

    #[test]
    fn reserve_capacity() {
        let mut v: Vec<i32> = Vec::new();
        v.reserve(10);
        assert!(v.capacity() >= 10);

        // `reserve_exact` is a varient which reserves exact (cannot guarentee exact size due to
        // underlying OS, page size, allocator rounding things off etc.)
        // `reserve` allocates a bit more just so that it can avoide allocating the next time.
    }

    #[test]
    fn safe_reserve_capacity() {
        let mut v: Vec<i32> = Vec::new();
        let _ = v.try_reserve(10);
        assert!(v.capacity() >= 10);

        // `reserve` and `reserve_exact` panics if they were unable to allocate enough capacity
        // `try_reserve` returns a result and avoids panicing
        // `try_reserve_exact` is another variant
    }

    #[test]
    fn reduce_capacity_of_vec() {
        let mut v = vec![1, 2, 3];
        v.reserve(100);
        assert!(v.capacity() >= 100);

        v.shrink_to_fit();
        assert!(v.capacity() <= 100);
        assert!(v.capacity() >= 3);
    }

    #[test]
    fn reduce_capacity_of_vec_min_to() {
        let mut v = vec![1, 2, 3];
        v.reserve(100);
        assert!(v.capacity() >= 100);

        v.shrink_to(5);
        assert!(v.capacity() <= 100);
        assert!(v.capacity() >= 5);
    }

    #[test]
    fn empty_the_vec() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.clear();

        assert_eq!(Vec::<i32>::new(), v);
        assert!(v.is_empty());
        assert!(v.capacity() >= 5);
    }

    #[test]
    fn reduce_vec_len_by_dropping_elems() {
        let mut v = vec![1, 2, 3, 4, 5];

        v.truncate(3);
        assert_eq!(vec![1, 2, 3], v);

        // if number more than `len` is given then it has no effect
        // if `0` is given then it is same as clear
    }

    #[test]
    fn change_size_of_vec() {
        let mut v = vec![1, 2, 3, 4, 5];
        // T must implement `Clone`

        v.resize(10, 10); // len, value
        assert_eq!(vec![1, 2, 3, 4, 5, 10, 10, 10, 10, 10], v);

        // if len < v.len() then it acts as truncate and value has no meaning
        v.resize(5, 9);
        assert_eq!(vec![1, 2, 3, 4, 5], v);
    }

    #[test]
    fn change_size_with_closure() {
        let mut v = vec![1, 2, 3, 4, 5];

        v.resize_with(10, Default::default);
        assert_eq!(vec![1, 2, 3, 4, 5, 0, 0, 0, 0, 0], v);

        v.truncate(5);
        v.resize_with(10, || { 0 });
        assert_eq!(vec![1, 2, 3, 4, 5, 0, 0, 0, 0, 0], v);

        let mut i = 1;
        v.clear();
        v.resize_with(10, || { i *= 2; i });
        assert_eq!(vec![2, 4, 8, 16, 32, 64, 128, 256, 512, 1024], v);
    }

    #[test]
    fn slice_from_vec() {
        let v = vec![1, 2, 3, 4, 5];

        assert_eq!(v.as_slice(), &v[..]);
        assert_eq!(v.as_slice(), vec![1, 2, 3, 4, 5]);
        // can't understand how it is useful
    }

    #[test]
    fn cut_vec_into_two_parts() {
        let mut v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.split_off(2), [3, 4, 5]);
        assert_eq!(v, [1, 2]);

        // any within bound index can be provided
        // returned [idx, len)
        // original [0, idx)
        // will panic if idx > len
        // Vec implements Deref which has a lot more of split_**
    }

}
