use std::collections::{HashMap, HashSet};

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

fn print_shape_size(key: String, shapes: &HashMap<String, i32>) {
    match shapes.get(&key) {
        Some(x) =>  println!("A {key} has {} sides",x),
        None => println!("{key} shape hasn't been found in shapes storage!")
    }
}

pub fn hashmap() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    print_shape_size(String::from("square"), &shapes);
    print_shape_size(String::from("triangle"), &shapes);
    print_shape_size(String::from("rectangle"), &shapes);


    for (key, value) in &shapes {
        println!("{} has {} sides!", key, value);
    }

    // override values
    shapes.insert("square".into(), 5);
    print_shape_size(String::from("square"), &shapes);

    // Insert value if they key doesn't exist.
    shapes.entry("circle".into()).or_insert(1);
    println!("Shapes = {:?}", shapes);
    {
        // get reference for the circle entry
        let actual = shapes.entry("circle".into()).or_default();
        *actual = 0;
    }

    println!("Shapes = {:?}", shapes);
}

pub fn hashset() {
    let mut greek = HashSet::new();

    greek.insert("gamma");
    greek.insert("delta");
    println!("HashSet greek = {:?}", greek);

    greek.insert("delta");
    println!("HashSet greek = {:?}", greek);

    // adding element
    let has_vega_added = greek.insert("vega");
    if has_vega_added {
        println!("vega keyword added to greek HashSet");
    }

    // checking if set contains the element
    if !greek.contains("kappa") {
        println!("HashSet \"greek\" doesn't contain kappa!");
    }

    // removing element
    let has_delta_removed = greek.remove("delta");
    if has_delta_removed {
        println!("\"delta\" removed from HashSet greek");
    }

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // subset
    println!("is {:?} a subset of {:?}? => {}", _2_8, _1_10, _2_8.is_subset(&_1_10));

    // disjoint -> they have no common elements
    println!("is {:?} disjoint of {:?}? => {}", _2_8, _1_10, _2_8.is_disjoint(&_1_10));
    println!("is {:?} disjoint of {:?}? => {}", _1_5, _6_10, _1_5.is_disjoint(&_6_10));

    // union & intersection
    println!("items in {:?} or {:?} are {:?}?", _1_5, _6_10, _1_5.union(&_6_10));
    println!("items in either {:?} and {:?} are {:?}?", _1_5, _6_10, _1_5.intersection(&_6_10));
}