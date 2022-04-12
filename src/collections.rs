/**
Veg<T> -> Dynamic (growable) array -> One Sided Dynamic Array
VecDeque<T> Double ended queue -> Ring Queue Array
LinkedList<T> Double linked list
BinaryHeap<T> where T:Ord -> Max heap priority_queue
HashMap<K, V> where K: Eq + Hash -> Dictionary (key-value map)
BTreeMap<K, V> where K: Ord -> Sorted dictionary (key-value table)
HashSet<T> where T: Eq + Hash -> Hash Table -> unordered set
BTreeSet<T> where T: Ord -> Sorted set
**/

pub fn vectors() {
    let mut a = Vec::new();
    println!("Vector a = {:?}", a);
    // adding element to th vector
    a.push(1);
    a.push(2);
    a.push(3);
    println!("Vector a = {:?}", a);
    // adding element to th vector
    a.push(44);
    println!("Vector a = {:?}", a);

    // Using index variable to get value
    let idx: usize = 0;
    println!("Vector a[{idx}] = {}", a[idx]);

    // Getting value from the vector with option type
    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("a[6] -> No such element!")
    }

    // removing element from the vector
    let last_elem = a.pop(); // Will remove
    match last_elem {
        Some(x) => println!("{} value removed from Vector a", x),
        None => println!("There is no element in Vector a!")
    }

    let mut i = 0;
    while let Some(x) = a.pop() {
        println!("Vector a[{i}] = {x}");
        i+=1;
    }
}