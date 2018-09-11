/// # 设计模式
///
/// Base usage: 建造者模式
///
/// Rust常用设计模式之一，标准库中std::process::Command就是典型
///
/// ```rust
/// struct Circle {
///     x: f64,
///     y: f64,
///     radius: f64,
/// }
/// struct CircleBuilder {
///     x: f64,
///     y: f64,
///     radius: f64,
/// }
/// impl Circle {
///     fn area(&self) -> f64 {
///         std::f64::consts::PI * (self.radius * self.radius)
///     }
///     fn new() -> CircleBuilder {
///         CircleBuilder {
///             x: 0.0, y: 0.0, radius: 1.0,
///         }
///    }
/// }
/// impl CircleBuilder {
///    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
///        self.x = coordinate;
///        self
///    }
///    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
///        self.y = coordinate;
///        self
///    }
///    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
///        self.radius = radius;
///       self
///    }
///    fn build(&self) -> Circle {
///        Circle {
///            x: self.x, y: self.y, radius: self.radius,
///        }
///    }
/// }
/// fn main() {
///    let c = Circle::new()
///             .x(1.0).y(2.0).radius(2.0)
///             . build();
///    assert_eq!(c.area(), 12.566370614359172); // 可能不同机器的值有所差异
///    assert_eq!(c.x, 1.0);
///    assert_eq!(c.y, 2.0);
/// }
/// ```
///
/// Base usage: 访问者模式
///
/// Rust源码中还包含两个案例：
/// 1. rustc编译器源码中AST处理
/// 2. Serde的架构是访问者模式
///
/// ```rust
/// use std::any::Any;
/// trait HouseElement {
///     fn accept(&self, visitor: &HouseElementVisitor);
///     fn as_any(&self) -> &Any;
/// }
/// trait HouseElementVisitor {
///     fn visit(&self, element: &HouseElement);
/// }
/// struct House {
///     components: Vec<Box<HouseElement>>,
/// }
/// impl House {
///     fn new() -> Self {
///         House {
///             components: vec![Box::new(Livingroom::new())],
///         }
///     }
/// }
/// impl HouseElement for House {
///     fn accept(&self, visitor: &HouseElementVisitor) {
///         for component in self.components.iter() {
///             component.accept(visitor);
///         }
///         visitor.visit(self);
///     }
///     fn as_any(&self) -> &Any { self }
/// }
///
/// struct Livingroom;
/// impl Livingroom {
///     fn new() -> Self { Livingroom }
/// }
/// impl HouseElement for Livingroom {
///     fn accept(&self, visitor: &HouseElementVisitor) {
///         visitor.visit(self);
///     }
///     fn as_any(&self) -> &Any { self }
/// }
///
/// struct HouseElementListVisitor;
/// impl HouseElementListVisitor {
///     fn new() -> Self { HouseElementListVisitor }
/// }
///
/// impl HouseElementVisitor for  HouseElementListVisitor {
///     fn visit(&self, element: &HouseElement) {
///         match element.as_any() {
///             house if house.is::<House>() => println!("Visiting the house..."),
///             living if living.is::<Livingroom>() => println!("Visiting the Living room..."),
///             _ => {}
///         }
///     }
/// }
/// struct HouseElementDemolishVisitor;
/// impl HouseElementDemolishVisitor {
///     pub fn new() -> Self {
///         HouseElementDemolishVisitor
///     }
/// }
/// impl HouseElementVisitor for HouseElementDemolishVisitor {
///     fn visit(&self, element: &HouseElement) {
///         match element.as_any() {
///             house if house.is::<House>() => println!("Annihilating the house...!!!"),
///             living if living.is::<Livingroom>() => println!("Bombing the Living room...!!!"),
///             _ => {}
///         }
///     }
/// }
///
/// fn main() {
///     let house = House::new();
///     // simply print out the house elements
///     house.accept(&HouseElementListVisitor::new());
///     println!();
///     // do something with the elements of a house
///     house.accept(&HouseElementDemolishVisitor::new());
/// }
/// ```
///
/// Base usage: RAII 模式
///
/// ```rust
/// /*
/// 利用Rust的ownership（linear/affine type）来设计接口
///
/// 存在的问题：
///
/// 1. Letter有可能复制多份到多个信封（envelope）里，不安全
/// 2. 信封里有可能有信，也有可能没有信；或者同一个信封装多次不同的信件，不安全
/// 3. 有没有把信交给邮车也是无法保证，不安全
///
/// */
///
/// #[derive(Clone)]
/// pub struct Letter {
///     text: String,
/// }
/// pub struct Envelope {
///     letter: Option<Letter>,
/// }
/// pub struct PickupLorryHandle {
///     done: bool,
/// }
/// impl Letter {
///     pub fn new(text: String) -> Self {
///         Letter {text: text}
///     }
/// }
/// impl Envelope {
///     pub fn wrap(&mut self, letter: &Letter){
///         self.letter = Some(letter.clone());
///     }
/// }
/// pub fn buy_prestamped_envelope() -> Envelope {
///     Envelope {letter: None}
/// }
/// impl PickupLorryHandle {
///     pub fn pickup(&mut self, envelope: &Envelope) {
///         /*give letter*/
///     }
///     pub fn done(&mut self) {
///         self.done = true;
///         println!("sent");
///     }
/// }
/// pub fn order_pickup() -> PickupLorryHandle {
///     PickupLorryHandle {done: false , /* other handles */}
/// }
/// fn main(){
///     let letter = Letter::new(String::from("Dear RustFest"));
///     let mut envelope = buy_prestamped_envelope();
///     envelope.wrap(&letter);
///     let mut lorry = order_pickup();
///     lorry.pickup(&envelope);
///     lorry.done();
/// }
///
/// pub fn builder_pattern(){
///     unimplemented!();
/// }
/// ```
///
/// Base usage: 【重构】RAII 模式
///
/// ```rust
/// /*
/// 利用Rust的ownership（linear/affine type）来设计接口
///
/// Refact
///
/// 1. 去掉letter的Clone，使用所有权保证其唯一性；并且信封的wrap方法只接受获得所有权的信封，而非引用
/// 2. 使用类型系统来保证信封的唯一性
/// 3. 使用RAII机制来管理收尾逻辑，比如实现Drop
///
/// 其他示例：
///
/// 1. http response
/// 2. steaming engine
///
/// */
///
/// // #[derive(Clone)]
/// pub struct Letter {
///     text: String,
/// }
/// pub struct EmptyEnvelope {}
/// pub struct ClosedEnvelope {
///     letter: Letter,
/// }
/// pub struct PickupLorryHandle {
///     done: bool,
/// }
/// impl Letter {
///     pub fn new(text: String) -> Self {
///         Letter {text: text}
///     }
/// }
/// impl EmptyEnvelope {
///     pub fn wrap(self, letter: Letter) -> ClosedEnvelope {
///         ClosedEnvelope {letter: letter}
///     }
/// }
/// pub fn buy_prestamped_envelope() -> EmptyEnvelope {
///     EmptyEnvelope {}
/// }
/// impl PickupLorryHandle {
///     pub fn pickup(&mut self, envelope: ClosedEnvelope) {
///         /*give letter*/
///     }
///     pub fn done(self) {}
/// }
/// impl Drop for PickupLorryHandle {
///     fn drop(&mut self) {
///         println!("sent");
///     }
/// }
/// pub fn order_pickup() -> PickupLorryHandle {
///     PickupLorryHandle {done: false , /* other handles */}
/// }
/// fn main(){
///     let letter = Letter::new(String::from("Dear RustFest"));
///     let envelope = buy_prestamped_envelope();
///     let closed_envelope = envelope.wrap(letter);
///     let mut lorry = order_pickup();
///     lorry.pickup(closed_envelope);
/// }
/// ```
pub fn design_patterns(){
    unimplemented!();
}
