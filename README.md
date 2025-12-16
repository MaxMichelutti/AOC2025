# Advent of Code 2025

This year we have a shorter than usual, 12-days [advent of code](https://adventofcode.com/2025).<br>

I will be publishing my solutions here, but since I am very busy lately, I will likely finish later than December 12th.<br>

Anyway, at least I remembered this year, so it will definitely be better than 2024.<br>

For this year, I plan to develop a decent Rust solution for each problem, since in the last few months, Rust has been my most used programming language. <br>

Feel free to use these solutions if you are stuck, but please, in the spirit of the challenge, try to come up with a solution of your own before relying on someone else's.

To run any of my solutions run cargo inside the respective day folder.
```
    cargo run -- <PATH/TO/YOUR/INPUT/FILE>
```
No particular dependencies should be required outside of [cargo](https://rust-lang.org/tools/install/).

## My thoughts and comments on each day

### Day 1

Pretty simple problem allowing me to prepare a decently general structure for the days to come. It still
took me quite some time and a couple of mistakes due to some edge cases, though.

### Day 2

Once again a relatively simple pair of problems, which hides a nasty edge case in part two. 
Luckily, the example already provides one occurrence of the edge case, so it was easy to spot.
Unfortunately, I was unable to find a very fast approach to handle the edge case in part two, so I had
to partially rely on brute force to check the ranges. Since the input did not
contain extremely wide ranges, though, my laptop was still able to handle the problem in milliseconds.

### Day 3

Easiest problem of the bunch until now, had a lot of fun solving it with my friends after a soccer match!<br>
I expected a far harder part two, but the A.O.C. creators had some mercy today.

### Day 4

Part two ended up being literally part one, just iterated multiple times. Kind of underwhelming since part one was already pretty easy.
There could just have been a very small quirk that could have given the wrong answer to unexperienced programmers who may have removed 
the paper rolls as they counted them (before ending each iteration). But I say <i>"could"</i> for a reason, and that reason is that it turns out this approach still gives the correct answer anyway, well, in part two at least.<br>
This kinda makes part one harder than part two since removing paper rolls during computation of part one may give incorrect results for some, but modifying the original problem isn't really of any use in that part.

### Day 5

Part one was very easy, while part two starts to show how tough advent of code can be on some days. Merging the ranges into a single collection of ranges
required a lot of thinking about all possible scenarios and some recursive thinking aswell. I personally handled it with 9 different cases, but maybe it could be handled with a few less. I also got the incorrect answer once due to a small mistake I made, which caused the collection of ranges to be partially wiped mid execution. The mistake was forgetting to reconnect a link, and was solved by adding line ```82``` in [problem.rs](day05/src/problem.rs). Had a lot of fun solving part two, while meeting some old friends!<br>
Turns out there is a bug in my code and me getting the solution was just dumb luck that a specific subcase did not happen. I may fix the code in the future, for now it is not a bug, it is a feature.<br>
I indeed fixed my code. Anyway, congratulations to my friend Anna who came up with a far nicer solution than mine, which leverages a greedy approach
with the idea of sorting the ranges by their starting point in order to easily combine them.

### Day 6

I know this is a bit late, but I was busy skiing and getting sick in the last few days, so I am now catching up with AOC. 
In the future I will try to do one problem a day, so I hope to finish with a 4 day delay if everything goes to plan.<br>
Anyway, these are my thoughts on day 6.<br>
Today (or 4 days ago to be precise) we got a pretty simple problem which was all about parsing the input in the correct way.
Part 2 surprised me so much that I had to write a completely separate parser, since the one I used for part one could not be used for it.
Anyway, once parsing was done the problem became a walk in the park, since all you need to do is adding or multiplying a small amount of numbers.<br>
We are now halfway there with AOC, up to now the challenge was easier than expected, and I am looking forward to the challenges that the final half of days 
will reserve!

### Day 7

Not much to say on this one, once again a nice and easy problem, which I had a lot of fun drawing and updating the current status for.
Rust came up really helpful thanks to its amazing enum system, allowing for a really well organized representation of the problem for both part one
and part two.

### Day 8

Weird day in which part one was miles harder than part two. I was a bit annoyed at the fact that I felt that the problem statement for part one was a bit ambigous.
If it was stated a bit better, maybe by showing the exact circuits of the example, I would have liked it better.
Anyway, this is the first day that I introduce an external dependency to this project! That is, because today <u>disjoint sets</u> were very helpful, and I don't think there is a disjoint set implementation inside Rust std library. 
So, I used [this](https://docs.rs/disjoint/latest/disjoint/) implementation, which worked very well.<br>
To be fair, I am grateful that part two was quick and easy, because I was incredibly scared it would have been some twist to the Travelling Salesman Problem, and,
as any computer science student would know, that is a very hard problem (so hard it is very likely that it will never be solved in polynomial time in its vanilla version). So, thank you for that, AOC.

### Day 9

Part one today was so quick, I did it in less than 5 minutes. Part two, however, required some thinking and some analysis of the given input.
In fact, I was able to come up with a fast solution that works with most inputs, but not all, so if your does not work that may be the reason.
I made the assumption that the problem never contains two adjacent lines, which would make the white spece between such lines 0, thus forcing
me to ignore those lines and rethink the problem. After making this assumption, part two turned out to be nothing more than a couple verbose 
comparisons between integers added to part one.

### Day 10

I won't spend much time writing about part one since I was able to solve it relatively quickly with a simple DFS. Not the most efficient way to do it, but it works.<br>
But part two, oh part two, how hard you were today!<br>
I read the task and I instantly knew it was going to be hard. <br>
As a first attempt, I tried to extend the DFS method used in part one, since the values in the problem did not look incredibly high. 
However, while this worked flawlessly for the example, once I treid it on the actual input, my laptop was taking ages to even solve the first of the 200 machines.
Watching that unffold, I instantly knew I had to improve my solution.<br>
However, I was completely lost! <br>
I though about a DP solution, but it simply would not even fit in RAM for some machines, so I discarded it.<br>
So I looked online, and I read of some people treating the problem as if it was a system of linear equations, and that guided me towards my solution.<br>
So... just convert the problem into a matrix and then solve the system, right?<br>
Well, it was not as simple as it may look. I haven't manually solved problems of linear algebra for a while, so I was very rusty on the topic,
and I had to relearn some concepts such as Gaussian elimination in order to even come up with a solution.<br>
But the issues did not stop there: a solution is not enough for today's advent of code problem, which requires the best solution among all.
While most of the machines had exactly one solution, some had an infinite amount, and I had to brute force some values for the free variables in order
to find the best solution of all.<br>
So, in the end, my solution takes around 30 seconds to run on my input, and most of the time is spent brute forcing the optimal values for the free variables.<br>
In my opinion, today's problem was a little too hard, but I'm still happy I was able to solve it, and I'm grateful that it taught me some linear
algebra concepts that I had completely forgotten.
While at first it made me rage a bit, in the end it was a positive experience overall.

### Day 11

Short story of why you should always double check that your problem is parsed correctly. Today I lost like 15 minutes just because I forgot to append to the problem the last line of the input. If it was not for that, I would have solved today's problem very quickly: I was very lucky that avoiding the easy brute force solution for part one, ended up payoing for part two, since all I had to do was re-use the same code with different start and end points. <br>
My opinion on today's advent of code, is that it was overall easy, especially when compared with yesterday. 
Graph theory knowledge was the key for a quick and fast solution.<br>
Looking forward to tomorrow, which will be the last chapter of this adventure!

### Day 12

Gotta be honest, I was completely lost today. The problem was so hard that the only way I could think of solving it in the general case was to feed it to a powerful SMT solver and hope that such a tool could crack it. Luckily, before I started encoding the problem, I decided to try to exclude the instances that were undoubtedly impossible because the region was too small to host all required shapes no matter the position and orientation. That left me with about half the regions to check, and before starting the encoding procedure I decided to attempt to feed the amount of possible regions to the AOC website, since the shapes I was given kinda looked like they could fit optimally in a big rectangle. Big surprise: that amount was the actual solution for today!<br>
There is no part two today since this is the last day of advent of code,
and, as per tradition, the last star is given for free if all other stars are collected.<br>