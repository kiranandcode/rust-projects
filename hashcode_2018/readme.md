# Hashcode 2018 - Manhattan Block Ride Assignment

## Problem statement
You are given a set of car rides designated by (start location,end location) pair and an earliest start time and a latest start time.
Coordinates are given in context of a manhatten block grid system, and time flows through a discreet unit system, whereby it takes 1 unit of time to
travel along one grid block.
You are given n vehicles all of the same speed, and must assign rides to the vehicles, which will perform the rides in given order.
Depending on the problem, bonuses may be given if the rides start exactly at the earliest start time.

## Solution
This was a little proof of concept solution to the hashcode 2018 problem set.
The idea I had on the day was to construct a 2D graph from the rides, whereby two rides A,B are connected by a directed edge A -> B iff starting at the start position for ride A, it is possible to complete ride A and then drive to ride B such that there is enough time to complete ride B before it's latest end.

Unfortunately, I was unable to implement this solution on the day, but was interested in evaluating the effectivenes of the solution. Thus following the live round, I wrote up the solution in Rust, and ran the solutions produced by the algorithm on the online judge.

After running the algorithm in the judge, I was able to obtain a final score of 40,000,000, which ranked my team at 997th position, with the top scores being in the 50,000,000. So while the algorithm worked per say, it might not have been the optimal solution.
