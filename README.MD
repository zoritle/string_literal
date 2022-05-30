# String Literal

A simple macro that auto insert `.to_string()` call to str literal.

It can save many keystrokes while writing test.

```rust
s!{
    MyStruct {
        a: "a",
        b: "b",
        c: 1,
    }
}
```

will expand to

```rust
MyStruct {
    a: "a".to_string(),
    b: "b".to_string(),
    c: 1,
}
```

## TODO

Currently embed macro won't work, so you can't use `s!{vec!["a","b","c"]}`

## similar project

[overload-strings](https://lib.rs/crates/overload-strings)
