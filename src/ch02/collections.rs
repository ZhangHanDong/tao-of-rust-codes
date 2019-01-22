/// # 线性序列：向量（Vec）
///
/// Basic usage:
///
/// ```
/// fn vec_example(){
///     let mut v1 = vec![];
///     v1.push(1);  // 使用push方法插入元素
///     v1.push(2);
///     v1.push(3);
///     assert_eq!(v1, [1,2,3]);
///     assert_eq!(v1[1], 2);
///     let mut v2 = vec!["hello".to_string(); 10];  // 像数组那样初始化
///     assert_eq!(v2.len(), 10);
///     assert_eq!(v2[5], "hello".to_string());
///     let mut v3 = Vec::new();
///     v3.push(4);
///     v3.push(5);
///     v3.push(6);
///     // v3[4];  error: index out of bounds
/// }
/// vec_example();
/// ```
pub fn vec_example() {
    let mut v1 = vec![];
    v1.push(1);
    v1.push(2);
    v1.push(3);
    assert_eq!(v1, [1, 2, 3]);
    assert_eq!(v1[1], 2);
    let mut v2 = vec![0; 10];
    let mut v3 = Vec::new();
    v3.push(4);
    v3.push(5);
    v3.push(6);
}

/// # 线性序列：双端队列（VecDeque）
///
/// Basic usage:
///
/// ```
/// use std::collections::VecDeque;
/// fn vec_deque() {
///     let mut buf = VecDeque::new();
///
///     buf.push_front(1);
///     buf.push_front(2);
///     assert_eq!(buf.get(0), Some(&2));
///     assert_eq!(buf.get(1), Some(&1));
///
///     buf.push_back(3);
///     buf.push_back(4);
///     buf.push_back(5);
///
///     assert_eq!(buf.get(2), Some(&3));
///     assert_eq!(buf.get(3), Some(&4));
///     assert_eq!(buf.get(4), Some(&5));
/// }
/// vec_deque();
/// ```
pub fn vec_deque() {
    use std::collections::VecDeque;
    let mut buf = VecDeque::new();

    buf.push_front(1);
    buf.push_front(2);
    assert_eq!(buf.get(0), Some(&2));
    assert_eq!(buf.get(1), Some(&1));

    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);

    assert_eq!(buf.get(2), Some(&3));
    assert_eq!(buf.get(3), Some(&4));
    assert_eq!(buf.get(4), Some(&5));
}

/// # 线性序列：链表（LinkedList）
///
/// Basic usage:
///
/// ```
/// use std::collections::LinkedList;
/// fn linked_list() {
///     let mut list1 = LinkedList::new();
///
///     list1.push_back('a');
///
///     let mut list2 = LinkedList::new();
///     list2.push_back('b');
///     list2.push_back('c');
///
///     list1.append(&mut list2);
///     println!("{:?}", list1); // ['a', 'b', 'c']
///     println!("{:?}", list2); // []
///
///     list1.pop_front();
///     println!("{:?}", list1); // ['b', 'c']
///
///     list1.push_front('e');
///     println!("{:?}", list1); // ['e', 'b', 'c']
///
///     list2.push_front('f');
///     println!("{:?}", list2); // ['f']
/// }
/// linked_list();
/// ```
pub fn linked_list() {
    use std::collections::LinkedList;
    let mut list1 = LinkedList::new();

    list1.push_back('a');

    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');

    list1.append(&mut list2);
    println!("{:?}", list1); // ['a', 'b', 'c']
    println!("{:?}", list2); // []

    list1.pop_front();
    println!("{:?}", list1); // ['b', 'c']

    list1.push_front('e');
    println!("{:?}", list1); // ['e', 'b', 'c']

    list2.push_front('f');
    println!("{:?}", list2); // ['f']
}

