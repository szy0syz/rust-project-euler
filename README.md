# rust-project-euler

## p001 - Multiples of 3 and 5

```rust
let mut sum = 0;
for i in 1..1000 {
    if i % 3 == 0 && i % 5 == 0 {
        sum += 1;
    }
}
```
