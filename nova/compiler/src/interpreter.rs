use crate::ast::*;
use crate::value::{Value, Environment};
use std::collections::HashMap;

#[derive(Debug)]
pub enum RuntimeError {
    UndefinedVariable(String),
    TypeError(String),
    DivisionByZero,
    InvalidOperation(String),
    ReturnValue(Value),
    UserThrown(String),
    RuntimeError(String),
    Break,
    Continue,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::UndefinedVariable(name) => write!(f, "Undefined variable: '{}'", name),
            RuntimeError::TypeError(msg) => write!(f, "Type error: {}", msg),
            RuntimeError::DivisionByZero => write!(f, "Division by zero"),
            RuntimeError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            RuntimeError::ReturnValue(_) => write!(f, "Return statement outside function"),
            RuntimeError::UserThrown(msg) => write!(f, "Thrown error: {}", msg),
            RuntimeError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            RuntimeError::Break => write!(f, "Break statement outside loop"),
            RuntimeError::Continue => write!(f, "Continue statement outside loop"),
        }
    }
}

type RuntimeResult<T> = Result<T, RuntimeError>;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.define_natives();
        
        Interpreter { environment: env }
    }

    pub fn interpret(&mut self, program: &Program) -> RuntimeResult<Value> {
        let mut last_value = Value::Null;
        
        for statement in &program.statements {
            match self.execute_statement(statement)? {
                Some(value) => last_value = value,
                None => {}
            }
        }
        
        Ok(last_value)
    }

    fn execute_statement(&mut self, stmt: &Stmt) -> RuntimeResult<Option<Value>> {
        match stmt {
            Stmt::Expression(expr) => {
                let value = self.evaluate(expr)?;
                Ok(Some(value))
            }
            Stmt::Let { name, value } => {
                let val = self.evaluate(value)?;
                self.environment.define(name.clone(), val);
                Ok(None)
            }
            Stmt::Function { name, params, body } => {
                let func = Value::Function {
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.environment.clone(),
                };
                self.environment.define(name.clone(), func);
                Ok(None)
            }
            Stmt::AsyncFunction { name, params, body } => {
                let func = Value::AsyncFunction {
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.environment.clone(),
                };
                self.environment.define(name.clone(), func);
                Ok(None)
            }
            Stmt::Return(expr) => {
                let value = match expr {
                    Some(e) => self.evaluate(e)?,
                    None => Value::Null,
                };
                Err(RuntimeError::ReturnValue(value))
            }
            Stmt::Import { module, imports, alias } => {
                // TODO: Implement proper import handling with imports specification
                self.import_module(module, alias.as_ref())
            }
            Stmt::Class { name, superclass, methods, constructor } => {
                self.define_class(name, superclass.as_ref(), methods, constructor.as_ref())
            }
            Stmt::Break => {
                Err(RuntimeError::Break)
            }
            Stmt::Continue => {
                Err(RuntimeError::Continue)
            }
            Stmt::Export { name, value } => {
                // TODO: Implement export handling
                let val = self.evaluate(value)?;
                self.environment.define(name.clone(), val);
                Ok(None)
            }
            Stmt::ExportDefault(expr) => {
                // TODO: Implement default export handling
                let val = self.evaluate(expr)?;
                self.environment.define("default".to_string(), val);
                Ok(None)
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> RuntimeResult<Value> {
        match expr {
            Expr::Literal(lit) => Ok(self.evaluate_literal(lit)),
            Expr::Identifier(name) => {
                self.environment
                    .get(name)
                    .ok_or_else(|| RuntimeError::UndefinedVariable(name.clone()))
            }
            Expr::Binary { left, operator, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.evaluate_binary(operator, &left_val, &right_val)
            }
            Expr::Unary { operator, operand } => {
                let operand_val = self.evaluate(operand)?;
                self.evaluate_unary(operator, &operand_val)
            }
            Expr::Call { callee, args } => {
                // Check if this is a method call (object.method())
                if let Expr::Property { object, property } = callee.as_ref() {
                    let obj_val = self.evaluate(object)?;
                    let arg_values: Result<Vec<_>, _> = args.iter()
                        .map(|arg| self.evaluate(arg))
                        .collect();
                    let arg_values = arg_values?;
                    
                    self.call_method(&obj_val, property, &arg_values)
                } else {
                    let func = self.evaluate(callee)?;
                    let arg_values: Result<Vec<_>, _> = args.iter()
                        .map(|arg| self.evaluate(arg))
                        .collect();
                    let arg_values = arg_values?;
                    
                    self.call_function(&func, &arg_values)
                }
            }
            Expr::New { class, args } => {
                let class_value = self.evaluate(class)?;
                let arg_values: Result<Vec<_>, _> = args.iter()
                    .map(|arg| self.evaluate(arg))
                    .collect();
                let arg_values = arg_values?;
                
                self.instantiate_class(&class_value, &arg_values)
            }
            Expr::Block(statements) => {
                // No separate scope for blocks - they share the same environment
                // This allows assignments in blocks to affect the outer scope
                let mut result = Value::Null;
                for statement in statements {
                    match self.execute_statement(statement) {
                        Ok(Some(value)) => result = value,
                        Ok(None) => {}
                        Err(RuntimeError::ReturnValue(value)) => {
                            return Ok(value);
                        }
                        Err(RuntimeError::Break) => return Err(RuntimeError::Break),
                        Err(RuntimeError::Continue) => return Err(RuntimeError::Continue),
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                
                Ok(result)
            }
            Expr::If { condition, then_branch, else_branch } => {
                let condition_val = self.evaluate(condition)?;
                
                if condition_val.is_truthy() {
                    self.evaluate(then_branch)
                } else if let Some(else_expr) = else_branch {
                    self.evaluate(else_expr)
                } else {
                    Ok(Value::Null)
                }
            }
            Expr::Array(elements) => {
                let values: Result<Vec<_>, _> = elements.iter()
                    .map(|elem| self.evaluate(elem))
                    .collect();
                Ok(Value::Array(values?))
            }
            Expr::Index { object, index } => {
                let obj_val = self.evaluate(object)?;
                let index_val = self.evaluate(index)?;
                
                match (obj_val, index_val) {
                    (Value::Array(arr), Value::Number(idx)) => {
                        let idx = idx as usize;
                        if idx < arr.len() {
                            Ok(arr[idx].clone())
                        } else {
                            Err(RuntimeError::InvalidOperation("Array index out of bounds".to_string()))
                        }
                    }
                    (Value::Object(obj), Value::String(key)) => {
                        Ok(obj.get(&key).cloned().unwrap_or(Value::Null))
                    }
                    (Value::String(s), Value::Number(idx)) => {
                        let idx = idx as usize;
                        let chars: Vec<char> = s.chars().collect();
                        if idx < chars.len() {
                            Ok(Value::String(chars[idx].to_string()))
                        } else {
                            Err(RuntimeError::InvalidOperation("String index out of bounds".to_string()))
                        }
                    }
                    _ => Err(RuntimeError::TypeError("Invalid indexing operation".to_string()))
                }
            }
            Expr::While { condition, body } => {
                let mut result = Value::Null;
                
                loop {
                    let condition_val = self.evaluate(condition)?;
                    if !condition_val.is_truthy() {
                        break;
                    }
                    
                    match self.evaluate(body) {
                        Ok(value) => result = value,
                        Err(RuntimeError::Break) => break,
                        Err(RuntimeError::Continue) => continue,
                        Err(e) => return Err(e),
                    }
                }
                
                Ok(result)
            }
            Expr::For { variable, iterable, body } => {
                let iterable_val = self.evaluate(iterable)?;
                let mut result = Value::Null;
                
                match iterable_val {
                    Value::Array(arr) => {
                        // Store previous value of loop variable if it exists
                        let previous_var = self.environment.get(variable);
                        
                        for item in arr {
                            // Just define the loop variable, don't create new scope
                            self.environment.define(variable.clone(), item);
                            match self.evaluate(body) {
                                Ok(value) => result = value,
                                Err(RuntimeError::Break) => break,
                                Err(RuntimeError::Continue) => continue,
                                Err(e) => return Err(e),
                            }
                        }
                        
                        // Restore or remove loop variable
                        match previous_var {
                            Some(val) => self.environment.define(variable.clone(), val),
                            None => {
                                // Variable didn't exist before loop, we should ideally remove it
                                // but our current Environment doesn't have a remove method
                                // This is a minor issue - variable remains defined
                            }
                        }
                    }
                    Value::String(s) => {
                        let previous_var = self.environment.get(variable);
                        
                        for ch in s.chars() {
                            self.environment.define(variable.clone(), Value::String(ch.to_string()));
                            match self.evaluate(body) {
                                Ok(value) => result = value,
                                Err(RuntimeError::Break) => break,
                                Err(RuntimeError::Continue) => continue,
                                Err(e) => return Err(e),
                            }
                        }
                        
                        match previous_var {
                            Some(val) => self.environment.define(variable.clone(), val),
                            None => {}
                        }
                    }
                    _ => return Err(RuntimeError::TypeError("Can only iterate over arrays and strings".to_string()))
                }
                
                Ok(result)
            }
            Expr::Object(pairs) => {
                let mut object = std::collections::HashMap::new();
                for (key, value_expr) in pairs {
                    let value = self.evaluate(value_expr)?;
                    object.insert(key.clone(), value);
                }
                Ok(Value::Object(object))
            }
            Expr::Property { object, property } => {
                let obj_val = self.evaluate(object)?;
                match obj_val {
                    Value::Object(ref obj) => {
                        Ok(obj.get(property).cloned().unwrap_or(Value::Null))
                    }
                    Value::Instance { ref fields, ref class } => {
                        // First check instance fields
                        if let Some(value) = fields.get(property) {
                            Ok(value.clone())
                        } else {
                            // Then check methods from class
                            if let Value::Class { methods, .. } = class.as_ref() {
                                Ok(methods.get(property).cloned().unwrap_or(Value::Null))
                            } else {
                                Ok(Value::Null)
                            }
                        }
                    }
                    _ => Err(RuntimeError::TypeError(format!("Cannot access property '{}' on {}", property, obj_val.type_name())))
                }
            }
            Expr::Assignment { target, value } => {
                let val = self.evaluate(value)?;
                
                match target.as_ref() {
                    Expr::Identifier(name) => {
                        self.environment.set(name, val.clone())
                            .map_err(|e| RuntimeError::UndefinedVariable(e))?;
                        Ok(val)
                    }
                    Expr::Index { object: _, index: _ } => {
                        // TODO: Implement array/object assignment by index
                        Err(RuntimeError::InvalidOperation("Index assignment not yet implemented".to_string()))
                    }
                    Expr::Property { object, property } => {
                        // For now, only support 'this.property = value' in constructors/methods
                        if let Expr::This = object.as_ref() {
                            if let Some(mut this_value) = self.environment.get("this") {
                                match &mut this_value {
                                    Value::Instance { fields, .. } => {
                                        fields.insert(property.clone(), val.clone());
                                        // Use define instead of set to ensure it works
                                        self.environment.define("this".to_string(), this_value);
                                        Ok(val)
                                    }
                                    _ => Err(RuntimeError::TypeError("'this' is not an instance".to_string()))
                                }
                            } else {
                                Err(RuntimeError::InvalidOperation("'this' used outside class method".to_string()))
                            }
                        } else {
                            Err(RuntimeError::InvalidOperation("Property assignment only supported on 'this' for now".to_string()))
                        }
                    }
                    _ => Err(RuntimeError::InvalidOperation("Invalid assignment target".to_string()))
                }
            }
            Expr::StringInterpolation(parts) => {
                let mut result = String::new();
                for part in parts {
                    let value = self.evaluate(part)?;
                    result.push_str(&value.to_string());
                }
                Ok(Value::String(result))
            }
            Expr::Try { body, catch, finally } => {
                let try_result = self.evaluate(body);
                
                let catch_result = match (try_result, catch) {
                    (Ok(value), _) => Ok(value),
                    (Err(error), Some((catch_var, catch_block))) => {
                        // Create error object with message
                        let error_value = Value::Object({
                            let mut map = std::collections::HashMap::new();
                            map.insert("message".to_string(), Value::String(format!("{:?}", error)));
                            map.insert("type".to_string(), Value::String("RuntimeError".to_string()));
                            map
                        });
                        
                        // Store previous value of catch variable if it exists
                        let previous_var = self.environment.get(catch_var);
                        
                        // Define catch variable with error object
                        self.environment.define(catch_var.clone(), error_value);
                        
                        // Execute catch block
                        let result = self.evaluate(catch_block);
                        
                        // Restore previous value of catch variable
                        match previous_var {
                            Some(val) => self.environment.define(catch_var.clone(), val),
                            None => {} // Variable didn't exist before, leave it defined
                        }
                        
                        result
                    }
                    (Err(error), None) => Err(error), // Re-throw if no catch block
                };
                
                // Execute finally block if present (ignores result)
                if let Some(finally_block) = finally {
                    let _ = self.evaluate(finally_block);
                }
                
                catch_result
            }
            Expr::Throw(expr) => {
                let error_value = self.evaluate(expr)?;
                let error_message = match error_value {
                    Value::String(msg) => msg,
                    _ => error_value.to_string(),
                };
                Err(RuntimeError::UserThrown(error_message))
            }
            Expr::Lambda { params, body } => {
                Ok(Value::Function {
                    params: params.clone(),
                    body: *body.clone(),
                    closure: self.environment.clone(),
                })
            }
            Expr::This => {
                self.environment
                    .get("this")
                    .ok_or_else(|| RuntimeError::InvalidOperation("'this' used outside class method".to_string()))
            }
            Expr::Super => {
                self.environment
                    .get("super")
                    .ok_or_else(|| RuntimeError::InvalidOperation("'super' used outside derived class method".to_string()))
            }
            Expr::TemplateString(parts) => {
                self.evaluate_template_string(parts)
            }
            Expr::Await(promise_expr) => {
                self.evaluate_await(promise_expr)
            }
        }
    }

    fn evaluate_literal(&self, lit: &Literal) -> Value {
        match lit {
            Literal::Number(n) => Value::Number(*n),
            Literal::String(s) => Value::String(s.clone()),
            Literal::Boolean(b) => Value::Boolean(*b),
            Literal::Array(arr) => {
                let values: Vec<Value> = arr.iter().map(|lit| self.evaluate_literal(lit)).collect();
                Value::Array(values)
            },
            Literal::Object(obj) => {
                // TODO: Implement proper object/map support
                let _obj = obj; // Consume parameter to avoid unused warning
                Value::Null // For now, return null
            }
            Literal::Null => Value::Null,
        }
    }

    fn evaluate_binary(&self, op: &BinaryOp, left: &Value, right: &Value) -> RuntimeResult<Value> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::Number(l + r)),
                    BinaryOp::Subtract => Ok(Value::Number(l - r)),
                    BinaryOp::Multiply => Ok(Value::Number(l * r)),
                    BinaryOp::Divide => {
                        if *r == 0.0 {
                            Err(RuntimeError::DivisionByZero)
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                    BinaryOp::Modulo => {
                        if *r == 0.0 {
                            Err(RuntimeError::DivisionByZero)
                        } else {
                            Ok(Value::Number(l % r))
                        }
                    }
                    BinaryOp::Power => Ok(Value::Number(l.powf(*r))),
                    BinaryOp::Equal => Ok(Value::Boolean(l == r)),
                    BinaryOp::NotEqual => Ok(Value::Boolean(l != r)),
                    BinaryOp::Less => Ok(Value::Boolean(l < r)),
                    BinaryOp::Greater => Ok(Value::Boolean(l > r)),
                    BinaryOp::LessEqual => Ok(Value::Boolean(l <= r)),
                    BinaryOp::GreaterEqual => Ok(Value::Boolean(l >= r)),
                    BinaryOp::BitwiseAnd => {
                        let l_int = *l as i64;
                        let r_int = *r as i64;
                        Ok(Value::Number((l_int & r_int) as f64))
                    }
                    BinaryOp::BitwiseOr => {
                        let l_int = *l as i64;
                        let r_int = *r as i64;
                        Ok(Value::Number((l_int | r_int) as f64))
                    }
                    BinaryOp::BitwiseXor => {
                        let l_int = *l as i64;
                        let r_int = *r as i64;
                        Ok(Value::Number((l_int ^ r_int) as f64))
                    }
                    BinaryOp::LeftShift => {
                        let l_int = *l as i64;
                        let r_int = *r as i64;
                        Ok(Value::Number((l_int << r_int) as f64))
                    }
                    BinaryOp::RightShift => {
                        let l_int = *l as i64;
                        let r_int = *r as i64;
                        Ok(Value::Number((l_int >> r_int) as f64))
                    }
                    _ => Err(RuntimeError::InvalidOperation(format!("Cannot apply {:?} to numbers", op))),
                }
            }
            (Value::String(l), Value::String(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::String(format!("{}{}", l, r))),
                    BinaryOp::Equal => Ok(Value::Boolean(l == r)),
                    BinaryOp::NotEqual => Ok(Value::Boolean(l != r)),
                    _ => Err(RuntimeError::InvalidOperation(format!("Cannot apply {:?} to strings", op))),
                }
            }
            (Value::Boolean(l), Value::Boolean(r)) => {
                match op {
                    BinaryOp::Equal => Ok(Value::Boolean(l == r)),
                    BinaryOp::NotEqual => Ok(Value::Boolean(l != r)),
                    BinaryOp::And => Ok(Value::Boolean(*l && *r)),
                    BinaryOp::Or => Ok(Value::Boolean(*l || *r)),
                    _ => Err(RuntimeError::InvalidOperation(format!("Cannot apply {:?} to booleans", op))),
                }
            }
            _ => {
                match op {
                    BinaryOp::Add => {
                        // Auto-convert values to string for concatenation
                        Ok(Value::String(format!("{}{}", left, right)))
                    }
                    BinaryOp::Equal => Ok(Value::Boolean(left == right)),
                    BinaryOp::NotEqual => Ok(Value::Boolean(left != right)),
                    BinaryOp::And => Ok(Value::Boolean(left.is_truthy() && right.is_truthy())),
                    BinaryOp::Or => Ok(Value::Boolean(left.is_truthy() || right.is_truthy())),
                    _ => Err(RuntimeError::TypeError(
                        format!("Type mismatch: cannot apply {:?} to {} and {}", 
                                op, left.type_name(), right.type_name())
                    )),
                }
            }
        }
    }

    fn evaluate_unary(&self, op: &UnaryOp, operand: &Value) -> RuntimeResult<Value> {
        match op {
            UnaryOp::Not => Ok(Value::Boolean(!operand.is_truthy())),
            UnaryOp::Minus => {
                match operand {
                    Value::Number(n) => Ok(Value::Number(-n)),
                    _ => Err(RuntimeError::TypeError(
                        format!("Cannot apply unary minus to {}", operand.type_name())
                    )),
                }
            }
            UnaryOp::Plus => {
                match operand {
                    Value::Number(n) => Ok(Value::Number(*n)),
                    _ => Err(RuntimeError::TypeError(
                        format!("Cannot apply unary plus to {}", operand.type_name())
                    )),
                }
            }
            UnaryOp::BitwiseNot => {
                match operand {
                    Value::Number(n) => {
                        let int_val = *n as i64;
                        Ok(Value::Number((!int_val) as f64))
                    }
                    _ => Err(RuntimeError::TypeError(
                        format!("Cannot apply bitwise not to {}", operand.type_name())
                    )),
                }
            }
        }
    }

    fn call_function(&mut self, func: &Value, args: &[Value]) -> RuntimeResult<Value> {
        match func {
            Value::NativeFunction { name, arity } => {
                if args.len() != *arity {
                    return Err(RuntimeError::InvalidOperation(
                        format!("Function '{}' expects {} arguments, got {}", name, arity, args.len())
                    ));
                }
                self.call_native_function(name, args)
            }
            Value::Function { params, body, closure } => {
                if args.len() != params.len() {
                    return Err(RuntimeError::InvalidOperation(
                        format!("Function expects {} arguments, got {}", params.len(), args.len())
                    ));
                }

                let previous_env = self.environment.clone();
                
                // Check if 'this' is already in environment (for constructor/method calls)
                let preserve_this = self.environment.get("this");
                
                self.environment = Environment::with_parent(closure.clone());

                // Restore 'this' if it was present
                if let Some(this_value) = preserve_this {
                    self.environment.define("this".to_string(), this_value);
                }

                for (param, arg) in params.iter().zip(args.iter()) {
                    self.environment.define(param.clone(), arg.clone());
                }

                let result = match self.evaluate(body) {
                    Ok(value) => Ok(value),
                    Err(RuntimeError::ReturnValue(value)) => Ok(value),
                    Err(e) => Err(e),
                };

                self.environment = previous_env;
                result
            }
            Value::Class { .. } => {
                // Calling a class directly instantiates it
                self.instantiate_class(func, args)
            }
            _ => Err(RuntimeError::TypeError(
                format!("Cannot call non-function value: {}", func.type_name())
            )),
        }
    }
    
    fn call_native_function(&mut self, name: &str, args: &[Value]) -> RuntimeResult<Value> {
        match name {
            "print" => {
                println!("{}", args[0]);
                Ok(Value::Null)
            }
            "println" => {
                println!("{}", args[0]);
                Ok(Value::Null)
            }
            "input" => {
                use std::io::{self, Write};
                print!("{}", args[0]);
                io::stdout().flush().unwrap();
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                Ok(Value::String(input.trim().to_string()))
            }
            "len" => {
                if args.len() != 1 {
                    return Err(RuntimeError::InvalidOperation("len expects exactly one argument".to_string()));
                }
                match &args[0] {
                    Value::String(s) => Ok(Value::Number(s.len() as f64)),
                    Value::Array(arr) => Ok(Value::Number(arr.len() as f64)),
                    _ => Err(RuntimeError::TypeError("len can only be applied to strings and arrays".to_string()))
                }
            }
            "push" => {
                if args.len() != 2 {
                    return Err(RuntimeError::InvalidOperation("push expects exactly two arguments".to_string()));
                }
                match &args[0] {
                    Value::Array(arr) => {
                        let mut new_arr = arr.clone();
                        new_arr.push(args[1].clone());
                        Ok(Value::Array(new_arr))
                    }
                    _ => Err(RuntimeError::TypeError("push can only be applied to arrays".to_string()))
                }
            }
            "pop" => {
                if args.len() != 1 {
                    return Err(RuntimeError::InvalidOperation("pop expects exactly one argument".to_string()));
                }
                match &args[0] {
                    Value::Array(arr) => {
                        if arr.is_empty() {
                            Ok(Value::Null)
                        } else {
                            let mut new_arr = arr.clone();
                            let popped = new_arr.pop().unwrap();
                            Ok(popped)
                        }
                    }
                    _ => Err(RuntimeError::TypeError("pop can only be applied to arrays".to_string()))
                }
            }
            "type" => {
                if args.len() != 1 {
                    return Err(RuntimeError::InvalidOperation("type expects exactly one argument".to_string()));
                }
                Ok(Value::String(args[0].type_name().to_string()))
            }
            "str" => {
                if args.len() != 1 {
                    return Err(RuntimeError::InvalidOperation("str expects exactly one argument".to_string()));
                }
                Ok(Value::String(args[0].to_string()))
            }
            "num" => {
                if args.len() != 1 {
                    return Err(RuntimeError::InvalidOperation("num expects exactly one argument".to_string()));
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Number(*n)),
                    Value::String(s) => {
                        match s.parse::<f64>() {
                            Ok(n) => Ok(Value::Number(n)),
                            Err(_) => Err(RuntimeError::InvalidOperation("Cannot convert string to number".to_string()))
                        }
                    }
                    Value::Boolean(true) => Ok(Value::Number(1.0)),
                    Value::Boolean(false) => Ok(Value::Number(0.0)),
                    _ => Err(RuntimeError::TypeError("Cannot convert value to number".to_string()))
                }
            }
            // Math functions
            "abs" => {
                match &args[0] {
                    Value::Number(n) => Ok(Value::Number(n.abs())),
                    _ => Err(RuntimeError::TypeError("abs() requires a number".to_string()))
                }
            }
            "sqrt" => {
                match &args[0] {
                    Value::Number(n) => {
                        if *n >= 0.0 {
                            Ok(Value::Number(n.sqrt()))
                        } else {
                            Err(RuntimeError::InvalidOperation("sqrt() of negative number".to_string()))
                        }
                    }
                    _ => Err(RuntimeError::TypeError("sqrt() requires a number".to_string()))
                }
            }
            "pow" => {
                match (&args[0], &args[1]) {
                    (Value::Number(base), Value::Number(exp)) => Ok(Value::Number(base.powf(*exp))),
                    _ => Err(RuntimeError::TypeError("pow() requires two numbers".to_string()))
                }
            }
            "sin" => {
                match &args[0] {
                    Value::Number(n) => Ok(Value::Number(n.sin())),
                    _ => Err(RuntimeError::TypeError("sin() requires a number".to_string()))
                }
            }
            "cos" => {
                match &args[0] {
                    Value::Number(n) => Ok(Value::Number(n.cos())),
                    _ => Err(RuntimeError::TypeError("cos() requires a number".to_string()))
                }
            }
            "random" => {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                use std::time::{SystemTime, UNIX_EPOCH};
                
                let mut hasher = DefaultHasher::new();
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
                let hash = hasher.finish();
                let random_val = (hash as f64) / (u64::MAX as f64);
                Ok(Value::Number(random_val))
            }
            // String functions
            "substr" => {
                if args.len() != 3 {
                    return Err(RuntimeError::InvalidOperation("substr() requires 3 arguments: string, start, length".to_string()));
                }
                match (&args[0], &args[1], &args[2]) {
                    (Value::String(s), Value::Number(start), Value::Number(len)) => {
                        let start = *start as usize;
                        let len = *len as usize;
                        let chars: Vec<char> = s.chars().collect();
                        if start >= chars.len() {
                            Ok(Value::String("".to_string()))
                        } else {
                            let end = std::cmp::min(start + len, chars.len());
                            let substring: String = chars[start..end].iter().collect();
                            Ok(Value::String(substring))
                        }
                    }
                    _ => Err(RuntimeError::TypeError("substr() requires string, number, number".to_string()))
                }
            }
            "upper" => {
                match &args[0] {
                    Value::String(s) => Ok(Value::String(s.to_uppercase())),
                    _ => Err(RuntimeError::TypeError("upper() requires a string".to_string()))
                }
            }
            "lower" => {
                match &args[0] {
                    Value::String(s) => Ok(Value::String(s.to_lowercase())),
                    _ => Err(RuntimeError::TypeError("lower() requires a string".to_string()))
                }
            }
            "trim" => {
                match &args[0] {
                    Value::String(s) => Ok(Value::String(s.trim().to_string())),
                    _ => Err(RuntimeError::TypeError("trim() requires a string".to_string()))
                }
            }
            "split" => {
                if args.len() != 2 {
                    return Err(RuntimeError::InvalidOperation("split() requires 2 arguments: string, delimiter".to_string()));
                }
                match (&args[0], &args[1]) {
                    (Value::String(s), Value::String(delimiter)) => {
                        let parts: Vec<Value> = s.split(delimiter)
                            .map(|part| Value::String(part.to_string()))
                            .collect();
                        Ok(Value::Array(parts))
                    }
                    _ => Err(RuntimeError::TypeError("split() requires two strings".to_string()))
                }
            }
            "join" => {
                if args.len() != 2 {
                    return Err(RuntimeError::InvalidOperation("join() requires 2 arguments: array, separator".to_string()));
                }
                match (&args[0], &args[1]) {
                    (Value::Array(arr), Value::String(sep)) => {
                        let strings: Vec<String> = arr.iter()
                            .map(|val| val.to_string())
                            .collect();
                        Ok(Value::String(strings.join(sep)))
                    }
                    _ => Err(RuntimeError::TypeError("join() requires array and string".to_string()))
                }
            }
            // Array functions
            "reverse" => {
                match &args[0] {
                    Value::Array(arr) => {
                        let mut reversed = arr.clone();
                        reversed.reverse();
                        Ok(Value::Array(reversed))
                    }
                    _ => Err(RuntimeError::TypeError("reverse() requires an array".to_string()))
                }
            }
            "sort" => {
                match &args[0] {
                    Value::Array(arr) => {
                        let mut sorted = arr.clone();
                        sorted.sort_by(|a, b| {
                            match (a, b) {
                                (Value::Number(n1), Value::Number(n2)) => n1.partial_cmp(n2).unwrap_or(std::cmp::Ordering::Equal),
                                (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
                                _ => std::cmp::Ordering::Equal,
                            }
                        });
                        Ok(Value::Array(sorted))
                    }
                    _ => Err(RuntimeError::TypeError("sort() requires an array".to_string()))
                }
            }
            "contains" => {
                if args.len() != 2 {
                    return Err(RuntimeError::InvalidOperation("contains() requires 2 arguments".to_string()));
                }
                match (&args[0], &args[1]) {
                    (Value::Array(arr), value) => {
                        Ok(Value::Boolean(arr.contains(value)))
                    }
                    (Value::String(s), Value::String(substr)) => {
                        Ok(Value::Boolean(s.contains(substr)))
                    }
                    _ => Err(RuntimeError::TypeError("contains() requires array/value or string/string".to_string()))
                }
            }
            // Time functions
            "now" => {
                use std::time::{SystemTime, UNIX_EPOCH};
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as f64;
                Ok(Value::Number(timestamp))
            }
            "sleep" => {
                match &args[0] {
                    Value::Number(seconds) => {
                        std::thread::sleep(std::time::Duration::from_millis((*seconds * 1000.0) as u64));
                        Ok(Value::Null)
                    }
                    _ => Err(RuntimeError::TypeError("sleep() requires a number (seconds)".to_string()))
                }
            }
            // File I/O functions
            "read_file" => {
                match &args[0] {
                    Value::String(filename) => {
                        match std::fs::read_to_string(filename) {
                            Ok(content) => Ok(Value::String(content)),
                            Err(e) => Err(RuntimeError::InvalidOperation(format!("Cannot read file '{}': {}", filename, e)))
                        }
                    }
                    _ => Err(RuntimeError::TypeError("read_file() requires a filename string".to_string()))
                }
            }
            "write_file" => {
                if args.len() != 2 {
                    return Err(RuntimeError::InvalidOperation("write_file() requires 2 arguments: filename, content".to_string()));
                }
                match (&args[0], &args[1]) {
                    (Value::String(filename), Value::String(content)) => {
                        match std::fs::write(filename, content) {
                            Ok(()) => Ok(Value::Null),
                            Err(e) => Err(RuntimeError::InvalidOperation(format!("Cannot write file '{}': {}", filename, e)))
                        }
                    }
                    _ => Err(RuntimeError::TypeError("write_file() requires filename and content strings".to_string()))
                }
            }
            "exists" => {
                match &args[0] {
                    Value::String(path) => Ok(Value::Boolean(std::path::Path::new(path).exists())),
                    _ => Err(RuntimeError::TypeError("exists() requires a path string".to_string()))
                }
            }
            // HTTP functions (basic implementation)
            "http_get" => {
                match &args[0] {
                    Value::String(url) => {
                        match reqwest::blocking::get(url) {
                            Ok(response) => {
                                match response.text() {
                                    Ok(body) => Ok(Value::String(body)),
                                    Err(e) => Err(RuntimeError::InvalidOperation(format!("HTTP GET failed: {}", e)))
                                }
                            }
                            Err(e) => Err(RuntimeError::InvalidOperation(format!("HTTP GET failed: {}", e)))
                        }
                    }
                    _ => Err(RuntimeError::TypeError("http_get() requires a URL string".to_string()))
                }
            }
            "http_post" => {
                if args.len() != 2 {
                    return Err(RuntimeError::InvalidOperation("http_post() requires 2 arguments: url, data".to_string()));
                }
                match (&args[0], &args[1]) {
                    (Value::String(url), Value::String(data)) => {
                        let client = reqwest::blocking::Client::new();
                        match client.post(url).body(data.clone()).send() {
                            Ok(response) => {
                                match response.text() {
                                    Ok(body) => Ok(Value::String(body)),
                                    Err(e) => Err(RuntimeError::InvalidOperation(format!("HTTP POST failed: {}", e)))
                                }
                            }
                            Err(e) => Err(RuntimeError::InvalidOperation(format!("HTTP POST failed: {}", e)))
                        }
                    }
                    _ => Err(RuntimeError::TypeError("http_post() requires URL and data strings".to_string()))
                }
            }
            // JSON functions
            "json_parse" => {
                match &args[0] {
                    Value::String(json_str) => {
                        match serde_json::from_str(json_str) {
                            Ok(json_value) => Ok(Value::from_json(&json_value)),
                            Err(e) => Err(RuntimeError::InvalidOperation(format!("JSON parse error: {}", e)))
                        }
                    }
                    _ => Err(RuntimeError::TypeError("json_parse() requires a JSON string".to_string()))
                }
            }
            "json_stringify" => {
                let json_value = args[0].to_json();
                match serde_json::to_string(&json_value) {
                    Ok(json_str) => Ok(Value::String(json_str)),
                    Err(e) => Err(RuntimeError::InvalidOperation(format!("JSON stringify error: {}", e)))
                }
            }
            // Regex functions
            "regex_match" => {
                if args.len() != 2 {
                    return Err(RuntimeError::InvalidOperation("regex_match() requires 2 arguments: pattern, text".to_string()));
                }
                match (&args[0], &args[1]) {
                    (Value::String(pattern), Value::String(text)) => {
                        match regex::Regex::new(pattern) {
                            Ok(re) => Ok(Value::Boolean(re.is_match(text))),
                            Err(e) => Err(RuntimeError::InvalidOperation(format!("Invalid regex pattern: {}", e)))
                        }
                    }
                    _ => Err(RuntimeError::TypeError("regex_match() requires two strings".to_string()))
                }
            }
            "regex_find" => {
                if args.len() != 2 {
                    return Err(RuntimeError::InvalidOperation("regex_find() requires 2 arguments: pattern, text".to_string()));
                }
                match (&args[0], &args[1]) {
                    (Value::String(pattern), Value::String(text)) => {
                        match regex::Regex::new(pattern) {
                            Ok(re) => {
                                match re.find(text) {
                                    Some(mat) => {
                                        let mut result = std::collections::HashMap::new();
                                        result.insert("match".to_string(), Value::String(mat.as_str().to_string()));
                                        result.insert("start".to_string(), Value::Number(mat.start() as f64));
                                        result.insert("end".to_string(), Value::Number(mat.end() as f64));
                                        Ok(Value::Object(result))
                                    }
                                    None => Ok(Value::Null)
                                }
                            }
                            Err(e) => Err(RuntimeError::InvalidOperation(format!("Invalid regex pattern: {}", e)))
                        }
                    }
                    _ => Err(RuntimeError::TypeError("regex_find() requires two strings".to_string()))
                }
            }
            "regex_replace" => {
                if args.len() != 3 {
                    return Err(RuntimeError::InvalidOperation("regex_replace() requires 3 arguments: pattern, text, replacement".to_string()));
                }
                match (&args[0], &args[1], &args[2]) {
                    (Value::String(pattern), Value::String(text), Value::String(replacement)) => {
                        match regex::Regex::new(pattern) {
                            Ok(re) => Ok(Value::String(re.replace_all(text, replacement.as_str()).to_string())),
                            Err(e) => Err(RuntimeError::InvalidOperation(format!("Invalid regex pattern: {}", e)))
                        }
                    }
                    _ => Err(RuntimeError::TypeError("regex_replace() requires three strings".to_string()))
                }
            }
            "regex_split" => {
                if args.len() != 2 {
                    return Err(RuntimeError::InvalidOperation("regex_split() requires 2 arguments: pattern, text".to_string()));
                }
                match (&args[0], &args[1]) {
                    (Value::String(pattern), Value::String(text)) => {
                        match regex::Regex::new(pattern) {
                            Ok(re) => {
                                let parts: Vec<Value> = re.split(text)
                                    .map(|part| Value::String(part.to_string()))
                                    .collect();
                                Ok(Value::Array(parts))
                            }
                            Err(e) => Err(RuntimeError::InvalidOperation(format!("Invalid regex pattern: {}", e)))
                        }
                    }
                    _ => Err(RuntimeError::TypeError("regex_split() requires two strings".to_string()))
                }
            }
            "regex_find_all" => {
                if args.len() != 2 {
                    return Err(RuntimeError::InvalidOperation("regex_find_all() requires 2 arguments: pattern, text".to_string()));
                }
                match (&args[0], &args[1]) {
                    (Value::String(pattern), Value::String(text)) => {
                        match regex::Regex::new(pattern) {
                            Ok(re) => {
                                let matches: Vec<Value> = re.find_iter(text)
                                    .map(|mat| {
                                        let mut result = std::collections::HashMap::new();
                                        result.insert("match".to_string(), Value::String(mat.as_str().to_string()));
                                        result.insert("start".to_string(), Value::Number(mat.start() as f64));
                                        result.insert("end".to_string(), Value::Number(mat.end() as f64));
                                        Value::Object(result)
                                    })
                                    .collect();
                                Ok(Value::Array(matches))
                            }
                            Err(e) => Err(RuntimeError::InvalidOperation(format!("Invalid regex pattern: {}", e)))
                        }
                    }
                    _ => Err(RuntimeError::TypeError("regex_find_all() requires two strings".to_string()))
                }
            }
            _ => Err(RuntimeError::InvalidOperation(format!("Unknown built-in function: {}", name)))
        }
    }

    fn import_module(&mut self, module_path: &str, alias: Option<&String>) -> RuntimeResult<Option<Value>> {
        // Try to load module from file system
        let module_file = if module_path.ends_with(".nova") {
            module_path.to_string()
        } else {
            format!("{}.nova", module_path)
        };

        // Check if file exists
        if !std::path::Path::new(&module_file).exists() {
            // Try standard library path
            let std_path = format!("std/{}", module_file);
            if !std::path::Path::new(&std_path).exists() {
                return Err(RuntimeError::InvalidOperation(format!("Module not found: {}", module_path)));
            }
        }

        // Read and execute module
        let module_content = match std::fs::read_to_string(&module_file) {
            Ok(content) => content,
            Err(_) => {
                // Try std path
                let std_path = format!("std/{}", module_file);
                match std::fs::read_to_string(&std_path) {
                    Ok(content) => content,
                    Err(e) => return Err(RuntimeError::InvalidOperation(format!("Cannot read module '{}': {}", module_path, e))),
                }
            }
        };

        // Parse and execute module
        let mut lexer = crate::lexer::Lexer::new(&module_content);
        let tokens = match lexer.tokenize() {
            Ok(tokens) => tokens,
            Err(e) => return Err(RuntimeError::InvalidOperation(format!("Module parse error: {}", e))),
        };

        let mut parser = crate::parser::Parser::new(tokens);
        let program = match parser.parse() {
            Ok(program) => program,
            Err(e) => return Err(RuntimeError::InvalidOperation(format!("Module parse error: {:?}", e))),
        };

        // Create new environment for module
        let previous_env = self.environment.clone();
        let mut module_env = crate::value::Environment::new();
        module_env.define_natives();
        self.environment = module_env;

        // Execute module
        let module_result = self.interpret(&program);
        let module_exports = self.environment.clone();
        
        // Restore previous environment
        self.environment = previous_env;

        match module_result {
            Ok(_) => {
                // Import module exports into current environment
                let module_name = match alias {
                    Some(alias_name) => alias_name.clone(),
                    None => module_path.replace("/", "_").replace(".", "_"),
                };
                
                // Create module object with all exported functions/variables
                let mut module_obj = std::collections::HashMap::new();
                for (name, value) in module_exports.get_all_variables() {
                    // Skip native functions, only import user-defined ones
                    match value {
                        crate::value::Value::NativeFunction { .. } => continue,
                        _ => {
                            module_obj.insert(name, value);
                        }
                    }
                }

                self.environment.define(module_name, crate::value::Value::Object(module_obj));
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    fn define_class(&mut self, name: &str, superclass: Option<&String>, methods: &[ClassMethod], constructor: Option<&ClassMethod>) -> RuntimeResult<Option<Value>> {
        // Handle superclass
        let superclass_value = if let Some(superclass_name) = superclass {
            match self.environment.get(superclass_name) {
                Some(Value::Class { .. }) => Some(Box::new(self.environment.get(superclass_name).unwrap())),
                Some(_) => return Err(RuntimeError::TypeError("Superclass must be a class".to_string())),
                None => return Err(RuntimeError::UndefinedVariable(format!("Undefined superclass: {}", superclass_name))),
            }
        } else {
            None
        };

        // Process methods
        let mut class_methods = HashMap::new();
        let mut static_methods = HashMap::new();

        for method in methods {
            let method_value = Value::Function {
                params: method.params.clone(),
                body: method.body.clone(),
                closure: self.environment.clone(),
            };

            if method.is_static {
                static_methods.insert(method.name.clone(), method_value);
            } else {
                class_methods.insert(method.name.clone(), method_value);
            }
        }

        // Process constructor
        let constructor_value = if let Some(ctor) = constructor {
            Some(Box::new(Value::Function {
                params: ctor.params.clone(),
                body: ctor.body.clone(),
                closure: self.environment.clone(),
            }))
        } else {
            None
        };

        let class = Value::Class {
            name: name.to_string(),
            superclass: superclass_value,
            methods: class_methods,
            static_methods,
            constructor: constructor_value,
        };

        self.environment.define(name.to_string(), class);
        Ok(None)
    }

    fn instantiate_class(&mut self, class: &Value, args: &[Value]) -> RuntimeResult<Value> {
        match class {
            Value::Class { constructor, .. } => {
                // Create new instance
                let mut instance = Value::Instance {
                    class: Box::new(class.clone()),
                    fields: HashMap::new(),
                };

                // Call constructor if it exists
                if let Some(constructor_func) = constructor {
                    // Set up constructor environment with 'this' binding
                    let prev_env = self.environment.clone();
                    let mut constructor_env = Environment::with_parent(self.environment.clone());
                    constructor_env.define("this".to_string(), instance.clone());
                    self.environment = constructor_env;
                    
                    // Call constructor
                    let result = self.call_function(constructor_func.as_ref(), args);
                    
                    // Get updated instance from 'this' if it was modified
                    if let Some(updated_this) = self.environment.get("this") {
                        instance = updated_this;
                    }
                    
                    // Restore environment
                    self.environment = prev_env;
                    
                    // Check constructor result
                    match result {
                        Ok(_) => {}
                        Err(RuntimeError::ReturnValue(_)) => {} // Constructor returns are ignored
                        Err(e) => return Err(e),
                    }
                }

                Ok(instance)
            }
            _ => Err(RuntimeError::TypeError(format!("Cannot instantiate non-class value: {}", class.type_name()))),
        }
    }

    fn call_method(&mut self, instance: &Value, method_name: &str, args: &[Value]) -> RuntimeResult<Value> {
        match instance {
            Value::Instance { class, .. } => {
                if let Value::Class { methods, .. } = class.as_ref() {
                    if let Some(method) = methods.get(method_name) {
                        // Set up method environment with 'this' binding
                        let prev_env = self.environment.clone();
                        let mut method_env = Environment::with_parent(self.environment.clone());
                        method_env.define("this".to_string(), instance.clone());
                        self.environment = method_env;
                        
                        // Call method
                        let result = self.call_function(method, args);
                        
                        // Restore environment
                        self.environment = prev_env;
                        
                        result
                    } else {
                        Err(RuntimeError::InvalidOperation(format!("Method '{}' not found", method_name)))
                    }
                } else {
                    Err(RuntimeError::TypeError("Invalid class structure".to_string()))
                }
            }
            _ => Err(RuntimeError::TypeError(format!("Cannot call method '{}' on {}", method_name, instance.type_name()))),
        }
    }

    fn evaluate_template_string(&mut self, parts: &[crate::ast::TemplateStringPart]) -> RuntimeResult<Value> {
        let mut result = String::new();
        
        for part in parts {
            match part {
                crate::ast::TemplateStringPart::Text(text) => {
                    result.push_str(text);
                }
                crate::ast::TemplateStringPart::Expression(expr) => {
                    let value = self.evaluate(expr)?;
                    result.push_str(&format!("{}", value));
                }
            }
        }
        
        Ok(Value::String(result))
    }

    fn evaluate_await(&mut self, promise_expr: &Expr) -> RuntimeResult<Value> {
        let promise_value = self.evaluate(promise_expr)?;
        
        match promise_value {
            Value::Promise(promise) => {
                match &promise.state {
                    crate::value::PromiseState::Resolved(value) => Ok(value.clone()),
                    crate::value::PromiseState::Rejected(error) => {
                        Err(RuntimeError::RuntimeError(error.clone()))
                    }
                    crate::value::PromiseState::Pending => {
                        // In a real implementation, this would block or schedule continuation
                        // For now, we'll return a runtime error
                        Err(RuntimeError::RuntimeError("Cannot await pending promise in synchronous context".to_string()))
                    }
                }
            }
            _ => Err(RuntimeError::TypeError("Cannot await non-promise value".to_string())),
        }
    }
}