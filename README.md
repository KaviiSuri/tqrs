# Tqrs

This project is inspired by tqdm, the python package/cli utility.

This project is a crate and also a cli tool.

## Usage - Library
This library lets you add progress bars to rust Iterators while using them normally.

Example:
```rust
use std::thread::sleep;

fn expensive_calc(_n: &i32) {
    sleep(std::time::Duration::from_secs(1))
}

fn main() {
    let v = vec![1, 2, 3];
    dbg!(&v);
    use tqrs::TqrsIteratorExt;
    for n in v.iter().tqrs().with_bound().with_delims(('<', '>')).with_bar("*") {
        expensive_calc(n);
    }
    for n in (0..).tqrs() {
        expensive_calc(&n);
        if n == 10 {
            break;
        }
    }
    println!();
}
```

## Usage - Cli

This is still a work in progress. But here's the usage that's planned

```bash
$> seq 999999999 | tqrs --bytes | wc -1
75.2MB [00:00, 217MB/s]
9999999
```

## Motivation

The rust type system and especially traits allow for a lot of interesting things. One of them is adding methods to existing. Looking at tqdm and rust's type system inspired me to make this.

I'm still a beginner in rust so I'd love feedback for the code style.
