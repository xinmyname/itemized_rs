# itemized_rs

This is the Rust version of Itemized. The program adds items to inventory and then prints out how many items you have. You can specify how many items to add by passing a number as an argument. If you don't specify any items, you will receive one item. 

Yes, it's very simple. That's the idea.

## Example

```
$> cargo run
You have:
  An item
$> cargo run 4
You have:
  Four items
$> cargo run 42
You have: 
  42 items
```

Or, alternatively...

```
$> cargo build --release
$> ./target/release/itemized_rs 42
You have:
  42 items
```

## Prerequisites
- rust 1.17

## Notes
Modules and lifetimes were a little weird, but pretty nice all around!
 
## The Good Parts
- No runtime
- Compiles to native executable
- Very easy builds

## Airing of Grievances
- Lifetimes are a little weird
- Piecing together intra-package modules is more difficult than it should be
