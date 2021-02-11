# Basic grep CLI tool in Rust
This project implements a basic functionality grep CLI (command line interface) tool using Rust. This project is an exercise in [Chapter 12 of the official Rust book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).

*Grep* (**g**lobally search a **r**egular **e**xpression and **p**rint) is a classic command line tool, whose primary function is to search for a user-input string within a given file, and print the lines that contains the queried string.

## Implementation Steps
1. Use [```args()``` a standard library function](https://doc.rust-lang.org/std/env/fn.args.html) to accept command line arguments. Function returns an iterator of input arguments. Collect it into a collection. Save arguments needed for grep.
2. Use [```read_to_string()```](https://doc.rust-lang.org/std/fs/fn.read_to_string.html) to read the entire contents of a text file into memory as a string.
3. Extract out the parsing logic for input command line arguments as a constructor that returns a Result type. If number of arguments is less than 3 (function call, string query, filename), return an error; else return a struct with all program configuration variables.
4. Error handling in ```main()```: if constructor returns a struct, use it; else display a clean and helpful message and exit the program gracefully.