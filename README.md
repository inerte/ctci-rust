# ctci-rust
Cracking the Code Interview (6th edition), in Rust

## TODO
- Chapter 1
 - Question 7

## Thougths

### 1.8

My solution differs uses a different approach than the 2 from the book. My is
probably better if there aren't many zeroes (since it loops on previous rows),
but it's just one loop otherwise. The book solutions loop through the matrix,
and then on the length of column/row many times. If you imagine a very large row,
but with only a few zeroes, my solution would be more efficient (but using more
space).
