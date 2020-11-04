# RustLSystem
Interprets and draws L-Systems with a turtle.  Written in rust.

## What is an L-System?
L-System is short for [Lindenmayer System](https://en.wikipedia.org/wiki/L-system), named after their creator Aristid Lindenmayer.
L-Systems were created by Dr. Lindenmayer to describe the growth patterns of bacteria.

### Defintions
* **Alphabet**:  A collection of characters considered valid in a system.  Example:  `A B F + -`
* **String**:  A list of characters of an Alphabet.  Example:  `AFAB-B` or simply `B`
* **Rule**:  A functional mapping of one character to a string of other characters.  Example:  `A -> BF` (read `A` yields string `BF`)
* **Axiom**:  The starting string of a system.  Example:  `FBFA+B` or simply `A`
* **Constant**:  Shorthand for a character whose rule only results in itself.  Example: `F -> F`
* **Recursive**:  Where the resulting string of the application of the system to a string is fed back through the system to form a new string.

### How It Works
A system is defined with a set **Alphabet**, **Axiom**, and collection of **Rules**.  Example:
```
 Alphabet:  A B
 Rules: 
    A -> AB
    B -> A
 Axiom: A
```
The rules are applied to the axiom, in this case `A`.  Because the rule for the character `A` yields `AB` our resulting string would be `AB`.  This output could be returned through the system recursivly to expand the result.  If we passed the result of our first iteration back through the system we would get `ABA` because `A` yields `AB` like previously and the `B` from the first iteartion yeilds `A`.

A tree of the results up to iteration 2 would look like this:
```
0.      A      // Our starting axiom.
       _|_     // A yields both an A and a B
1.    A   B    //
     _|_   \   // A yields both A and B, B yeilds only an A.
2.  A   B   A  // The order of the results depends on the order of the characters that spawn them in the input.
```
The second `A` in the second iteration would go after **all** of the characters yeilded by the `A` in the first iteration.

A more compicated example:
```
 Alphabet:  A F -
 Constants:  -
 Rules:
    A -> A-F
    F -> FA
 Axiom: A
```
And its tree up to 3 iterations:
```
0.          A                 // Our starting axiom.
          __|__               // A yeilds "A-F"
1.        A - F               //
       __/__ \  \__           // A yields "A-F", - is constant, F yields "FA"
2.     A - F  -  F A          // Arrows for constants will be ommited for clarity.
    __/__   \__   \__ '_,___  // Via an aplication of the rules, "A-F" is expanded to "A-F-FA", - is constant, "FA" becomes "FAA-F"
3.  A - F - F A - F A  A - F  // 
```
The full string or iteration four would be `A-F-FA-FAA-F-FAA-FA-F-FA`.

### Applications
L-Systems are commonly used in conjuntion with graphical programs called Turtles that take a definite list of instructions to follow one by one.  The characters in the string output from an L-System can be mapped to instructions for a Turtle.  The turtle would follow these instructions as they appear in the string.  In my program the string `FF--F+F` would mean go forward twice, turn left twice, go forward once, turn right once, go forward once.