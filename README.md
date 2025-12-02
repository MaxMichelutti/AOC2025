# Advent of Code 2025

This year we have a shorter than usual, 12-days advent of code.<br>

I will be publishing my solutions here, but since I am very busy lately, I will likely finish later than December 12th.<br>

Anyway, at least I remembered this year, so it will definitely be better than 2024.<br>

For this year, I plan to develop a decent Rust solution for each problem, since in the last few months, Rust has been my most used programming language. <br>

Feel free to use these solutions if you are stuck, but please, in the spirit of the challenge, try to come up with a solution of your own before relying on someone else's.

## My thoughts and comments on each day

### Day 1

Pretty simple problem allowing me to prepare a decently general structure for the days to come. It still
took me quite some time and a couple of mistakes due to some edge cases though.

### Day 2

Once again a relatively simple pair of problems, which hides a nasty edge case in part two. 
Luckily, the example already provides one occurrence of the edge case, so it was easy to spot.
Unfortunately, I was unable to find a very fast approach to handle the edge case in part two, so I had
to partially rely on brute force to check the ranges. Since the input did not
contain extremely wide ranges, my laptop was able to handle the problem in milliseconds anyway.