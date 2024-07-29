# Part 1
## Task
You get 2d grid fo cells containing either
- Your start location
- A target location
- Walls
- Empty Cells

You can move in each of the four cardinal directions

The goal is to visit all target locations in the fewest amount of steps.

## Thoughs
It's basically become a traveling salesman problem.

### First solution idea
find the shortest path between every pairs of points generating a weighted graph which can then be brute forced.

#### Problems

