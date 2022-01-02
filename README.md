# Integer Char CLI Conversion Program

## What does this program do?

The program has two modes:
- Integer to Character conversion
- Character to Integer conversion

The program will prompt the user to either insert an integer or a unicode character, depending
on the mode. The program will then convert it to the other type.

The mode the program is in is selected once the program is started. Since rust uses
unicode for the characters, the integers will have to be in that format, but since
it is backwards compatible with ASCHII those values will work as well.

## Why was this created?

Mostly to gain experience using these elements of the rust language:
- character and integer primitives
- console input and output
- functions
- input parameters
