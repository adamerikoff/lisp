# 🚀 lisp: A Simple Lisp Interpreter in Rust

lisp is a minimalistic Lisp-like interpreter built from scratch in Rust.
-----

## ✨ Features

  * **REPL Mode**: Interact with the interpreter directly from your terminal.
  * **File Execution**: Run Lisp code from `.example` files.
  * **Basic Data Types**: Numbers (f64), Strings, Booleans (`true`/`false`), and `nil`.
  * **Arithmetic Operations**: `+`, `-`, `*`, `/`.
  * **Comparison Operations**: `=`, `!=`, `>`, `<`, `>=`, `<=`.
  * **`print` function**: Output values to the console.
  * **`let` special form**: Define and bind variables in the current scope.
  * **`if` special form**: Conditional execution.
  * **`lambda` special form**: Define anonymous functions (closures) with lexical scoping.
  * **Lexical Scoping**: Functions retain access to the environment where they were defined.

-----

## 🏗️ Project Structure

The project is organized into modular components, reflecting the typical stages of an interpreter's pipeline.

```
.
├── Cargo.toml               # Rust project manifest
└── src                      # Source code
    ├── ast                  # Abstract Syntax Tree definitions
    │   ├── ast.rs           # Defines the `Expression` enum
    │   └── mod.rs
    ├── evaluator            # Responsible for executing the AST
    │   ├── builtins.rs      # Implementations of built-in functions
    │   ├── environment.rs   # Handles variable scoping and binding
    │   ├── evaluator.rs     # The core evaluation logic
    │   ├── mod.rs
    │   └── value.rs         # Defines `Value` and `Callable` enums
    ├── lib.rs               # Library root (makes modules accessible)
    ├── main.rs              # Entry point for REPL/file execution
    ├── parser               # Converts tokens into an AST
    │   ├── mod.rs
    │   └── parser.rs
    └── tokenizer            # Converts source code into tokens
        ├── mod.rs
        ├── token.rs         # Defines the `Token` enum
        └── tokenizer.rs
```

-----

## 🧠 How lisp Works: The Interpreter Pipeline

A language interpreter typically follows a multi-stage process to understand and execute code. lisp implements these stages sequentially:

1.  **Tokenizer (Lexer)** ➡️ 2.  **Parser** ➡️ 3.  **Evaluator**

### 1\. Tokenization (Lexical Analysis)

The first step is to transform the raw source code (a string of characters) into a sequence of meaningful units called **tokens**. This process ignores whitespace and comments, identifying keywords, identifiers, numbers, strings, and operators.

**Example Input:** `(+ 10 "hello")`

**Output Tokens:**

```rust
[
    Token::LeftParen,
    Token::Identifier("+"),
    Token::Number(10.0),
    Token::String("hello"),
    Token::RightParen,
    Token::Eof
]
```

**Files:** `src/tokenizer/`

  * `token.rs`: Defines the `Token` enum, representing the types of tokens lisp understands.
  * `tokenizer.rs`: Contains the `Tokenizer` struct, which reads the input character by character and produces a stream of tokens. It also handles basic lexical errors like unexpected characters or unterminated strings.

-----

### 2\. Parsing (Syntax Analysis)

Once we have a stream of tokens, the parser's job is to check if the sequence of tokens conforms to the language's grammar rules and, if so, to build an **Abstract Syntax Tree (AST)**. The AST is a tree-like representation of the code's structure, abstracting away the concrete syntax.

**Example Tokens:** `[Token::LeftParen, Token::Identifier("+"), Token::Number(10.0), Token::String("hello"), Token::RightParen, Token::Eof]`

**Output AST:**

```rust
Expression::List([
    Expression::Identifier("+"),
    Expression::Number(10.0),
    Expression::String("hello"),
])
```

**Files:** `src/parser/`

  * `ast/ast.rs`: Defines the `Expression` enum, which represents the nodes in the AST.
  * `parser.rs`: Contains the `Parser` struct, which consumes the token stream and constructs the `Expression` (AST) tree. It's responsible for enforcing the syntax rules (e.g., matching parentheses).

-----

### 3\. Evaluation (Execution)

The final stage is evaluation. The `Evaluator` walks the AST, interpreting each node and producing a **result `Value`**. This is where the actual computation happens, variables are looked up, functions are called, and control flow is managed.

**Input AST:**

```rust
Expression::List([
    Expression::Identifier("+"),
    Expression::Number(10.0),
    Expression::Number(20.0),
])
```

**Output Value:** `Value::Number(30.0)`

#### Key Concepts in Evaluation:

  * **Environment (`Environment` struct)**: This is crucial for **scoping**. It's essentially a `HashMap` that maps variable names (strings) to their corresponding `Value`s.

      * Environments can be **nested** (each having a `parent` environment), forming a chain. When looking up a variable, the evaluator searches the current environment first, then its parent, and so on, until the variable is found or the global environment is exhausted.
      * **Global Environment**: The top-most environment, initialized with built-in functions.
      * 
  * **Values (`Value` enum)**: Represents all possible data types that an expression can evaluate to in lisp (numbers, strings, booleans, `nil`, and functions).

  * **Callables (`Callable` enum)**: Represents anything that can be "called" like a function.

      * **`Builtin`**: Native Rust functions (like `+`, `print`).
      * **`Lambda`**: User-defined functions. These are where **closures** come into play.

