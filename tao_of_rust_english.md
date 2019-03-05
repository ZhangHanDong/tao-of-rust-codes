# The Tao of Rust
《Rust编程之道》

## Chapter 1:  Language For the New Age

### 1.1 Origins

### 1.2 Design Philosophy

- 1.2.1 Memory Safety
- 1.2.2 Zero Cost abstraction
- 1.2.3 Practicality

### 1.3 Status and Future

- 1.3.1 Language Architecture
- 1.3.2 Open source community
- 1.3.3 Future development

### 1.4 How Does the Rust Code execute?

### 1.5 Summary

## Chapter 2: Language Essentials

### 2.1 The basic structure of the Rust Language 

- 2.1.1 Language Specification
- 2.1.2 Compiler
- 2.1.3 Core Library
- 2.1.4 Standard Library
- 2.1.5 Package Manager

### 2.2 Statements and Expressions

### 2.3 Variables and bindings

- 2.3.1 Position Expressions and Value Expressions
- 2.3.2 Immutable binding and   Mutable  binding
- 2.3.3 Ownership and  Reference

### 2.4 Functions and Closures

- 2.4.1 Function definitions
- 2.4.2 Scope and LifeTime
- 2.4.3 Function pointers
- 2.4.4 CTFE mechanism
- 2.4.5 Closure

### 2.5 Control of Flow

- 2.5.1 Conditional Expressions
- 2.5.2 Loop expressions
- 2.5.3 match expression and pattern matching
- 2.5.4 if let and while let expressions

### 2.6 Basic data types

- 2.6.1 Boolean Type
- 2.6.2 Basic number Type
- 2.6.3 Character Type
- 2.6.4 Array type
- 2.6.5 Range Type
- 2.6.6 Slice Type
- 2.6.7 str  Type
- 2.6.8 Raw Pointers
- 2.6.9 Never Type

### 2.7 Composite Data Types

- 2.7.1 tuples
- 2.7.2 Structure
- 2.7.3 Enumerators

### 2.8 Common Collection Types

- 2.8.1 Linear sequence: Vector
- 2.8.2 Linear Sequence: Double-Ended Queue
- 2.8.3 Linear sequence: Linked List
- 2.8.4 Key-Value mapping table: HashMap and BTreeMap
- 2.8.5 Collections: HashSet and BTreeSet
- 2.8.6 Priority Queue: BinaryHeap

### 2.9 Smart Pointers

### 2.10  Generics Type and traits

- 2.10.1 Generics Type
- 2.10.2 trait

### 2.11 Error Handling

### 2.12 Expression Priority

### 2.13 Notes and Prints

### 2.14 Summary

## Chapter 3: Type System

### 3.1 General Concepts

- 3.1.1 The role of the type system
- 3.1.2 Classification of type systems
- 3.1.3 Type System and Polymorphism

### 3.2 Rust type system overview

- 3.2.1 Type size
- 3.2.2  Type Inference

### 3.3 Generics Type

- 3.3.1 Generic Functions
- 3.3.2 Automatic return of generic return values

### 3.4 In Depth trait

- 3.4.1 Interface abstraction
- 3.4.2 Generic constraints
- 3.4.3 Abstract Types
- 3.4.4 Marker  trait

### 3.5 Type Conversion

- 3.5.1 Deref Dereference
- 3.5.2 as Operator
- 3.5.3 From and Into

### 3.6 Weakness of the current trait

- 3.6.1 Limitations of orphan rules
- 3.6.2 Code reuse is not efficient
- 3.6.3 Abstract expression ability needs to be improved

### 3.7 Summary

## Chapter 4 : Memory Management

### 4.1 General Concepts

- 4.1.1 Stack
- 4.1.2 Heap
- 4.1.3 Memory layout

### 4.2 Resource Management 
in Rust

- 4.2.1 Variables and Functions
- 4.2.2 Smart Pointer and RAII 
- 4.2.3 Memory Leakage and Memory Safety
- 4.2.4 Composite type of memory allocation and layout

### 4.3 Summary

## Chapter 5: Ownership System

### 5.1 General Concepts

- 5.3.1 Immutable and Mutable
- 5.3.2 The temporal properties  of binding: LifeTime

### 5.2 Ownership Mechanism

### 5.3 Binding, Scope and LifeTime

### 5.4 Ownership Borrowing

### 5.5 LifeTime Parameters

- 5.5.1 Explicit LifeTime parameters
- 5.5.2 Omitting lifeTime parameters
- 5.5.3 LifeTime Bound
- 5.5.4  LifeTime of   trait Object

### 5.6 Smart pointers and ownership

- 5.6.1 Shared Ownership:  Rc<T> and Weak<T>
- 5.6.2 Internal mutable:  Cell<T> and RefCell<T>
- 5.6.3 Copy on Write: Cow<T>

### 5.7 Concurrency Safety and Ownership

### 5.8 Non-lexical Scope LifeTime

### 5.9 Summary

## Chapter 6: Functions, Closures, and Iterators

### 6.1 Functions

- 6.1.1 Function Shadow
- 6.1.2  Function arguments and Pattern Matching
- 6.1.3  Return value
- 6.1.4 Generic Functions
- 6.1.5 Methods and Functions
- 6.1.6 Higher order functions

### 6.2 Closures