/// # Key-Value映射表: HashMap和BTreeMap
///
/// Basic usage:
///
/// ```
/// use std::collections::BTreeMap;
/// use std::collections::HashMap;
/// fn map_demo() {
///     let mut hmap = HashMap::new();
///     let mut bmap = BTreeMap::new();
///
///     hmap.insert(3, "c");
///     hmap.insert(1, "a");
///     hmap.insert(2, "b");
///     hmap.insert(5, "e");
///     hmap.insert(4, "d");
///
///     bmap.insert(3, "c");
///     bmap.insert(2, "b");
///     bmap.insert(1, "a");
///     bmap.insert(5, "e");
///     bmap.insert(4, "d");
///
///      // 输出结果为：{1: "a", 2: "b", 3: "c", 5: "e", 4: "d"}，但key的顺序是随机的，因为HashMap是无序的
///     println!("{:?}", hmap);
///     // 输出结果永远都是 {1: "a", 2: "b", 3: "c", 4: "d", 5: "e"}，因为BTreeMap是有序的
///     println!("{:?}", bmap);
/// }
/// map_demo();
/// ```
pub fn map_demo() {
    use std::collections::BTreeMap;
    use std::collections::HashMap;
    let mut hmap = HashMap::new();
    let mut bmap = BTreeMap::new();
    hmap.insert(3, "c");
    hmap.insert(1, "a");
    hmap.insert(2, "b");
    hmap.insert(5, "e");
    hmap.insert(4, "d");
    bmap.insert(3, "c");
    bmap.insert(2, "b");
    bmap.insert(1, "a");
    bmap.insert(5, "e");
    bmap.insert(4, "d");
    // 输出结果为：{1: "a", 2: "b", 3: "c", 5: "e", 4: "d"}，但key的顺序是随机的，因为HashMap是无序的
    println!("{:?}", hmap);
    // 输出结果永远都是 {1: "a", 2: "b", 3: "c", 4: "d", 5: "e"}，因为BTreeMap是有序的
    println!("{:?}", bmap);
}

/// # 集合: HashSet和BTreeSet
///
/// Basic usage:
///
/// ```
/// use std::collections::HashSet;
/// use std::collections::BTreeSet;
/// fn hashset_btreeset() {
///     let mut hbooks = HashSet::new();
///     let mut bbooks = BTreeSet::new();
///
///     // 插入数据
///     hbooks.insert("A Song of Ice and Fire");
///     hbooks.insert("The Emerald City");
///     hbooks.insert("The Odyssey");
///
///     // 判断元素是否存在，contains方法和HashMap中的一样
///     if !hbooks.contains("The Emerald City") {
///         println!("We have {} books, but The Emerald City ain't one.",
///                  hbooks.len());
///     }
///     // 顺序是随机 {"The Emerald City", "The Odyssey", "A Song of Ice and Fire"}
///     println!("{:?}", hbooks);
///
///     bbooks.insert("A Song of Ice and Fire");
///     bbooks.insert("The Emerald City");
///     bbooks.insert("The Odyssey");
///     // 顺序永远是  {"A Song of Ice and Fire", "The Emerald City", "The Odyssey"}
///     println!("{:?}", bbooks);
/// }
/// hashset_btreeset();
/// ```
pub fn hashset_btreeset() {
    use std::collections::BTreeSet;
    use std::collections::HashSet;
    let mut hbooks = HashSet::new();
    let mut bbooks = BTreeSet::new();
    // 插入数据
    hbooks.insert("A Song of Ice and Fire");
    hbooks.insert("The Emerald City");
    hbooks.insert("The Odyssey");
    // 判断元素是否存在，contains方法和HashMap中的一样
    if !hbooks.contains("The Emerald City") {
        println!(
            "We have {} books, but The Emerald City ain't one.",
            hbooks.len()
        );
    }
    println!("{:?}", hbooks);
    bbooks.insert("A Song of Ice and Fire");
    bbooks.insert("The Emerald City");
    bbooks.insert("The Odyssey");
    println!("{:?}", bbooks);
}

/// # 优先队列：BinaryHeap
///
/// Basic usage:
///
/// ```
/// use std::collections::BinaryHeap;
/// fn binary_heap() {
///     let mut heap = BinaryHeap::new();
///     assert_eq!(heap.peek(), None);
///     heap.push(93);
///     heap.push(80);
///     heap.push(48);
///     heap.push(53);
///     heap.push(72);
///     heap.push(30);
///     heap.push(18);
///     heap.push(36);
///     heap.push(15);
///     heap.push(35);
///     heap.push(45);
///     assert_eq!(heap.peek(), Some(&93));
///     println!("{:?}", heap);  // [93, 80, 48, 53, 72, 30, 18, 36, 15, 35, 45]
/// }
/// binary_heap();
/// ```
pub fn binary_heap() {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    assert_eq!(heap.peek(), None);
    heap.push(93);
    heap.push(80);
    heap.push(48);
    heap.push(53);
    heap.push(72);
    heap.push(30);
    heap.push(18);
    heap.push(36);
    heap.push(15);
    heap.push(35);
    heap.push(45);
    assert_eq!(heap.peek(), Some(&93));
    println!("{:?}", heap); // [93, 80, 48, 53, 72, 30, 18, 36, 15, 35, 45]
}
