# How to learn a new programming language

Let's solve problems in [Advent of code](https://adventofcode.com/2023) 2023 challenge to learn Rust.

Each day of the Advent period includes a pair of challenges to solve.

Create a dedicated commit for each day. Pushing code regularly serves a a SMART *(Specific, Measurable, Achievable, Relevant, Time-bound)* goal that helps measuring your own progress.

Having a regular pace will indicate you're making consistent progress during the learning process.

# Environment setup

## Runtime
Install [Rust programming language](https://www.rust-lang.org/tools/install).

## IDE
Install [Visual Studio Code](https://code.visualstudio.com/). It will serve as an IDE to edit, compile and run the code. In the *extension marketplace* in Visual Code, install **Rust** & **AWS toolkit** extensions.

## Git
Install [git](https://git-scm.com/downloads). Create a new repository in github, like [AdventOfCode](https://github.com/alexis779/AdventOfCode).


# Dev setup
Create a new workspace

```
$ cargo new advent_of_code_2023
```

# Dev process

## Product Requirements

Read the problem statement, say [day1](https://adventofcode.com/2023/day/1). This initial step is equivalent to gathering the product requirements.

## Problem Solving
Work out the outline of a possible solution. Think of the data structures and algorithms you need to solve the problem.

## Unit test
- create a new input file for the test sample in `inputs/day1.txt`
- add 2 new unit test functions in `src/lib.rs`

Writing a unit test establishes the contract the module is responsible for. Create a file containing the sample input. Then call a public function that parses this file using the file name as a String parameter. Carefully choose the function name to be self-explanatory. Add a test assertion on the actual value the function should return based the content of the input file.

## Implement a solution
- create a new module in `src/day1.rs`
- add 2 new public functions in the new module `src/day1.rs`

Implement the algorithm you came up with in the problem Solving part, using adequate data structures and business models, until the tests pass.

Add all the implementation in a dedicated module under test coverage.

It should include a set of functions providing the implementation. They each take a set of inputs and return an output, providing a top-down execution of the code organized as a Russian doll game.

It should also contain the definitions of the models using a set of `struct` declarations.

## Submit the solution
- create a new input file for the application sample in `inputs/day1_input.txt`
- update the main application in `src/main.rs` to call the public function. It will output the result you submit to solve the 2 parts of the challenge. This will grant you 2 gold starts.

## Push the new commit

Make sure the tests pass

```
$ cargo test
```

Then push the code
```
$ git push
```

# Rust books
When blocked, read a book documenting the key features of the programming language.
- https://github.com/Dhghomon/easy_rust?tab=readme-ov-file#introduction

# Start a conversation with Chat bots
- [Mistral](https://chat.mistral.ai/chat)

Great for low level syntax review.
Confront the generated version with your own implementation

- [Claude](https://claude.ai/chat)

Great for high level suggestions on coding best practices.
It may also be able to spot your bug ?

- Amazon Q

Click the hexagon icon in the left panel in Visual Code to open it.
Its answers provide external links to relevant blog articles for general questions.

# Install a code assistant
*Amazon CodeWhisperer* is available as part of *AWS Toolkit* extension after signing up for an *AWS builder id*.

1. Code assistant will generate a one-liner or even a block of code based on what you're starting to type.

2. Write a brief description of what you intend to do as a code comment. It prompts the model prior to generating an implementation of the spec provided in the comment.

This will be hit or miss, but serves as a placeholder when stuck when trying to come up with valid syntax.

You can approve the hint by pressing tab or discard it by pressing Escape.