#set page(
    // width : auto,
    height : auto,
    fill : none,
    margin : 0.4cm
)

#import "@preview/codly:1.3.0": *
#import "@preview/codly-languages:0.1.1": *



#show: codly-init.with()

#codly(languages: codly-languages)
```rust
pub fn main() {
    println!("Hello, world!");
}
```
