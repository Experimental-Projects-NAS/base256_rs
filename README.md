# base256_rs
Base 256 encoder/decoder for Rust

## Installation

```shell
$ cargo add base256_lib
```

## Usage

From uint8 array to base256 string:

```rust
use base256_lib::{u82base256};
...
let bytes: &[u8] = b"Hello, World!";
let result = u82base256(bytes);

println!("result: {}", result); // should prints "ĨŅŌŌŏČĀķŏŒŌńā"
```

From base256 string to uint8 array:
```rust
use base256_lib::{base2562u8};
...
let input: &str = "ĨŅŌŌŏČĀķŏŒŌńā";
let result = base2562u8(input);

let result_str = match str::from_utf8(&result) {
  Ok(v) => v,
  Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
};
println!("result: {}", result_str); // should prints "Hello, World!"
```

## Charset

Charset used :
```
àáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿĀāĂăĄąĆćĈĉĊċČčĎďĐđĒēĔĕĖėĘęĚěĜĝĞğĠġĢģĤĥĦħĨĩĪīĬĭĮįİıĲĳĴĵĶķĸĹĺĻļĽľĿŀŁłŃńŅņŇňŉŊŋŌōŎŏŐőŒœŔŕŖŗŘřŚśŜŝŞşŠšŢţŤťŦŧŨũŪūŬŭŮůŰűŲųŴŵŶŷŸŹźŻżŽžſƀƁƂƃƄƅƆƇƈƉ=_-`~|[]{}ƞ?,()^*$%!#.ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/
```