# About
This repository contains various implementations of the linked list data structure in Rust.  
The aim of this project was to help me learn Rust design patterns and features of the language.  
Inspiration - [Too Many Lists](https://rust-unofficial.github.io/too-many-lists/)

# What did I learn?
The various files in the src directory are different implementations of the linked list data structure.  
The different "levels" are simply ones that are more memory safe and or faster but also have more complex code.

## Bad Stack **(bad_stack.rs)**
This is a singly linked stack of i32 that implements `push` and `pop`.  
It also implements the Drop trait to ensure memory safety.  
  
I learned the following from this implementation -
* Using Cargo to create and manage projects
* How to define structs, enums and implementation blocks
* The Option enum and its uses
* The difference between C enums and enums in Rust
* The `new() returns Self` design pattern
* Use of basic macros like `unimplemented!` and `assert_eq!`
* Writing tests using `assert_eq!`

## Ok Stack **(ok_stack.rs)**
This is a generic singly linked stack that implements `peek` as well.  
It also implements the Iterator trait and provides all 3 types of iterators.  
  
I learned the following from this implementation -
* Option methods like `take()` and `map()`
* Inline function syntax
* Using Generic templates
* How little I know about lifetimes
* Difference between and uses of `IterInto`, `Iter` and `IterMut`
* Option background optimizations
