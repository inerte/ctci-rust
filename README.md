# ctci-rust
Cracking the Code Interview (6th edition), in Rust

## TODO
- Chapter 1
 - Question 7

## Thougths

### 1.4

I confess I "cheated" this problem. I couldn't figure out what was being asked,
so I went to the answer to understand the problem, read it twice, and of course
couldn't get the solution out of my mind.
After solving with the first solution, I realized it's one of those problems
that greatly benefit from random knowledge about a shortcut.
If you didn't know a palindrome couldn't have more than one odd character
frequency (and never happen to think about this during the interview), you would
try to brute-force by computing all permutations.

### 1.8

My solution differs uses a different approach than the 2 from the book. My is
probably better if there aren't many zeroes (since it loops on previous rows),
but it's just one loop otherwise. The book solutions loop through the matrix,
and then on the length of column/row many times. If you imagine a very large row,
but with only a few zeroes, my solution would be more efficient (but using more
space).
