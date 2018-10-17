#![feature(plugin)]
#![plugin(plugin_demo)]

#[test]
fn test_plugin() {
    assert_eq!(roman_to_digit!(MMXVIII), 2018);
    // // errors
    // assert_eq!(roman_to_digit!(M,X), 2018);
    // assert_eq!(roman_to_digit!(), 2018);
    // assert_eq!(roman_to_digit!(123), 2018);
}