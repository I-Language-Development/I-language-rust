# The I-Language book
By [@MasterOktagon](https://github.com/MasterOktagon) and [@ElBe-Plaq](https://github.com/ElBe-Plaq).

## Contents

- [The I-Language book](#the-i-language-book)
  - [Contents](#contents)
  - [1. Why use the I-Programming-Language?](#1-why-use-the-i-programming-language)
  - [2. FAQ](#2-faq)
  - [3. Tutorials](#3-tutorials)
    - [3.1 Hello World](#31-hello-world)
      - [3.1.1 How does it work?](#311-how-does-it-work)
    - [3.2 Guess the number](#32-guess-the-number)

## 1. Why use the I-Programming-Language?

So why should you use the I-Programming-Language? I have tested a lot of programming languages and not found the one I like most. I am generally a fan of Object-Oriented
programming languages. I have used Java and C/C++ a lot and can say that I like them very much. Java however has a far to complicated system library built-in
and produces (on my opinion) to big projects. Also, Java only supports JIT compiling. While this allows it to be executed on most platforms, this is definitively a
drawback in performance. C++ is very powerful but can also be very dangerous. Forgetting to clear some of the objects will trash your RAM over time (any maybe leak some important data). Also a good knowledge over the STL is required for efficient programming.

Therefore the I-Programming-Language tries to fix most of these issues with these design principles:

- Easy-to-learn syntax
- OOP
- Optimized compiler
- Fast
- Compatible with many ecosystems and products
- Easy to extend

## 2. FAQ

> Question 1

Answer 1

> Question 2

Answer 2

## 3. Tutorials

So you want to start a project? Lets just get the basics before that:
To compile a single-file program (make it executable), open the command-line in your working directory and type

```bash

$ ilang <filename> [arguments]
Compiling <filename>...

[===================>] 100%
Compiled <filename>.

```

You should end up with an `.exe` (Windows) or plain (Unix) file in the `output` directory.

!!! note
    This will be a debug build, for releasing look further down.

### 3.1 Hello World

Lets start with the standard Hello world example:

```il

output("Hello World!");

```

Thats it! Go ahead and compile it!

#### 3.1.1 How does it work?

`output` will send a print string signal to the standard output (stdout), in this case the command line.

### 3.2 Guess the number

Lets do something more complicated: A guess the number game! This allows us to learn some more complicated things.

Lets start with a greeting, using the function we learned in our last tutorial:

```il

output("Welcome to: Guess the number!");

```

Now we have to generate a random number a store it somewhere to look it up and compare it later. For this we will use a variable. Variables can store all kinds of data, like integers, strings or booleans.

To initialize an empty variable use

```il

int goal;

```

This variable can now hold an integer. However, because you did not specify the value the variable should have, calling it will result in an value error.

Change the line to hold a variable like this:

```il

int goal = 3;

```

You now have a variable containing a `3`. We will add true random numbers later.

To get the guesses we will need to process input. As you are using `output` for printing, the way to get inputs from the command line is `input`.
So add this line:

```il

int guess = int(input("Please input a number to guess: "));

```

Now we can store an input value too! However if we type something else than a number in the field, you will notice that you get something like

```bash

Error: Type error
 --> guess_the_number.il:3:17
  |
3 | int guess = int(input("Please input a number to guess: "));
  |                 ^---------------------------------------^
  |
  = Cannot convert from string "Test" to integer

```

To solve this write this instead:

```il

int guess = 0;
try {
    guess = int(input("Please input a number to guess: ));
}
catch TypeError(value) {
    output("{value} is not a number. Using zero instead.");
}

```

This is a lot of code, I know. So lets walk through each line.
First we initialize `guess` with a value of 0. Then we start a new `try` block. Everything within the braces after the `try` keyword will be executed, and if a `TypeError` is raised (specified behind the `catch`) the code behind catch will be executed. So if you were to pass in  `Test` as the number, it would resort to the fallback value of zero instead.

Now we have to compare the input with the goal. To do that, use the `if` statement.

```il

if (guess == goal){
    output("Perfect! You guessed the number right!");
    return Exit; // This line ends the program with a exit code of zero.
}

```

This compares guess and goal and executes the block of code in the braces if guess is equal to goal (note that we use `==` for equality to distinguish between the equal and assign symbol, which is `=`). `return Exit` makes the program exit with an exit `Perfect! You guessed the number right!` as the output.

Now lets print the higher/lower.

```il

if (guess > goal){
    output("Lower");
}
if (guess < goal){
    output("Higher");
}

```

We are again comparing the two values and conditionally outputting lower or higher, but when compiling you will notice that this code will only be executed once, which means it will exit after printing higher or lower. to solve this, wrap this block of code around the rest (except the greeting):

```il

while(guess != goal){
    ...
}

```

This will ensure that your program will keep running until you guessed right.

The final file looks something like this:

```il

output("Welcome to: Guess the number!");

int goal = 3;
int guess = 0

while(guess != goal){
    try {
        guess = int(input("Please input a number to guess: ));
    }
    catch TypeError(value) {
        output("{value} is not a number. Using zero instead.");
    }

    if (guess == goal){
        output("Perfect! You guessed the number right!");
        return Exit;
    }
    if (guess > goal){
        output("Lower");
    }
    if (guess < goal){
        output("Higher");
    }
}

```

Now you can also move the block that checks if guess equals to goal out of the loop.

```il

output("Welcome to: Guess the number!");

int goal = 3;
int guess = 0

while(guess != goal){
    try {
        guess = int(input("Please input a number to guess: ));
    }
    catch TypeError(value) {
        output("{value} is not a number. Using zero instead.");
    }

    if (guess > goal){
        output("Lower");
    }
    if (guess < goal){
        output("Higher");
    }
}

output("Perfect! You guessed the number right!");

```