- 6.2.1 Basic syntax of closures
- 6.2.2 Implementation of closures
- 6.2.3 Closures and Ownership
- 6.2.4 Closures as function arguments and return values
- 6.2.5 Higher-order LifeTime

### 6.3 Iterators

- 6.3.1 External iterators and internal iterators
- 6.3.2 Iterator trait
- 6.3.3 IntoIterator trait and Iterator
- 6.3.4 Iterator Adapters
- 6.3.5 Consumer
- 6.3.6 Custom Iterator Adapter

### 6.4 Summary

## Chapter 7: Structured Programming

### 7.1 Object-Oriented Style Programming

- 7.1.1 Structure
- 7.1.2 Enumerators
- 7.1.3 Destruction order

### 7.2 Common Design Patterns

- 7.2.1 Builder  Pattern
- 7.2.2 Visitor Pattern
- 7.2.3 RAII Pattern

### 7.3 Summary

## Chapter 8:  Strings and Collection Types

### 8.1 Strings

- 8.1.1 Character encoding
- 8.1.2 Characters
- 8.1.3 String classification
- 8.1.4 Two ways of dealing with strings
- 8.1.5 Modification of Strings
- 8.1.6 Finding Strings
- 8.1.7 Conversion with other types
- 8.1.8 Review

### 8.2 Collection Types

- 8.2.1 Dynamically Growable Array
- 8.2.2 Mapping Sets

### 8.3 Understanding Capacity

### 8.4 Summary

## Chapter 9: Building a Robust System

### 9.1 General Concepts

### 9.2 Eliminate the failure

### 9.3  Layer-based 
error handling

- 9.3.1 Optional Values : Option<T>
- 9.3.2 Error Handling : Result<T, E>

### 9.4 Panic

### 9.5 Third Party Library

### 9.6 Summary

## Chapter 10: Modular Programming

### 10.1 Package Management

- 10.1.1 Creating a crate with Cargo
- 10.1.2 Use of third party  crates
- 10.1.3 Cargo.toml file format
- 10.1.4 Custom Cargo

### 10.2 Module System

### 10.3 Implementing a
 full function crate
  from scratch

- 10.3.1 Creating a new project with Cargo
- 10.3.2 Using structopt to parse command line arguments
- 10.3.3 Defining a uniform error type
- 10.3.4 Reading CSV files
- 10.3.5 Replacing the contents of a CSV file
- 10.3.6 Further improve the crate

### 10.4 Visibility and Privatility

### 10.5 Summary

## Chapter 11: Concurrency Safety

### 11.1 General Concepts

- 11.1.1 Multi-process and Multi-threading
- 11.1.2 Event Driven, Asynchronous Callbacks, and Coroutines
- 11.1.3 Thread Safety

### 11.2 Multi-threading  
Concurrent Programming

- 11.2.1 Thread Management
- 11.2.2 Send and Sync
- 11.2.3 Thread synchronization using locks
- 11.2.4 Barriers and Conditional Variables
- 11.2.5 Atomic Types
- 11.2.6 Inter-thread communication using Channel.
- 11.2.7 Internal Mutability Inquiry
- 11.2.8 Thread Pool
- 11.2.9 Performing Parallel Tasks with Rayon
- 11.2.10 Using Crossbeam

### 11.3 Asynchronous Concurrency

- 11.3.1 Generators
- 11.3.2 Future Concurrency Mode
- 11.3.3 async/await

### 11.4 Data Parallel

- 11.4.1 What is SIMD
- 11.4.2 Using SIMD in Rust

### 11.5 Summary

## Chapter 12: Metaprogramming

### 12.1 Reflection

- 12.1.1 Judging the type by the is function
- 12.1.2 Conversion to a specific type
- 12.1.3 Non-static LifeTime types

### 12.2 Macro System

- 12.2.1 Origins
- 12.2.2 Classification of macros in Rust
- 12.2.3 Compilation Process
- 12.2.4 Declaration Macros
- 12.2.5 ProcedureMacros

### 12.3 Compiler Plugin

### 12.4 Summary

## Chapter 13 Beyond the Safety

### 13.1 Introduction to Unsafe Rust

- 13.1.1 Unsafe Syntax
- 13.1.2 Accessing and Modifying Mutable Static Variables
- 13.1.3 Union Type
- 13.1.4 Dereferencing raw pointers

### 13.2 Safe abstraction 
based on Unsafe

- 13.2.1 Raw pointers
- 13.2.2 Subtypes and Types
- 13.2.3 Unbound lifeTime
- 13.2.4 Drop Check
- 13.2.5 NonNull<T> pointer
- 13.2.6 Unsafe and Panic safety
- 13.2.7 Heap Memory Allocation
- 13.2.8  Three Principles of Mixed Code‘s Memory Safety Architecture 

### 13.3 Interacting with 
other languages

- 13.3.1 Foreign Function Interface
- 13.3.2 Interacting with the C/C++ Language
- 13.3.3 Using Rust to Improve Dynamic Language Performance

### 13.4 Rust and WebAssembly

- 13.4.1 WebAssembly Highlights
- 13.4.2 Developing WebAssembly with Rust 
- 13.4.3 Building a WebAssembly Development Ecosystem
- 13.5 Summary

*XMind: ZEN - Trial Version*