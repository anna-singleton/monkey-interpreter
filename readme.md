# a parser for monkey, written in rust

following [this lovely book by Thorsten Ball](https://interpreterbook.com/)

except im writing it in rust instead of go because i like rust and i dont know
go.

## progress

- [x] 1. Lexing
    - [x] 1.1 Lexical Analysis
    - [x] 1.2 Defining our Tokens
    - [x] 1.3 The Lexer
    - [x] 1.4 Extending our Token Set Lexer
    - [x] 1.5 Start of a REPL
- [ ] 2. Parsing
    - [x] 2.1 Parsers
    - [x] 2.2 Why not a parser generator?
    - [x] 2.3 Writing a parser for the monkey programming language
    - [ ] 2.4 Parser's first steps: parsing let statements
    - [ ] 2.5 Parsing return statements
    - [ ] 2.6 Parsing expressions
        - [ ] Expressions in monkey
        - [ ] Top down operator precedence (Pratt Parsing)
        - [ ] Terminology
        - [ ] Preparing the AST
        - [ ] Implementing the Pratt Parser
        - [ ] Identifiers
        - [ ] Integer Literals
        - [ ] Prefix Operators
        - [ ] Infix Operators
    - [ ] 2.7 How Pratt Parsing works
    - [ ] 2.8 Extending the parser
        - [ ] Boolean literals
        - [ ] Grouped expressions
        - [ ] If expressions
        - [ ] Function literals
        - [ ] Call expressions
        - [ ] Removing TODOs
    - [ ] 2.9 Read-Parse-Print-Loop
- [ ] 3. Evaluation
    - [ ] 3.1 Giving meaning to symbols
    - [ ] 3.2 Strategies of evaluation
    - [ ] 3.3 A tree-walking interpreter
    - [ ] 3.4 Representing Objects
        - [ ] Foundation of our object system
        - [ ] Integers
        - [ ] Booleans
        - [ ] Null
    - [ ] 3.5 Evaluating Expressions
        - [ ] Integer literals
        - [ ] Completing the REPL
        - [ ] Boolean literals
        - [ ] Null
        - [ ] Prefix expressions
        - [ ] Infix expressions
    - [ ] 3.6 Conditionals
    - [ ] 3.7 Return statements
    - [ ] 3.8 Abort! Abort! There's been a mistake! or error handling
    - [ ] 3.9 Bindings & the environment
    - [ ] 3.10 Functions & function calls
    - [ ] 3.11 Who's taking the trash out?
- [ ] 4. Extending the interpreter
    - [ ] 4.1 Data types & functions
    - [ ] 4.2 Strings
        - [ ] Supporting strings in our lexer
        - [ ] Parsing strings
        - [ ] Evaluating strings
        - [ ] String concatenation
    - [ ] 4.3 Built-in functions
        - [ ] len
    - [ ] 4.4 Array
        - [ ] Supporting arrays in our lexer
        - [ ] Parsing array literals
        - [ ] Parsing index operator expressions
        - [ ] Evaluating array literals
        - [ ] Evaluating index operator expressions
        - [ ] Adding built-in functions for arrays
        - [ ] Test-driving arrays
    - [ ] 4.5 Hashes
        - [ ] Lexing hash literals
        - [ ] Parsing hash literals
        - [ ] Hashing objects
        - [ ] Evaluating hash literals
        - [ ] Evaluating index expressions with hashes
    - [ ] 4.6 The grand finale
