
- Start with 2 coördinates: Head and Tail (struct: coördinate) (x and y: isize, can be negative)
- Keep a hashmap of coördinates; key: coördinate, value: amount

- Implement movements for head
- Calculate where tail should go => do this only when the tail is not adjacent to the head:
    - If tail is aligned with head horizontally or vertically: move towards it
    - If this is not the case: a diagonal movement is made to align head and tail

Example

Hxx    HTx
xxT -> xxx
xxx    xxx

{ x: 3, y: -3 }
{ x: 0, y: 0 }
{ x: 3, y: 0 }
{ x: 3, y: -2 }
{ x: 1, y: -3 }
{ x: 1, y: 0 }
{ x: 3, y: -1 }
{ x: 2, y: -3 }
{ x: 4, y: -3 }
{ x: 2, y: 0 }


R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2

.####    
...#.
...#.
####.


..##..
...##.
.####.
....#.
####..

