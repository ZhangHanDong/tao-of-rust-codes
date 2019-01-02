/// # HashMap
///
/// Base usage:  增删改查
///
/// ```rust
/// use std::collections::HashMap;
/// fn main() {
///     let mut book_reviews = HashMap::with_capacity(10);
///     book_reviews.insert("Rust Book", "good");
///     book_reviews.insert("Programming Rust", "nice");
///     book_reviews.insert("The Tao of Rust", "deep");
///     for key in book_reviews.keys() {
///         println!("{}", key);
///     }
///    for val in book_reviews.values() {
///        println!("{}", val);
///    }
///    if !book_reviews.contains_key("rust book") {
///         println!("find {} times ", book_reviews.len());
///     }
///    book_reviews.remove("Rust Book");
///     let to_find = ["Rust Book", "The Tao of Rust"];
///     for book in &to_find {
///        match book_reviews.get(book) {
///        Some(review) => println!("{}: {}", book, review),
///            None => println!("{} is unreviewed.", book),
///        }
///    }
///    for (book, review) in &book_reviews {
///        println!("{}: \"{}\"", book, review);
///    }
///    assert_eq!(book_reviews["The Tao of Rust"], "deep");
/// }
/// ```
///
/// Base usage:  Entry模式
///
/// ```rust
/// use std::collections::HashMap;
/// fn main() {
///     let mut map: HashMap<&str, u32> = HashMap::new();
///     map.entry("current_year").or_insert(2017);
///     assert_eq!(map["current_year"], 2017);
///     *map.entry("current_year").or_insert(2017) += 10;
///     assert_eq!(map["current_year"], 2027);
///     let last_leap_year = 2016;
///     map.entry("next_leap_year")
///        .or_insert_with(|| last_leap_year + 4 );
///    assert_eq!(map["next_leap_year"], 2020);
///    assert_eq!(map.entry("current_year").key(), &"current_year");
/// }
/// ```
///
/// Base usage:  HashMap的三种合并方式
///
/// ```rust
/// use std::collections::HashMap;
/// fn merge_extend<'a>(
///     map1: &mut HashMap<&'a str, &'a str>,
///     map2: HashMap<&'a str, &'a str>
/// ) {
///     map1.extend(map2);
/// }
/// fn merge_chain<'a>(
///     map1: HashMap<&'a str, &'a str>,
///    map2:  HashMap<&'a str, &'a str>
/// ) -> HashMap<&'a str, &'a str> {
///    map1.into_iter().chain(map2).collect()
/// }
/// fn merge_by_ref<'a>(
///     map: &mut HashMap<&'a str, &'a str>,
///     map_ref: &HashMap<&'a str, &'a str>
/// ){
///    map.extend(map_ref.into_iter()
///                         .map(|(k, v)| (k.clone(), v.clone()))
///    );
/// }
/// fn main() {
///    let mut book_reviews1 = HashMap::new();
///    book_reviews1.insert("Rust Book", "good");
///    book_reviews1.insert("Programming Rust", "nice");
///    book_reviews1.insert("The Tao of Rust", "deep");
///    let mut book_reviews2 = HashMap::new();
///    book_reviews2.insert("Rust in Action", "good");
///    book_reviews2.insert("Rust Primer", "nice");
///    book_reviews2.insert("Matering Rust", "deep");
///    // merge_extend(&mut book_reviews1, book_reviews2);
///    // let book_reviews1 = merge_chain(book_reviews1, book_reviews2);
///    merge_by_ref(&mut book_reviews1, &book_reviews2);
///    for key in book_reviews1.keys() {
///       println!("{}", key);
///    }
/// }
/// ```
pub fn hashmaps(){
    unimplemented!();
}
