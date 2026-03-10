fn main() {
    // Common collection types in Rust
    // Vec<T>: Growable array
    // VecDeque<T>: Double Ended Queue
    // LinkedList<T>: Doubly Linked List
    // BinaryHeap<T>: Max Heap
    // HashMap<K, V>: key-value hash table
    // BTreeMap<K, V>: sorted key-value table
    // HashSet<T>: unoredered hash based set
    // BTreeSet<T>: sorted set

    // empty vector
    #[allow(unused_mut, unused_variables)]
    let mut numbers: Vec<i32> = vec![];

    // vector with some content
    #[allow(clippy::useless_vec)]
    let words = vec!["Niche", "Opulence", "Insouciant"];
    let sub_word = words[0..2].to_vec();

    println!("Sub word: {sub_word:?}");

    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();
        println!("Bytes Form: {bytes:?}");

        for (i, &item) in bytes.iter().enumerate() {
            println!("Bytes [{i}]: {item}");
        }

        s.len()
    }

    let first_name_end_index = first_word(&"Brian David Obot".to_string());
    println!(
        "First Name = {}",
        &"Brian David Obot"[0..first_name_end_index]
    );

    // methods available on slices
    #[allow(clippy::useless_vec)]
    let names = vec![1, 2, 3, 4, 5, 6];
    let _first = &names[..].first(); // returns a reference to the first item of a slice as an Option
    let _last = &names[..].last(); // returns a reference to the last item of a slice as an Option
    let _item = &names[..].get(10); // get the item at the index, if the item exist or None

    // each of the methods above have a mut variant that returns a mutable reference to the items
    // .to_vec(): Makes copies of the slice creating a new vector from the item in it, this is only available if the elements are cloneable
    //
    // .len(): returns the length of the slice
    // .is_empty(): returns True if the slice contains no element
    //
    // Vector specific methods
    // .capacity(): returns the capacity of a vector
    // .reserve(n): reserves enough space for n more items in the vectors capacity
    // .reserve_exact(n): reserves exactly enough space for n item and no more
    // .shrink_to_fit(): tries to free up extra space is capaccity > length
    // .push(value): Add an element to the vector
    // .pop(): removes nd return the last element from a vector as an Option
    // .insert(index, value)
    // .remove(index)
    // .resize(new_size, new_value): increase the length of a vector to new_size and fills extra spaces with new_value as needed
    // .resize_with(new_len, closure): same as above, but uses a closure to generate the new_values
    // .truncate(new_len): cut a vector to a length dropping all elements outide this length
    // .clear(): same as truncate(0)
    // .extend(iterable): takes a value that implements IntoIterator and extends the vector with the values from it
    // .split_off(index): like truncate, but the dropped values are returns as a vector to the caller similar to pop but for many values
    // .append(vec2): Moves all elements from vec2 into the vector leving vec2 empty afterwards
    // .drain(range)
    // .retain(test): drops all elements that don't pass the test provided in the function, similar to calling filter and then collect on a iterator
    // .dedup(): drops duplicate adjacent values so that only one is left
    // .reverse(): reverse a slice in place
    // Once a slice is sorted it can be efficiently searched
    // .binary_search(&value)
    // .binary_search_by(&value, cmp)
    //
    //
    // These methods work on arrays of arrays
    // .concat(): return a new vector by concatenating all the slices
    // .join(&seperator): same as above, except that the seperator is inserted between slices
    //
    // to get the index of an item in a slice use the position method on the iterator
    // slice.iter().postion(|x| *x == value)

    // Random elements
    // .choose(&mut rng)
    // .shuffle(&mut rng)
    //
    // VecDeque: Allow for efficiently adding and removing elements from the front and end of the vector
    // .push_front(value)
    // .push_back(value)
    // .pop_front()
    // .pop_back()
    // .front(), .back() : similar to first and last in normal Vectors
    // .front_mut(), back_mut(): similar to first_mut and last_mut for normal vectors
    //
    // you can convert to and from vecdeque to normal vectors using the following methods
    // Vec::from(deque)
    // VecDeque::from(vec)

    // BinaryHeap
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::new();
    heap.push(2);
    heap.push(32);
    heap.push(23);

    let greatest = heap.pop().unwrap();

    println!("Heap: {heap:?} former greatest: {greatest}");
    // we can create binary heap from another collections as so
    let heap = BinaryHeap::from(vec![1, 2, 3, 4, 5, 6, 7]);
    println!("Heap: {heap:?}");
    // binary heap can hold any type that implements the Ord trait
    // this makes BinaryHeap useful for a task queue, you can define a task struct that implements the Ord trait
    // and then pass those into a binaryheap which would also put the one with the higest priority at the top
    //
    // Binary heap is itereable and has an iter method, but the iterator produce the elements in an arbitary order
}
