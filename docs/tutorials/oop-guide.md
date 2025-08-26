# Object-Oriented Programming in Nova

This guide introduces you to object-oriented programming (OOP) concepts in Nova, covering classes, inheritance, encapsulation, and practical patterns.

## Table of Contents

1. [Introduction to OOP](#introduction-to-oop)
2. [Your First Class](#your-first-class)
3. [Constructors and Properties](#constructors-and-properties)
4. [Methods](#methods)
5. [Static Methods](#static-methods)
6. [Inheritance](#inheritance)
7. [Encapsulation](#encapsulation)
8. [Polymorphism](#polymorphism)
9. [Common Patterns](#common-patterns)
10. [Best Practices](#best-practices)

## Introduction to OOP

Object-oriented programming is a paradigm that organizes code around objects that contain both data (properties) and code (methods). Nova's class system provides:

- **Encapsulation**: Bundling data and methods together
- **Inheritance**: Creating new classes based on existing ones
- **Polymorphism**: Objects of different types responding to the same interface
- **Abstraction**: Hiding complexity behind simple interfaces

## Your First Class

Let's start with a simple class:

```nova
class Person {
    constructor(name) {
        this.name = name;
    }
    
    fn greet() {
        return "Hello, I'm " + this.name;
    }
}

// Create an instance
let alice = new Person("Alice");
print(alice.greet()); // "Hello, I'm Alice"
```

### Key Concepts

- **`class`**: Keyword to define a class
- **`constructor`**: Special method for initialization
- **`this`**: Reference to the current instance
- **`new`**: Creates a new instance of the class

## Constructors and Properties

Constructors initialize new instances with their initial state:

```nova
class BankAccount {
    constructor(accountNumber, initialBalance) {
        this.accountNumber = accountNumber;
        this.balance = initialBalance;
        this.transactions = [];
        this.isActive = true;
    }
    
    fn getAccountInfo() {
        return {
            number: this.accountNumber,
            balance: this.balance,
            active: this.isActive
        };
    }
}

let account = new BankAccount("12345", 1000);
print(account.balance); // 1000
```

### Constructor Best Practices

1. Initialize all instance properties
2. Validate input parameters
3. Set default values when appropriate

```nova
class Rectangle {
    constructor(width, height) {
        // Validation
        if (width <= 0 or height <= 0) {
            throw "Width and height must be positive";
        }
        
        this.width = width;
        this.height = height;
        this.color = "white"; // Default value
    }
}
```

## Methods

Methods are functions that operate on class instances:

```nova
class Calculator {
    constructor() {
        this.result = 0;
        this.history = [];
    }
    
    fn add(value) {
        this.result = this.result + value;
        push(this.history, "+" + str(value));
        return this; // Enable method chaining
    }
    
    fn multiply(value) {
        this.result = this.result * value;
        push(this.history, "*" + str(value));
        return this;
    }
    
    fn getResult() {
        return this.result;
    }
    
    fn getHistory() {
        return this.history;
    }
    
    fn clear() {
        this.result = 0;
        this.history = [];
        return this;
    }
}

// Usage with method chaining
let calc = new Calculator();
let result = calc.add(10).multiply(2).add(5).getResult();
print(result); // 25
```

## Static Methods

Static methods belong to the class itself, not to instances:

```nova
class MathUtils {
    static fn max(a, b) {
        if (a > b) {
            return a;
        } else {
            return b;
        }
    }
    
    static fn min(a, b) {
        if (a < b) {
            return a;
        } else {
            return b;
        }
    }
    
    static fn average(numbers) {
        let sum = 0;
        for num in numbers {
            sum = sum + num;
        }
        return sum / len(numbers);
    }
}

// Static methods are called on the class, not instances
print(MathUtils.max(10, 5)); // 10
print(MathUtils.average([1, 2, 3, 4, 5])); // 3
```

### When to Use Static Methods

- Utility functions that don't need instance data
- Factory methods for creating instances
- Constants and configuration values

```nova
class User {
    constructor(name, email) {
        this.name = name;
        this.email = email;
        this.id = User.generateId();
    }
    
    static fn generateId() {
        // Simple ID generation (in practice, use a proper UUID library)
        return "user_" + str(Math.random() * 100000);
    }
    
    static fn createAdmin(name, email) {
        let user = new User(name, email);
        user.isAdmin = true;
        return user;
    }
    
    static fn createGuest() {
        return new User("Guest", "guest@example.com");
    }
}

let admin = User.createAdmin("Alice", "alice@company.com");
let guest = User.createGuest();
```

## Inheritance

Inheritance allows you to create new classes based on existing ones:

```nova
// Base class
class Vehicle {
    constructor(brand, model) {
        this.brand = brand;
        this.model = model;
        this.isRunning = false;
    }
    
    fn start() {
        this.isRunning = true;
        return this.brand + " " + this.model + " is starting";
    }
    
    fn stop() {
        this.isRunning = false;
        return this.brand + " " + this.model + " has stopped";
    }
    
    fn getInfo() {
        return this.brand + " " + this.model;
    }
}

// Derived class
class Car extends Vehicle {
    constructor(brand, model, doors) {
        super(brand, model); // Call parent constructor
        this.doors = doors;
        this.type = "car";
    }
    
    fn honk() {
        return "Beep beep! " + this.getInfo();
    }
    
    // Override parent method
    fn getInfo() {
        return super.getInfo() + " (" + str(this.doors) + " doors)";
    }
}

class Motorcycle extends Vehicle {
    constructor(brand, model, engineSize) {
        super(brand, model);
        this.engineSize = engineSize;
        this.type = "motorcycle";
    }
    
    fn wheelie() {
        if (this.isRunning) {
            return this.getInfo() + " does a wheelie!";
        } else {
            return "Start the engine first!";
        }
    }
}

// Usage
let car = new Car("Toyota", "Camry", 4);
let bike = new Motorcycle("Harley", "Sportster", 883);

print(car.start()); // "Toyota Camry is starting"
print(car.honk()); // "Beep beep! Toyota Camry (4 doors)"

print(bike.start()); // "Harley Sportster is starting"
print(bike.wheelie()); // "Harley Sportster does a wheelie!"
```

### Inheritance Key Points

- Use `extends` to inherit from a parent class
- Call `super()` in the constructor to initialize the parent
- Use `super.method()` to call parent methods
- Child classes can override parent methods
- Child classes inherit all public methods from parents

## Encapsulation

Encapsulation hides internal details and provides controlled access to data:

```nova
class BankAccount {
    constructor(accountHolder, initialBalance) {
        this.accountHolder = accountHolder;
        this.balance = initialBalance;
        this.transactions = [];
    }
    
    // Public methods
    public fn deposit(amount) {
        if (this.validateAmount(amount)) {
            this.balance = this.balance + amount;
            this.recordTransaction("deposit", amount);
            return "Deposited " + str(amount) + ". New balance: " + str(this.balance);
        } else {
            return "Invalid deposit amount";
        }
    }
    
    public fn withdraw(amount) {
        if (this.validateAmount(amount) and amount <= this.balance) {
            this.balance = this.balance - amount;
            this.recordTransaction("withdrawal", amount);
            return "Withdrew " + str(amount) + ". New balance: " + str(this.balance);
        } else {
            return "Invalid withdrawal amount or insufficient funds";
        }
    }
    
    public fn getBalance() {
        return this.balance;
    }
    
    public fn getTransactionHistory() {
        return this.transactions;
    }
    
    // Private methods - internal implementation details
    private fn validateAmount(amount) {
        return amount > 0;
    }
    
    private fn recordTransaction(type, amount) {
        let transaction = {
            type: type,
            amount: amount,
            date: "2024-08-26", // In real implementation, use proper date
            balance: this.balance
        };
        push(this.transactions, transaction);
    }
}

let account = new BankAccount("Alice", 1000);
print(account.deposit(500)); // "Deposited 500. New balance: 1500"
print(account.withdraw(200)); // "Withdrew 200. New balance: 1300"

// Private methods are not accessible from outside
// account.validateAmount(100); // This would cause an error
```

## Polymorphism

Polymorphism allows objects of different classes to be used interchangeably:

```nova
// Base class defining a common interface
class Shape {
    constructor(name) {
        this.name = name;
    }
    
    fn area() {
        return "Area calculation not implemented";
    }
    
    fn describe() {
        return "This is a " + this.name + " with area " + str(this.area());
    }
}

class Circle extends Shape {
    constructor(radius) {
        super("circle");
        this.radius = radius;
    }
    
    fn area() {
        return 3.14159 * this.radius * this.radius;
    }
}

class Rectangle extends Shape {
    constructor(width, height) {
        super("rectangle");
        this.width = width;
        this.height = height;
    }
    
    fn area() {
        return this.width * this.height;
    }
}

class Triangle extends Shape {
    constructor(base, height) {
        super("triangle");
        this.base = base;
        this.height = height;
    }
    
    fn area() {
        return 0.5 * this.base * this.height;
    }
}

// Polymorphic usage - same interface, different implementations
let shapes = [
    new Circle(5),
    new Rectangle(10, 20),
    new Triangle(6, 8)
];

for shape in shapes {
    print(shape.describe());
}
// Output:
// This is a circle with area 78.53975
// This is a rectangle with area 200
// This is a triangle with area 24
```

## Common Patterns

### 1. Builder Pattern

```nova
class QueryBuilder {
    constructor() {
        this.selectFields = [];
        this.fromTable = "";
        this.whereConditions = [];
        this.orderByField = "";
    }
    
    fn select(fields) {
        this.selectFields = fields;
        return this;
    }
    
    fn from(table) {
        this.fromTable = table;
        return this;
    }
    
    fn where(condition) {
        push(this.whereConditions, condition);
        return this;
    }
    
    fn orderBy(field) {
        this.orderByField = field;
        return this;
    }
    
    fn build() {
        let query = "SELECT " + join(this.selectFields, ", ");
        query = query + " FROM " + this.fromTable;
        
        if (len(this.whereConditions) > 0) {
            query = query + " WHERE " + join(this.whereConditions, " AND ");
        }
        
        if (this.orderByField != "") {
            query = query + " ORDER BY " + this.orderByField;
        }
        
        return query;
    }
}

let query = new QueryBuilder()
    .select(["name", "email", "age"])
    .from("users")
    .where("age > 18")
    .where("active = true")
    .orderBy("name")
    .build();

print(query);
// SELECT name, email, age FROM users WHERE age > 18 AND active = true ORDER BY name
```

### 2. Observer Pattern

```nova
class EventEmitter {
    constructor() {
        this.listeners = [];
    }
    
    fn on(event, callback) {
        push(this.listeners, {event: event, callback: callback});
        return this;
    }
    
    fn emit(event, data) {
        for listener in this.listeners {
            if (listener.event == event) {
                listener.callback(data);
            }
        }
    }
    
    fn off(event) {
        let newListeners = [];
        for listener in this.listeners {
            if (listener.event != event) {
                push(newListeners, listener);
            }
        }
        this.listeners = newListeners;
    }
}

class Button extends EventEmitter {
    constructor(label) {
        super();
        this.label = label;
        this.clickCount = 0;
    }
    
    fn click() {
        this.clickCount = this.clickCount + 1;
        this.emit("click", {
            label: this.label,
            count: this.clickCount
        });
    }
}

// Usage
let button = new Button("Submit");

button.on("click", fn(data) {
    print("Button clicked: " + data.label + " (count: " + str(data.count) + ")");
});

button.click(); // Button clicked: Submit (count: 1)
button.click(); // Button clicked: Submit (count: 2)
```

### 3. Strategy Pattern

```nova
class PaymentProcessor {
    constructor(strategy) {
        this.strategy = strategy;
    }
    
    fn setStrategy(strategy) {
        this.strategy = strategy;
    }
    
    fn processPayment(amount) {
        return this.strategy.pay(amount);
    }
}

class CreditCardPayment {
    constructor(cardNumber) {
        this.cardNumber = cardNumber;
    }
    
    fn pay(amount) {
        return "Charged $" + str(amount) + " to credit card " + this.cardNumber;
    }
}

class PayPalPayment {
    constructor(email) {
        this.email = email;
    }
    
    fn pay(amount) {
        return "Sent $" + str(amount) + " via PayPal to " + this.email;
    }
}

class BitcoinPayment {
    constructor(walletAddress) {
        this.walletAddress = walletAddress;
    }
    
    fn pay(amount) {
        return "Sent $" + str(amount) + " in Bitcoin to " + this.walletAddress;
    }
}

// Usage
let processor = new PaymentProcessor(new CreditCardPayment("1234-5678"));
print(processor.processPayment(100)); // Charged $100 to credit card 1234-5678

processor.setStrategy(new PayPalPayment("user@example.com"));
print(processor.processPayment(50)); // Sent $50 via PayPal to user@example.com
```

## Best Practices

### 1. Single Responsibility Principle

Each class should have one reason to change:

```nova
// Good - Each class has a single responsibility
class User {
    constructor(name, email) {
        this.name = name;
        this.email = email;
    }
    
    fn getName() {
        return this.name;
    }
    
    fn getEmail() {
        return this.email;
    }
}

class UserValidator {
    static fn validateEmail(email) {
        // Email validation logic
        return len(email) > 0 and email.contains("@");
    }
    
    static fn validateName(name) {
        // Name validation logic
        return len(name) > 0;
    }
}

class UserRepository {
    constructor() {
        this.users = [];
    }
    
    fn save(user) {
        push(this.users, user);
    }
    
    fn findByEmail(email) {
        for user in this.users {
            if (user.getEmail() == email) {
                return user;
            }
        }
        return null;
    }
}
```

### 2. Favor Composition over Inheritance

```nova
// Instead of deep inheritance hierarchies, use composition
class Engine {
    constructor(horsepower) {
        this.horsepower = horsepower;
    }
    
    fn start() {
        return "Engine started (" + str(this.horsepower) + " HP)";
    }
}

class GPS {
    fn navigate(destination) {
        return "Navigating to " + destination;
    }
}

class Car {
    constructor(brand, engine, hasGPS) {
        this.brand = brand;
        this.engine = engine; // Composition
        if (hasGPS) {
            this.gps = new GPS(); // Composition
        }
    }
    
    fn start() {
        return this.brand + ": " + this.engine.start();
    }
    
    fn navigate(destination) {
        if (this.gps) {
            return this.gps.navigate(destination);
        } else {
            return "No GPS available";
        }
    }
}
```

### 3. Use Meaningful Names

```nova
// Good naming
class CustomerOrderProcessor {
    fn processOrder(order) { ... }
    fn calculateTotalAmount(order) { ... }
    fn sendConfirmationEmail(customer, order) { ... }
}

// Avoid generic names
class Manager {
    fn process(data) { ... }
    fn handle(item) { ... }
}
```

### 4. Keep Methods Small and Focused

```nova
class OrderProcessor {
    fn processOrder(order) {
        if (this.validateOrder(order)) {
            this.calculatePricing(order);
            this.updateInventory(order);
            this.sendNotifications(order);
            return "Order processed successfully";
        } else {
            return "Invalid order";
        }
    }
    
    private fn validateOrder(order) {
        // Validation logic
    }
    
    private fn calculatePricing(order) {
        // Pricing logic
    }
    
    private fn updateInventory(order) {
        // Inventory logic
    }
    
    private fn sendNotifications(order) {
        // Notification logic
    }
}
```

### 5. Use Constructor Validation

```nova
class Rectangle {
    constructor(width, height) {
        if (width <= 0) {
            throw "Width must be positive";
        }
        if (height <= 0) {
            throw "Height must be positive";
        }
        
        this.width = width;
        this.height = height;
    }
}
```

This comprehensive guide covers the essentials of object-oriented programming in Nova. With these concepts and patterns, you can build robust, maintainable applications using Nova's class system.