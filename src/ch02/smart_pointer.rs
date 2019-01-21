/// # 智能指针：Box<T>
///
/// Basic usage:
///
/// ```
/// fn box_demo(){
///     #[derive(PartialEq, Debug)]
///     struct Point {
///       x: f64,
///       y: f64,
///     }
///     // 将Point实例装箱（放到堆内存）
///     let box_point = Box::new(Point { x: 0.0, y: 0.0 });
///     let unboxed_point: Point = *box_point; // 通过解引用操作符取出Point实例
///     assert_eq!(unboxed_point, Point { x: 0.0, y: 0.0 });
/// }
/// box_demo();
/// ```
pub fn box_demo(){
    #[derive(Debug, PartialEq)]
    struct Point {
      x: f64,
      y: f64,
    }
    let box_point = Box::new(Point { x: 0.0, y: 0.0 });
    let unboxed_point: Point = *box_point;
    assert_eq!(unboxed_point, Point { x: 0.0, y: 0.0 });
}
