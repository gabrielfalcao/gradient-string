# Gradient String

Gradient String is a safe crate to iterate over a gradient of
permutations of string slices.

## Example

```rust
use gradient_string::Gradient;

let result = Gradient::new(" abc ")
    .collect::<Vec<String>>();

assert_eq!(
    result,
    vec![
        " ", "a", "b", "c", " ", " a", "ab", "bc", "c ", " ab", "abc", "bc ", " abc",
        "abc ", " abc "
    ]
);
```
