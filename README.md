# from_wrap
The macro to generate simple implementation of From, when the target type value is a wrapper on source type value.

## Installation

If you're using `Cargo` to manage dependencies, just add from_wrap to the `Cargo.toml`:

```
[dependencies]
from_wrap = { git = "https://github.com/rust-toolbox/from_wrap.git" }
```

## Usage

```rust
#[macro_use]
extern crate from_wrap;

#[derive(FromWrap)]
struct MyTuple(String);

#[derive(FromWrap)]
struct MyStruct {
    s: String
}

#[derive(FromWrap)]
enum MyEnum {
    Str { s: String },
    Int(i32),
    SomeUnit
}
```

This will generate

```rust
impl From<String> for MyTuple {
    fn from(value: String) -> MyTuple {
        MyTuple(value)
    }
}

impl From<String> for MyStruct {
    fn from(value: String) -> MyStruct {
        MyStruct { s: value }
    }
}

impl From<String> for MyEnum {
    fn from(value: String) -> MyEnum {
        MyEnum::Str { s: value }
    }
}

impl From<i32> for MyEnum {
    fn from(value: i32) -> MyEnum {
        MyEnum::Int(i32)
    }
}
```
If you want to generate From implementations for only some variants, use `generate_from_wrap` or `not_generate_from_wrap` attributes as follows:
```rust
#[derive(FromWrap)]
enum MyEnum {
    Str { s: String },
    Int(i32),
    SomeUnit,
    #[not_generate_from_wrap]
    Pair(i32, i32),
    #[not_generate_from_wrap]
    Flt(f64),
    #[not_generate_from_wrap]
    SomeStruct { a: i32, b: i32 }
}
```
or
```rust
#[derive(FromWrap)]
enum MyEnum {
    #[generate_from_wrap]
    Str { s: String },
    #[generate_from_wrap]
    Int(i32),
    SomeUnit,
    Pair(i32, i32),
    Flt(f64),
    SomeStruct { a: i32, b: i32 }
}
```
The From implementation will not be generated for Enum variants such as Units, Structs and Tuples containing more than one field.

## License

Public Domain