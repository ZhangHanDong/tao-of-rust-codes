/// # 制造内存泄漏
///
/// Base usage: 内存泄漏不属于内存安全范围
///    内存泄漏是可以通过相互引用精心构造出来的
/// ```
/// fn main() {
///     use std::rc::Rc;
///     use std::cell::RefCell;
///     type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;
///     struct Node<T> {
///         data: T,
///         next: NodePtr<T>,
///     }
///     impl<T> Drop for Node<T> {
///         fn drop(&mut self) {
///             println!("Dropping!");
///         }
///     }
///     fn main() {
///         let first = Rc::new(RefCell::new(Node {
///             data: 1,
///             next: None,
///         }));
///         let second = Rc::new(RefCell::new(Node {
///             data: 2,
///             next: Some(first.clone()),
///         }));
///         first.borrow_mut().next = Some(second.clone());
///         second.borrow_mut().next = Some(first.clone());
///     }
/// }
/// ```
fn memory_leak(){
    use std::rc::Rc;
    use std::cell::RefCell;
    type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;
    struct Node<T> {
        data: T,
        next: NodePtr<T>,
    }
    impl<T> Drop for Node<T> {
        fn drop(&mut self) {
            println!("Dropping!");
        }
    }
    fn main() {
        let first = Rc::new(RefCell::new(Node {
            data: 1,
            next: None,
        }));
        let second = Rc::new(RefCell::new(Node {
            data: 2,
            next: Some(first.clone()),
        }));
        first.borrow_mut().next = Some(second.clone());
        second.borrow_mut().next = Some(first.clone());
    }

}
