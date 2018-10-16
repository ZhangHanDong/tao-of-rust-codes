use derive_new::New;
// A struct with no fields.
#[derive(New, PartialEq, Debug)]
pub struct Foo {}
// A struct with fields.
#[derive(New, PartialEq, Debug)]
pub struct Bar {
    pub x: i32,
    pub y: String,
}
// A unit struct.
#[derive(New, PartialEq, Debug)]
pub struct Baz;
// A tuple struct
#[derive(New, PartialEq, Debug)]
pub struct Tuple(pub i32, pub i32);
#[test]
fn test_empty_struct() {
    let x = Foo::new();
    assert_eq!(x, Foo {});
}
#[test]
fn test_simple_struct() {
    let x = Bar::new(42, "Hello".to_owned());
    assert_eq!(x, Bar { x: 42, y: "Hello".to_owned() });
}
#[test]
fn test_unit_struct() {
    let x = Baz::new();
    assert_eq!(x, Baz);
}
#[test]
fn test_simple_tuple_struct() {
    let x = Tuple::new(5, 6);
    assert_eq!(x, Tuple(5, 6));
}


#[derive(New, PartialEq, Debug)]
pub struct Intersection<'scene> {
    pub object: &'scene Bar,
    pub normal: Foo,
    pub point: Foo,
    pub t: f64,
}

#[test]
fn test_struct_with_lifetime() {
    let b = Bar::new(42, "Hello".to_owned());
    let x = Intersection::new(&b, Foo::new(), Foo::new(), 42.0);
    assert_eq!(x, Intersection { object: &b, normal: Foo {}, point: Foo {}, t: 42.0 });
}

//
// #[derive(new, PartialEq, Debug)]
// pub struct Fred {
//     #[new(value = "1 + 2")]
//     pub x: i32,
//     pub y: String,
//     #[new(value = "vec![-42, 42]")]
//     pub z: Vec<i8>,
// }
// //
// #[test]
// fn test_struct_with_values() {
//     let x = Fred::new("Fred".to_owned());
//     assert_eq!(x, Fred { x: 3, y: "Fred".to_owned(), z: vec![-42, 42] });
// }
