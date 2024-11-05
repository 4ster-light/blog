This is the content of my first blog post. It supports **Markdown** formatting!

## Code Examples

```rust
fn main() {
    struct Fibonacci {
        a: i32,
        b: i32,
    }
    impl Iterator for Fibonacci {
        type Item = i32;
        fn next(&mut self) -> Option<i32> {
            let next = self.a + self.b;
            self.a = self.b;
            self.b = next;
            Some(next)
        }
    }

    for n in Fibonacci {
        println!("{}", n);
    }
}
```

## Lists

- Item 1
- Item 2
- Item 3

### Nested Lists

- Item 1
  - Item 1
    - Nested Item 1
  - Item 2
    - Nested Item 2
- Item 2

## Tables

| Tables        | Are           | Cool  |
|---------------|---------------|-------|
| col 3 is      | right-aligned | $1600 |
| col 2 is      | centered      |   $12 |
| zebra stripes | are neat      |    $1 |

## Links

[I'm an inline-style link](https://www.google.com)
[I'm an inline-style link with title](https://www.google.com "Google's Homepage")
