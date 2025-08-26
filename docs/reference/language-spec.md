# Nova Language Specification v0.3.0

## Table of Contents

1. [Introduction](#introduction)
2. [Lexical Structure](#lexical-structure)
3. [Data Types](#data-types)
4. [Expressions](#expressions)
5. [Statements](#statements)
6. [Functions](#functions)
7. [Classes](#classes)
8. [Control Flow](#control-flow)
9. [Built-in Functions](#built-in-functions)
10. [Grammar](#grammar)

## Introduction

Nova is a dynamically-typed, interpreted programming language designed for simplicity and expressiveness. It features first-class functions, arrays, objects, object-oriented programming with classes and inheritance, and a clean syntax inspired by modern programming languages.

## Lexical Structure

### Comments
```nova
// This is a line comment
```

### Keywords
```
let fn if else while for in return true false null and or
class extends super this constructor private public static new
```

### Identifiers
Identifiers must start with a letter or underscore, followed by any combination of letters, digits, or underscores.

### Literals

#### Numbers
- Integer: `42`, `-17`
- Float: `3.14`, `-2.5`

#### Strings
- Double-quoted: `"Hello, World!"`
- Escape sequences: `\n`, `\t`, `\\`, `\"`

#### Booleans
- `true`
- `false`

#### Null
- `null`

#### Arrays
- `[1, 2, 3]`
- `["a", "b", "c"]`
- `[1, "hello", true]` (mixed types)

### Operators

#### Arithmetic
- `+` Addition
- `-` Subtraction  
- `*` Multiplication
- `/` Division

#### Comparison
- `==` Equality
- `!=` Inequality
- `<` Less than
- `>` Greater than
- `<=` Less than or equal
- `>=` Greater than or equal

#### Logical
- `and` Logical AND
- `or` Logical OR
- `!` Logical NOT

#### Assignment
- `=` Assignment

### Delimiters
- `(` `)` Parentheses
- `{` `}` Braces
- `[` `]` Brackets
- `,` Comma
- `;` Semicolon

## Data Types

### Number
64-bit floating point numbers.

### String
UTF-8 encoded text strings.

### Boolean
`true` or `false`

### Array
Ordered collection of values of any type.

### Function
First-class function objects.

### Null
Represents absence of value.

## Expressions

### Literals
Direct representation of values.

### Identifiers
Variable and function names.

### Binary Operations

#### Arithmetic Operations
```nova
a + b        // Addition
a - b        // Subtraction  
a * b        // Multiplication
a / b        // Division
a % b        // Modulo
```

#### String Operations (v0.2.0+)
```nova
"Hello" + " " + "World"    // String concatenation: "Hello World"
"Name: " + name            // String + variable: "Name: John"  
"Count: " + 42             // Automatic type conversion: "Count: 42"
"Value: " + true           // Any type converts to string: "Value: true"
```

**Note**: The `+` operator automatically converts non-string operands to strings when used with mixed types.

#### Comparison Operations
```nova
a == b       // Equal
a != b       // Not equal
a < b        // Less than
a > b        // Greater than
a <= b       // Less than or equal
a >= b       // Greater than or equal
```

#### Logical Operations
```nova
a and b      // Logical AND
a or b       // Logical OR
```

### Unary Operations
```nova
-value
!condition
```

### Function Calls
```nova
func()
func(arg1, arg2)
```

### Array Access
```nova
array[index]
```

### Array Literals
```nova
[1, 2, 3]
```

## Statements

### Expression Statement
```nova
expression;
```

### Variable Declaration
```nova
let variable = expression;
```

### Function Declaration
```nova
fn name(param1, param2) {
    body
}
```

### Return Statement
```nova
return;
return expression;
```

## Functions

Functions are first-class values in Nova.

### Declaration
```nova
fn add(a, b) {
    a + b
}
```

### Anonymous Functions (Future)
```nova
let add = fn(a, b) { a + b };
```

### Closures
Functions capture their lexical environment.

## Classes

Nova supports object-oriented programming with classes.

### Class Declaration
```nova
class ClassName {
    constructor(params) {
        // Initialization
    }
    
    fn methodName(params) {
        // Method body
    }
    
    static fn staticMethod() {
        // Static method body
    }
}
```

### Constructor
The `constructor` method is called when creating new instances:
```nova
class Person {
    constructor(name, age) {
        this.name = name;
        this.age = age;
    }
}
```

### Instance Creation
Use the `new` keyword to create instances:
```nova
let person = new Person("Alice", 25);
```

### Method Definition
Instance methods can access `this`:
```nova
class Person {
    constructor(name) {
        this.name = name;
    }
    
    fn greet() {
        return "Hello, I'm " + this.name;
    }
}
```

### Static Methods
Static methods belong to the class, not instances:
```nova
class MathUtils {
    static fn add(a, b) {
        return a + b;
    }
}

let sum = MathUtils.add(5, 3);
```

### Inheritance
Classes can inherit from other classes using `extends`:
```nova
class Animal {
    constructor(name) {
        this.name = name;
    }
    
    fn speak() {
        return this.name + " makes a sound";
    }
}

class Dog extends Animal {
    constructor(name, breed) {
        super(name);  // Call parent constructor
        this.breed = breed;
    }
    
    fn speak() {  // Override parent method
        return this.name + " barks";
    }
}
```

### Visibility Modifiers
- `public` - Accessible from outside the class (default)
- `private` - Only accessible within the class

```nova
class BankAccount {
    constructor(balance) {
        this.balance = balance;
    }
    
    public fn getBalance() {
        return this.balance;
    }
    
    private fn validateAmount(amount) {
        return amount > 0;
    }
}
```

### Super Keyword
Use `super` to access parent class methods:
```nova
class Child extends Parent {
    fn method() {
        let result = super.method();  // Call parent method
        return result + " enhanced";
    }
}
```

## Control Flow

### Conditional
```nova
if (condition) {
    then_body
} else {
    else_body
}
```

### While Loop
```nova
while (condition) {
    body
}
```

### For Loop
```nova
for item in iterable {
    body
}
```

## Built-in Functions

### I/O
- `print(value)` - Print value to console

### Type System
- `type(value)` - Get type name as string
- `str(value)` - Convert to string
- `num(value)` - Convert to number

### Collections
- `len(collection)` - Get length
- `push(array, value)` - Add element (returns new array)
- `pop(array)` - Remove last element

## Grammar

```ebnf
program        := statement* ;
statement      := exprStmt | letStmt | fnStmt | returnStmt ;
exprStmt       := expression ";" ;
letStmt        := "let" IDENTIFIER "=" expression ";" ;
fnStmt         := "fn" IDENTIFIER "(" parameters? ")" expression ;
returnStmt     := "return" expression? ";" ;

expression     := or ;
or             := and ( "or" and )* ;
and            := equality ( "and" equality )* ;
equality       := comparison ( ( "!=" | "==" ) comparison )* ;
comparison     := term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           := factor ( ( "-" | "+" ) factor )* ;
factor         := unary ( ( "/" | "*" ) unary )* ;
unary          := ( "!" | "-" ) unary | call ;
call           := primary ( "(" arguments? ")" | "[" expression "]" )* ;
primary        := "true" | "false" | "null" | NUMBER | STRING | IDENTIFIER
                | "(" expression ")" | "[" arguments? "]"
                | "{" statements "}" | "if" expression expression ( "else" expression )?
                | "while" expression expression
                | "for" IDENTIFIER "in" expression expression ;

parameters     := IDENTIFIER ( "," IDENTIFIER )* ;
arguments      := expression ( "," expression )* ;
statements     := statement* ;
```

## Type System

Nova uses dynamic typing with runtime type checking. Types are determined at runtime and can be queried using the `type()` built-in function.

## Memory Model

Nova uses garbage collection for automatic memory management. Values are reference-counted with cycle detection.

## Error Handling

Runtime errors halt execution with descriptive error messages. Common errors include:
- Type errors (invalid operations)
- Index out of bounds
- Undefined variables
- Division by zero

## Standard Library (Future)

Future versions will include:
- Math functions
- String manipulation
- File I/O
- Regular expressions
- JSON parsing
- HTTP client