#### How Closures Work (`lambda`):

When a `lambda` expression is evaluated, it doesn't immediately run the function body. Instead, it creates a `Value::Function` containing a `Callable::Lambda`. Critically, this `Lambda` **captures a clone of the `Environment` where it was *defined***.

When this `Lambda` is later called:

1.  A **new environment** is created for the function's execution.
2.  This new environment's **parent** is set to the `captured_env` (the environment from definition time), not the environment where the function was called.
3.  The function's arguments are bound to its parameters within this new environment.
4.  The function body is then evaluated within this new, chained environment.

This mechanism ensures **lexical scoping**, meaning a function can access variables from its defining scope, even if it's called from a different part of the program.

**Files:** `src/evaluator/`

  * `value.rs`: Defines `Value` and `Callable` enums.
  * `environment.rs`: Implements the `Environment` struct for managing variable scopes.
  * `builtins.rs`: Provides the Rust implementations for all built-in functions (e.g., `builtin_add`, `builtin_print`).
  * `evaluator.rs`: Contains the `Evaluator` struct and its core `evaluate` method, which recursively traverses the AST and interprets expressions. It also handles special forms and function application.

-----

## 🛠️ Getting Started

### Prerequisites

  * **Rust**: Make sure you have Rust and Cargo installed.

### Build and Run

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/adamerikoff/lisp.git
    cd lisp
    ```
2.  **Build the project:**
    ```bash
    cargo build
    ```

### Usage

lisp supports two modes: REPL and File Execution.

#### 1\. REPL Mode (Interactive)

To start the interactive REPL:

```bash
cargo run
```

You'll see a `>` prompt where you can type Lisp expressions. Type `exit` to quit.

**Example REPL Session:**

```
Lisp REPL (Rust Edition)
Type 'exit' to quit.
> (+ 10 20)
30
> (print "Hello, REPL!")
Hello, REPL!
nil
> (let x 5)
nil
> (* x 2)
10
> (let my-func (lambda (a b) (+ a b)))
#<lambda (a b)>
> (my-func 3 4)
7
> exit
Exiting REPL.
```

#### 2\. File Execution Mode

To run a Lisp program from a file:

```bash
cargo run -- ./filename.ext
```

*(Replace `./filename.ext` with the path to your Lisp file.)*

-----

## 💻 Example Programs

Here are some examples of Lisp code you can run with lisp:

### Basic Arithmetic and Printing

```lisp
; Basic addition
(+ 10 20)

; Inequality check
(!= 10 20)

; Nested arithmetic operations
(- (* 5 8) (/ 10 2))

; Print a string
(print "Hello, world!")

; Print the result of an arithmetic expression
(print (+ 1 2 3))
(print (+ 10 (* 2 5)))
```

### Variable Definition (`let`)

```lisp
; Define and print a number
(let x 10)
(print x)

; Define and print a string
(let message "Rust is fun")
(print message)

; Define and print a boolean
(let is_active true)
(print is_active)
```

### Conditional Logic (`if`)

```lisp
(let number 7)
(if (> number 5)
    (print "Number is greater than 5")
    (print "Number is not greater than 5"))

; Another if example
(if (= 10 10)
    (print "Ten equals ten")
    (print "This won't print"))
```

### Functions (`lambda` for Closures)

```lisp
; Define a simple anonymous function and store it in 'my_func'
; It takes two arguments 'a' and 'b' and returns 'a * (b * b)'
(let my_func (lambda (a b) (* a (* b b))))

; Call the defined function
(print (my_func 2 3)) ; Expected: 2 * (3 * 3) = 18

; A function demonstrating lexical scope (closure)
(let create-adder (lambda (x) (lambda (y) (+ x y))))

; 'add-five' is a closure that "remembers" 'x' as 5
(let add-five (create-adder 5))

; Call the closure
(print (add-five 10)) ; Expected: 15 (5 + 10)
(print (add-five 20)) ; Expected: 25 (5 + 20)

; Another closure
(let add-ten (create-adder 10))
(print (add-ten 3)) ; Expected: 13 (10 + 3)
```

-----

## 🐛 Error Handling

lisp provides clear error messages for various issues:

  * **`TokenizerError`**: Problems during lexical analysis (e.g., `Unexpected character '!' at position 0`).
  * **`ParserError`**: Syntax issues (e.g., `Unmatched parenthesis`).
  * **`EvalError`**: Runtime errors during execution (e.g., `Undefined variable: 'x'`, `Division by zero`, `Wrong number of arguments: + expects at least 1 arguments, but got 0`).

-----