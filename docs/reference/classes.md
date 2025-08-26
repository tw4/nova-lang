# Class System - Object-Oriented Programming in Nova

Nova supports a comprehensive class system that enables object-oriented programming with features like inheritance, encapsulation, and polymorphism.

## Table of Contents

- [Class Declaration](#class-declaration)
- [Constructors](#constructors)
- [Methods](#methods)
- [Static Methods](#static-methods)
- [Visibility Modifiers](#visibility-modifiers)
- [Inheritance](#inheritance)
- [Instance Creation](#instance-creation)
- [Property Access](#property-access)
- [Method Overriding](#method-overriding)
- [Best Practices](#best-practices)

## Class Declaration

Classes are declared using the `class` keyword followed by the class name and body:

```nova
class ClassName {
    // Constructor and methods go here
}
```

### Example

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

## Constructors

Constructors are special methods that initialize new instances of a class. They are declared using the `constructor` keyword:

```nova
class Person {
    constructor(name, age) {
        this.name = name;
        this.age = age;
    }
}
```

### Key Points

- Constructors are called automatically when creating instances with `new`
- Constructors can take parameters
- Use `this` to refer to the instance being created
- Each class can have only one constructor

## Methods

Methods are functions that belong to a class and operate on instances:

```nova
class Calculator {
    constructor(initial) {
        this.value = initial;
    }
    
    fn add(x) {
        this.value = this.value + x;
        return this;
    }
    
    fn multiply(x) {
        this.value = this.value * x;
        return this;
    }
    
    fn getValue() {
        return this.value;
    }
}
```

### Method Features

- Methods can access instance properties using `this`
- Methods can return values
- Methods can return `this` for method chaining
- Methods are called on instances using dot notation

## Static Methods

Static methods belong to the class itself rather than instances. They're declared with the `static` keyword:

```nova
class MathUtils {
    static fn add(a, b) {
        return a + b;
    }
    
    static fn multiply(a, b) {
        return a * b;
    }
    
    static fn pi() {
        return 3.14159;
    }
}

// Usage
let sum = MathUtils.add(5, 3);        // 8
let product = MathUtils.multiply(4, 7); // 28
let pi = MathUtils.pi();               // 3.14159
```

### Static Method Features

- Called on the class, not on instances
- Cannot access instance properties (`this` is not available)
- Useful for utility functions and factory methods

## Visibility Modifiers

Nova supports visibility modifiers to control access to methods:

```nova
class BankAccount {
    constructor(balance) {
        this.balance = balance;
    }
    
    public fn deposit(amount) {
        if (amount > 0) {
            this.balance = this.balance + amount;
            return "Deposited " + str(amount);
        }
        return "Invalid amount";
    }
    
    public fn getBalance() {
        return this.balance;
    }
    
    private fn validateOperation(amount) {
        return amount > 0 and amount <= this.balance;
    }
}
```

### Visibility Types

- **`public`**: Accessible from outside the class (default)
- **`private`**: Only accessible within the class

## Inheritance

Classes can inherit from other classes using the `extends` keyword:

```nova
class Animal {
    constructor(name) {
        this.name = name;
    }
    
    fn speak() {
        return this.name + " makes a sound";
    }
    
    fn move() {
        return this.name + " moves";
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
    
    fn wagTail() {  // New method specific to Dog
        return this.name + " wags tail";
    }
}
```

### Inheritance Features

- Use `extends` to inherit from a parent class
- Use `super()` in constructor to call parent constructor
- Child classes inherit all methods from parent
- Methods can be overridden in child classes
- Use `super.methodName()` to call parent methods

## Instance Creation

Create instances of classes using the `new` keyword:

```nova
class Person {
    constructor(name, age) {
        this.name = name;
        this.age = age;
    }
}

// Create instances
let alice = new Person("Alice", 25);
let bob = new Person("Bob", 30);
```

### Instance Creation Process

1. `new` creates a new instance
2. Constructor is called with provided arguments
3. Constructor initializes the instance
4. Instance is returned

## Property Access

Access instance properties using dot notation:

```nova
class Person {
    constructor(name, age) {
        this.name = name;
        this.age = age;
    }
    
    fn birthday() {
        this.age = this.age + 1;
    }
}

let person = new Person("Alice", 25);

// Read properties
print(person.name); // "Alice"
print(person.age);  // 25

// Modify through methods
person.birthday();
print(person.age);  // 26
```

## Method Overriding

Child classes can override methods from parent classes:

```nova
class Shape {
    constructor(name) {
        this.name = name;
    }
    
    fn describe() {
        return "This is a " + this.name;
    }
    
    fn area() {
        return "Area calculation not implemented";
    }
}

class Circle extends Shape {
    constructor(radius) {
        super("circle");
        this.radius = radius;
    }
    
    fn area() {  // Override parent method
        return 3.14159 * this.radius * this.radius;
    }
    
    fn describe() {  // Override with enhanced functionality
        let parentDesc = super.describe();  // Call parent method
        return parentDesc + " with radius " + str(this.radius);
    }
}

let circle = new Circle(5);
print(circle.describe()); // "This is a circle with radius 5"
print(circle.area());     // 78.53975
```

## Advanced Examples

### Method Chaining

```nova
class StringBuilder {
    constructor() {
        this.parts = [];
    }
    
    fn append(text) {
        push(this.parts, text);
        return this;  // Return this for chaining
    }
    
    fn appendLine(text) {
        push(this.parts, text);
        push(this.parts, "\n");
        return this;
    }
    
    fn build() {
        return join(this.parts, "");
    }
}

let result = new StringBuilder()
    .append("Hello ")
    .append("World")
    .appendLine("!")
    .append("Nova is great")
    .build();

print(result); // "Hello World!\nNova is great"
```

### Factory Pattern

```nova
class Vehicle {
    constructor(type, brand) {
        this.type = type;
        this.brand = brand;
    }
    
    fn describe() {
        return this.brand + " " + this.type;
    }
    
    static fn createCar(brand) {
        return new Vehicle("car", brand);
    }
    
    static fn createTruck(brand) {
        return new Vehicle("truck", brand);
    }
}

let car = Vehicle.createCar("Toyota");
let truck = Vehicle.createTruck("Ford");

print(car.describe());   // "Toyota car"
print(truck.describe()); // "Ford truck"
```

## Best Practices

### 1. Use Descriptive Class Names

```nova
// Good
class BankAccount { ... }
class EmailValidator { ... }
class FileManager { ... }

// Avoid
class Thing { ... }
class Helper { ... }
class Util { ... }
```

### 2. Initialize Properties in Constructor

```nova
class Person {
    constructor(name, age) {
        // Initialize all properties
        this.name = name;
        this.age = age;
        this.isActive = true;
    }
}
```

### 3. Use Static Methods for Utilities

```nova
class StringUtils {
    static fn capitalize(str) {
        return str[0].toUpperCase() + str.substring(1);
    }
    
    static fn reverse(str) {
        // Implementation
    }
}
```

### 4. Implement Method Chaining When Appropriate

```nova
class QueryBuilder {
    constructor() {
        this.conditions = [];
    }
    
    fn where(condition) {
        push(this.conditions, condition);
        return this;
    }
    
    fn orderBy(field) {
        this.orderField = field;
        return this;
    }
    
    fn execute() {
        // Execute query logic
        return "Query result";
    }
}

let result = new QueryBuilder()
    .where("age > 18")
    .where("status = 'active'")
    .orderBy("name")
    .execute();
```

### 5. Use Inheritance Judiciously

```nova
// Good: Clear is-a relationship
class Animal { ... }
class Dog extends Animal { ... }

// Questionable: Favor composition
class Engine { ... }
class Car extends Engine { ... }  // Car is-a Engine?

// Better: Use composition
class Car {
    constructor() {
        this.engine = new Engine();
    }
}
```

## Common Patterns

### Singleton Pattern

```nova
class Database {
    constructor() {
        if (Database.instance) {
            return Database.instance;
        }
        
        this.connections = [];
        Database.instance = this;
    }
    
    fn connect() {
        // Connection logic
        return "Connected to database";
    }
    
    static fn getInstance() {
        return new Database();
    }
}

let db1 = Database.getInstance();
let db2 = Database.getInstance();
// db1 and db2 are the same instance
```

### Observer Pattern

```nova
class EventEmitter {
    constructor() {
        this.listeners = [];
    }
    
    fn on(event, callback) {
        push(this.listeners, {event: event, callback: callback});
    }
    
    fn emit(event, data) {
        for listener in this.listeners {
            if (listener.event == event) {
                listener.callback(data);
            }
        }
    }
}

class Button extends EventEmitter {
    constructor(label) {
        super();
        this.label = label;
    }
    
    fn click() {
        print("Button " + this.label + " clicked");
        this.emit("click", this.label);
    }
}
```

## Error Handling with Classes

```nova
class ValidationError {
    constructor(message, field) {
        this.message = message;
        this.field = field;
        this.type = "ValidationError";
    }
    
    fn toString() {
        return this.type + ": " + this.message + " (field: " + this.field + ")";
    }
}

class Validator {
    static fn validateEmail(email) {
        if (len(email) == 0) {
            throw new ValidationError("Email cannot be empty", "email");
        }
        // Additional validation logic
        return true;
    }
}
```

This comprehensive class system makes Nova suitable for complex object-oriented applications while maintaining the language's simplicity and readability.