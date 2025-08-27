use crate::ast::Expr;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HashableValue {
    Number(i64),
    String(String),
    Boolean(bool),
    Null,
}

impl HashableValue {
    pub fn from_value(value: &Value) -> Option<HashableValue> {
        match value {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    Some(HashableValue::Number(*n as i64))
                } else {
                    None // Floating point numbers can't be hashed reliably
                }
            },
            Value::String(s) => Some(HashableValue::String(s.clone())),
            Value::Boolean(b) => Some(HashableValue::Boolean(*b)),
            Value::Null => Some(HashableValue::Null),
            _ => None, // Complex types can't be hashed
        }
    }

    pub fn to_value(&self) -> Value {
        match self {
            HashableValue::Number(n) => Value::Number(*n as f64),
            HashableValue::String(s) => Value::String(s.clone()),
            HashableValue::Boolean(b) => Value::Boolean(*b),
            HashableValue::Null => Value::Null,
        }
    }
}

pub use crate::ast::TemplateStringPart;

#[derive(Debug, Clone, PartialEq)]
pub enum PromiseState {
    Pending,
    Resolved(Value),
    Rejected(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Promise {
    pub state: PromiseState,
    pub then_callbacks: Vec<Value>,
    pub catch_callbacks: Vec<Value>,
    pub finally_callbacks: Vec<Value>,
}

impl Promise {
    pub fn new() -> Self {
        Promise {
            state: PromiseState::Pending,
            then_callbacks: Vec::new(),
            catch_callbacks: Vec::new(),
            finally_callbacks: Vec::new(),
        }
    }

    pub fn resolved(value: Value) -> Self {
        Promise {
            state: PromiseState::Resolved(value),
            then_callbacks: Vec::new(),
            catch_callbacks: Vec::new(),
            finally_callbacks: Vec::new(),
        }
    }

    pub fn rejected(error: String) -> Self {
        Promise {
            state: PromiseState::Rejected(error),
            then_callbacks: Vec::new(),
            catch_callbacks: Vec::new(),
            finally_callbacks: Vec::new(),
        }
    }

    pub fn is_pending(&self) -> bool {
        matches!(self.state, PromiseState::Pending)
    }

    pub fn is_resolved(&self) -> bool {
        matches!(self.state, PromiseState::Resolved(_))
    }

    pub fn is_rejected(&self) -> bool {
        matches!(self.state, PromiseState::Rejected(_))
    }

    pub fn resolve(&mut self, value: Value) -> Result<(), String> {
        if self.is_pending() {
            self.state = PromiseState::Resolved(value);
            Ok(())
        } else {
            Err("Promise already settled".to_string())
        }
    }

    pub fn reject(&mut self, error: String) -> Result<(), String> {
        if self.is_pending() {
            self.state = PromiseState::Rejected(error);
            Ok(())
        } else {
            Err("Promise already settled".to_string())
        }
    }

    pub fn add_then_callback(&mut self, callback: Value) {
        self.then_callbacks.push(callback);
    }

    pub fn add_catch_callback(&mut self, callback: Value) {
        self.catch_callbacks.push(callback);
    }

    pub fn add_finally_callback(&mut self, callback: Value) {
        self.finally_callbacks.push(callback);
    }

    pub fn get_value(&self) -> Option<&Value> {
        match &self.state {
            PromiseState::Resolved(value) => Some(value),
            _ => None,
        }
    }

    pub fn get_error(&self) -> Option<&String> {
        match &self.state {
            PromiseState::Rejected(error) => Some(error),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    TemplateString(Vec<TemplateStringPart>),
    Boolean(bool),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Function {
        params: Vec<String>,
        body: Expr,
        closure: Environment,
    },
    NativeFunction {
        name: String,
        arity: usize,
    },
    Class {
        name: String,
        superclass: Option<Box<Value>>,
        methods: HashMap<String, Value>,
        static_methods: HashMap<String, Value>,
        constructor: Option<Box<Value>>,
    },
    Instance {
        class: Box<Value>,
        fields: HashMap<String, Value>,
    },
    Map(HashMap<String, Value>),
    Set(HashSet<HashableValue>),
    Promise(Box<Promise>),
    AsyncFunction {
        params: Vec<String>,
        body: Expr,
        closure: Environment,
    },
    Null,
}

impl Value {
    // Class-specific methods
    pub fn get_method(&self, name: &str) -> Option<Value> {
        match self {
            Value::Class { methods, static_methods, .. } => {
                methods.get(name).cloned().or_else(|| static_methods.get(name).cloned())
            }
            Value::Instance { class, .. } => {
                if let Value::Class { methods, .. } = class.as_ref() {
                    methods.get(name).cloned()
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn get_field(&self, name: &str) -> Option<Value> {
        match self {
            Value::Instance { fields, .. } => fields.get(name).cloned(),
            _ => None,
        }
    }

    pub fn set_field(&mut self, name: String, value: Value) -> Result<(), String> {
        match self {
            Value::Instance { fields, .. } => {
                fields.insert(name, value);
                Ok(())
            }
            _ => Err("Cannot set field on non-instance".to_string()),
        }
    }

    // Map methods
    pub fn map_get(&self, key: &str) -> Option<Value> {
        match self {
            Value::Map(map) => map.get(key).cloned(),
            _ => None,
        }
    }

    pub fn map_set(&mut self, key: String, value: Value) -> Result<(), String> {
        match self {
            Value::Map(map) => {
                map.insert(key, value);
                Ok(())
            }
            _ => Err("Cannot set key on non-map".to_string()),
        }
    }

    pub fn map_has(&self, key: &str) -> bool {
        match self {
            Value::Map(map) => map.contains_key(key),
            _ => false,
        }
    }

    pub fn map_delete(&mut self, key: &str) -> bool {
        match self {
            Value::Map(map) => map.remove(key).is_some(),
            _ => false,
        }
    }

    pub fn map_keys(&self) -> Vec<String> {
        match self {
            Value::Map(map) => map.keys().cloned().collect(),
            _ => Vec::new(),
        }
    }

    pub fn map_values(&self) -> Vec<Value> {
        match self {
            Value::Map(map) => map.values().cloned().collect(),
            _ => Vec::new(),
        }
    }

    pub fn map_size(&self) -> usize {
        match self {
            Value::Map(map) => map.len(),
            _ => 0,
        }
    }

    // Set methods
    pub fn set_add(&mut self, value: Value) -> Result<bool, String> {
        match self {
            Value::Set(set) => {
                if let Some(hashable) = HashableValue::from_value(&value) {
                    Ok(set.insert(hashable))
                } else {
                    Err("Cannot add non-hashable value to set".to_string())
                }
            }
            _ => Err("Cannot add to non-set".to_string()),
        }
    }

    pub fn set_has(&self, value: &Value) -> bool {
        match self {
            Value::Set(set) => {
                if let Some(hashable) = HashableValue::from_value(value) {
                    set.contains(&hashable)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn set_delete(&mut self, value: &Value) -> bool {
        match self {
            Value::Set(set) => {
                if let Some(hashable) = HashableValue::from_value(value) {
                    set.remove(&hashable)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn set_values(&self) -> Vec<Value> {
        match self {
            Value::Set(set) => set.iter().map(|h| h.to_value()).collect(),
            _ => Vec::new(),
        }
    }

    pub fn set_size(&self) -> usize {
        match self {
            Value::Set(set) => set.len(),
            _ => 0,
        }
    }

    pub fn set_clear(&mut self) -> Result<(), String> {
        match self {
            Value::Set(set) => {
                set.clear();
                Ok(())
            }
            _ => Err("Cannot clear non-set".to_string()),
        }
    }

    // Template string methods
    pub fn is_template_string(&self) -> bool {
        matches!(self, Value::TemplateString(_))
    }

    pub fn template_parts(&self) -> Option<&Vec<TemplateStringPart>> {
        match self {
            Value::TemplateString(parts) => Some(parts),
            _ => None,
        }
    }

    pub fn create_template_string(parts: Vec<TemplateStringPart>) -> Value {
        Value::TemplateString(parts)
    }

    // Promise methods
    pub fn is_promise(&self) -> bool {
        matches!(self, Value::Promise(_))
    }

    pub fn is_async_function(&self) -> bool {
        matches!(self, Value::AsyncFunction { .. })
    }

    pub fn promise_then(&mut self, callback: Value) -> Result<(), String> {
        match self {
            Value::Promise(promise) => {
                promise.add_then_callback(callback);
                Ok(())
            }
            _ => Err("Cannot call 'then' on non-promise".to_string()),
        }
    }

    pub fn promise_catch(&mut self, callback: Value) -> Result<(), String> {
        match self {
            Value::Promise(promise) => {
                promise.add_catch_callback(callback);
                Ok(())
            }
            _ => Err("Cannot call 'catch' on non-promise".to_string()),
        }
    }

    pub fn promise_finally(&mut self, callback: Value) -> Result<(), String> {
        match self {
            Value::Promise(promise) => {
                promise.add_finally_callback(callback);
                Ok(())
            }
            _ => Err("Cannot call 'finally' on non-promise".to_string()),
        }
    }

    pub fn promise_resolve(&mut self, value: Value) -> Result<(), String> {
        match self {
            Value::Promise(promise) => promise.resolve(value),
            _ => Err("Cannot resolve non-promise".to_string()),
        }
    }

    pub fn promise_reject(&mut self, error: String) -> Result<(), String> {
        match self {
            Value::Promise(promise) => promise.reject(error),
            _ => Err("Cannot reject non-promise".to_string()),
        }
    }

    pub fn promise_state(&self) -> Option<&PromiseState> {
        match self {
            Value::Promise(promise) => Some(&promise.state),
            _ => None,
        }
    }

    pub fn is_instance_of(&self, class: &Value) -> bool {
        match (self, class) {
            (Value::Instance { class: instance_class, .. }, Value::Class { name, .. }) => {
                if let Value::Class { name: instance_class_name, .. } = instance_class.as_ref() {
                    instance_class_name == name
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::TemplateString(parts) => !parts.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            Value::Object(obj) => !obj.is_empty(),
            Value::Map(map) => !map.is_empty(),
            Value::Set(set) => !set.is_empty(),
            Value::Promise(_) => true,
            Value::Function { .. } | Value::NativeFunction { .. } => true,
            Value::AsyncFunction { .. } => true,
            Value::Class { .. } => true,
            Value::Instance { .. } => true,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::TemplateString(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Map(_) => "map",
            Value::Set(_) => "set",
            Value::Promise(_) => "promise",
            Value::Function { .. } => "function",
            Value::AsyncFunction { .. } => "asyncfunction",
            Value::NativeFunction { .. } => "function",
            Value::Class { .. } => "class",
            Value::Instance { .. } => "instance",
            Value::Null => "null",
        }
    }

    pub fn is_callable(&self) -> bool {
        matches!(self, 
            Value::Function { .. } | 
            Value::AsyncFunction { .. } | 
            Value::NativeFunction { .. } | 
            Value::Class { .. }
        )
    }

    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Value::Number(n) => serde_json::json!(*n),
            Value::String(s) => serde_json::json!(s),
            Value::TemplateString(_) => serde_json::json!("<template string>"),
            Value::Boolean(b) => serde_json::json!(*b),
            Value::Array(arr) => {
                let json_arr: Vec<serde_json::Value> = arr.iter().map(|v| v.to_json()).collect();
                serde_json::json!(json_arr)
            }
            Value::Object(obj) => {
                let mut json_obj = serde_json::Map::new();
                for (k, v) in obj {
                    json_obj.insert(k.clone(), v.to_json());
                }
                serde_json::Value::Object(json_obj)
            }
            Value::Map(map) => {
                let mut json_obj = serde_json::Map::new();
                for (k, v) in map {
                    json_obj.insert(k.clone(), v.to_json());
                }
                serde_json::Value::Object(json_obj)
            }
            Value::Set(set) => {
                let json_arr: Vec<serde_json::Value> = set.iter()
                    .map(|h| h.to_value().to_json())
                    .collect();
                serde_json::json!(json_arr)
            }
            Value::Promise(_) => serde_json::json!("<promise>"),
            Value::AsyncFunction { params, .. } => {
                serde_json::json!(format!("<async function({})>", params.join(", ")))
            }
            Value::Null => serde_json::Value::Null,
            _ => serde_json::json!(format!("<{}>", self.type_name())),
        }
    }

    pub fn from_json(json: &serde_json::Value) -> Value {
        match json {
            serde_json::Value::Number(n) => Value::Number(n.as_f64().unwrap_or(0.0)),
            serde_json::Value::String(s) => Value::String(s.clone()),
            serde_json::Value::Bool(b) => Value::Boolean(*b),
            serde_json::Value::Array(arr) => {
                let nova_arr: Vec<Value> = arr.iter().map(|v| Value::from_json(v)).collect();
                Value::Array(nova_arr)
            }
            serde_json::Value::Object(obj) => {
                let mut nova_obj = HashMap::new();
                for (k, v) in obj {
                    nova_obj.insert(k.clone(), Value::from_json(v));
                }
                Value::Object(nova_obj)
            }
            serde_json::Value::Null => Value::Null,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::String(s) => write!(f, "{}", s),
            Value::TemplateString(_) => write!(f, "<template string>"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| format!("{}", v)).collect();
                write!(f, "[{}]", elements.join(", "))
            }
            Value::Object(obj) => {
                let pairs: Vec<String> = obj.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                write!(f, "{{{}}}", pairs.join(", "))
            }
            Value::Map(map) => {
                let pairs: Vec<String> = map.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                write!(f, "Map{{{}}}", pairs.join(", "))
            }
            Value::Set(set) => {
                let elements: Vec<String> = set.iter()
                    .map(|h| format!("{}", h.to_value()))
                    .collect();
                write!(f, "Set{{{}}}", elements.join(", "))
            }
            Value::Function { params, .. } => {
                write!(f, "<function({})>", params.join(", "))
            }
            Value::AsyncFunction { params, .. } => {
                write!(f, "<async function({})>", params.join(", "))
            }
            Value::NativeFunction { name, arity } => {
                write!(f, "<native function {}({} args)>", name, arity)
            }
            Value::Promise(promise) => {
                match &promise.state {
                    PromiseState::Pending => write!(f, "<Promise [Pending]>"),
                    PromiseState::Resolved(_) => write!(f, "<Promise [Resolved]>"),
                    PromiseState::Rejected(_) => write!(f, "<Promise [Rejected]>"),
                }
            }
            Value::Class { name, .. } => {
                write!(f, "<class {}>", name)
            }
            Value::Instance { class, .. } => {
                if let Value::Class { name, .. } = class.as_ref() {
                    write!(f, "<{} instance>", name)
                } else {
                    write!(f, "<instance>")
                }
            }
            Value::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    pub vars: HashMap<String, Value>,
    pub parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            vars: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Environment) -> Self {
        Environment {
            vars: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.vars.get(name).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.get(name))
        })
    }

    pub fn set(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.vars.contains_key(name) {
            self.vars.insert(name.to_string(), value);
            Ok(())
        } else if let Some(ref mut parent) = self.parent {
            parent.set(name, value)
        } else {
            Err(format!("Undefined variable: {}", name))
        }
    }

    pub fn define_natives(&mut self) {
        // Core I/O functions
        self.define("print".to_string(), Value::NativeFunction {
            name: "print".to_string(),
            arity: 1,
        });
        
        self.define("println".to_string(), Value::NativeFunction {
            name: "println".to_string(),
            arity: 1,
        });

        self.define("input".to_string(), Value::NativeFunction {
            name: "input".to_string(),
            arity: 1,
        });

        // Type functions
        self.define("type".to_string(), Value::NativeFunction {
            name: "type".to_string(),
            arity: 1,
        });

        self.define("str".to_string(), Value::NativeFunction {
            name: "str".to_string(),
            arity: 1,
        });

        self.define("num".to_string(), Value::NativeFunction {
            name: "num".to_string(),
            arity: 1,
        });

        self.define("bool".to_string(), Value::NativeFunction {
            name: "bool".to_string(),
            arity: 1,
        });

        // Collection functions
        self.define("len".to_string(), Value::NativeFunction {
            name: "len".to_string(),
            arity: 1,
        });

        self.define("push".to_string(), Value::NativeFunction {
            name: "push".to_string(),
            arity: 2,
        });

        self.define("pop".to_string(), Value::NativeFunction {
            name: "pop".to_string(),
            arity: 1,
        });

        self.define("keys".to_string(), Value::NativeFunction {
            name: "keys".to_string(),
            arity: 1,
        });

        self.define("values".to_string(), Value::NativeFunction {
            name: "values".to_string(),
            arity: 1,
        });

        // JSON functions
        self.define("json_parse".to_string(), Value::NativeFunction {
            name: "json_parse".to_string(),
            arity: 1,
        });

        self.define("json_stringify".to_string(), Value::NativeFunction {
            name: "json_stringify".to_string(),
            arity: 1,
        });

        // File I/O functions
        self.define("read_file".to_string(), Value::NativeFunction {
            name: "read_file".to_string(),
            arity: 1,
        });

        self.define("write_file".to_string(), Value::NativeFunction {
            name: "write_file".to_string(),
            arity: 2,
        });

        // HTTP functions
        self.define("http_get".to_string(), Value::NativeFunction {
            name: "http_get".to_string(),
            arity: 1,
        });

        self.define("http_post".to_string(), Value::NativeFunction {
            name: "http_post".to_string(),
            arity: 2,
        });

        // Math functions
        self.define("abs".to_string(), Value::NativeFunction {
            name: "abs".to_string(),
            arity: 1,
        });

        self.define("sqrt".to_string(), Value::NativeFunction {
            name: "sqrt".to_string(),
            arity: 1,
        });

        self.define("pow".to_string(), Value::NativeFunction {
            name: "pow".to_string(),
            arity: 2,
        });

        self.define("sin".to_string(), Value::NativeFunction {
            name: "sin".to_string(),
            arity: 1,
        });

        self.define("cos".to_string(), Value::NativeFunction {
            name: "cos".to_string(),
            arity: 1,
        });

        self.define("random".to_string(), Value::NativeFunction {
            name: "random".to_string(),
            arity: 0,
        });

        // Enhanced math functions
        self.define("log".to_string(), Value::NativeFunction {
            name: "log".to_string(),
            arity: 1,
        });

        self.define("log10".to_string(), Value::NativeFunction {
            name: "log10".to_string(),
            arity: 1,
        });

        self.define("exp".to_string(), Value::NativeFunction {
            name: "exp".to_string(),
            arity: 1,
        });

        self.define("asin".to_string(), Value::NativeFunction {
            name: "asin".to_string(),
            arity: 1,
        });

        self.define("acos".to_string(), Value::NativeFunction {
            name: "acos".to_string(),
            arity: 1,
        });

        self.define("atan".to_string(), Value::NativeFunction {
            name: "atan".to_string(),
            arity: 1,
        });

        self.define("atan2".to_string(), Value::NativeFunction {
            name: "atan2".to_string(),
            arity: 2,
        });

        self.define("degrees".to_string(), Value::NativeFunction {
            name: "degrees".to_string(),
            arity: 1,
        });

        self.define("radians".to_string(), Value::NativeFunction {
            name: "radians".to_string(),
            arity: 1,
        });

        self.define("random_int".to_string(), Value::NativeFunction {
            name: "random_int".to_string(),
            arity: 2,
        });

        self.define("random_float".to_string(), Value::NativeFunction {
            name: "random_float".to_string(),
            arity: 2,
        });

        // String utility functions
        self.define("substr".to_string(), Value::NativeFunction {
            name: "substr".to_string(),
            arity: 3,
        });
        
        self.define("upper".to_string(), Value::NativeFunction {
            name: "upper".to_string(),
            arity: 1,
        });
        
        self.define("lower".to_string(), Value::NativeFunction {
            name: "lower".to_string(),
            arity: 1,
        });
        
        self.define("trim".to_string(), Value::NativeFunction {
            name: "trim".to_string(),
            arity: 1,
        });
        
        self.define("split".to_string(), Value::NativeFunction {
            name: "split".to_string(),
            arity: 2,
        });
        
        self.define("join".to_string(), Value::NativeFunction {
            name: "join".to_string(),
            arity: 2,
        });
        
        self.define("contains".to_string(), Value::NativeFunction {
            name: "contains".to_string(),
            arity: 2,
        });

        // Array utility functions
        self.define("reverse".to_string(), Value::NativeFunction {
            name: "reverse".to_string(),
            arity: 1,
        });
        
        self.define("sort".to_string(), Value::NativeFunction {
            name: "sort".to_string(),
            arity: 1,
        });

        // Time functions
        self.define("now".to_string(), Value::NativeFunction {
            name: "now".to_string(),
            arity: 0,
        });

        self.define("sleep".to_string(), Value::NativeFunction {
            name: "sleep".to_string(),
            arity: 1,
        });

        // File I/O functions
        self.define("exists".to_string(), Value::NativeFunction {
            name: "exists".to_string(),
            arity: 1,
        });

        // Enhanced File I/O functions
        self.define("read_dir".to_string(), Value::NativeFunction {
            name: "read_dir".to_string(),
            arity: 1,
        });

        self.define("create_dir".to_string(), Value::NativeFunction {
            name: "create_dir".to_string(),
            arity: 1,
        });

        self.define("remove_file".to_string(), Value::NativeFunction {
            name: "remove_file".to_string(),
            arity: 1,
        });

        self.define("remove_dir".to_string(), Value::NativeFunction {
            name: "remove_dir".to_string(),
            arity: 1,
        });

        self.define("copy_file".to_string(), Value::NativeFunction {
            name: "copy_file".to_string(),
            arity: 2,
        });

        self.define("move_file".to_string(), Value::NativeFunction {
            name: "move_file".to_string(),
            arity: 2,
        });

        self.define("file_size".to_string(), Value::NativeFunction {
            name: "file_size".to_string(),
            arity: 1,
        });

        self.define("file_modified".to_string(), Value::NativeFunction {
            name: "file_modified".to_string(),
            arity: 1,
        });

        self.define("is_file".to_string(), Value::NativeFunction {
            name: "is_file".to_string(),
            arity: 1,
        });

        self.define("is_dir".to_string(), Value::NativeFunction {
            name: "is_dir".to_string(),
            arity: 1,
        });

        // Path manipulation functions
        self.define("join_path".to_string(), Value::NativeFunction {
            name: "join_path".to_string(),
            arity: 2,
        });

        self.define("dirname".to_string(), Value::NativeFunction {
            name: "dirname".to_string(),
            arity: 1,
        });

        self.define("basename".to_string(), Value::NativeFunction {
            name: "basename".to_string(),
            arity: 1,
        });

        self.define("extension".to_string(), Value::NativeFunction {
            name: "extension".to_string(),
            arity: 1,
        });

        self.define("absolute_path".to_string(), Value::NativeFunction {
            name: "absolute_path".to_string(),
            arity: 1,
        });

        // Environment variables
        self.define("env_get".to_string(), Value::NativeFunction {
            name: "env_get".to_string(),
            arity: 1,
        });

        self.define("env_set".to_string(), Value::NativeFunction {
            name: "env_set".to_string(),
            arity: 2,
        });

        self.define("env_list".to_string(), Value::NativeFunction {
            name: "env_list".to_string(),
            arity: 0,
        });

        // System information
        self.define("os_name".to_string(), Value::NativeFunction {
            name: "os_name".to_string(),
            arity: 0,
        });

        self.define("home_dir".to_string(), Value::NativeFunction {
            name: "home_dir".to_string(),
            arity: 0,
        });

        self.define("current_dir".to_string(), Value::NativeFunction {
            name: "current_dir".to_string(),
            arity: 0,
        });

        self.define("temp_dir".to_string(), Value::NativeFunction {
            name: "temp_dir".to_string(),
            arity: 0,
        });

        // Process functions
        self.define("exit".to_string(), Value::NativeFunction {
            name: "exit".to_string(),
            arity: 1,
        });

        self.define("exec".to_string(), Value::NativeFunction {
            name: "exec".to_string(),
            arity: 1,
        });

        self.define("spawn".to_string(), Value::NativeFunction {
            name: "spawn".to_string(),
            arity: 2,
        });

        // Regex functions
        self.define("regex_match".to_string(), Value::NativeFunction {
            name: "regex_match".to_string(),
            arity: 2,
        });
        
        self.define("regex_find".to_string(), Value::NativeFunction {
            name: "regex_find".to_string(),
            arity: 2,
        });
        
        self.define("regex_replace".to_string(), Value::NativeFunction {
            name: "regex_replace".to_string(),
            arity: 3,
        });
        
        self.define("regex_split".to_string(), Value::NativeFunction {
            name: "regex_split".to_string(),
            arity: 2,
        });
        
        self.define("regex_find_all".to_string(), Value::NativeFunction {
            name: "regex_find_all".to_string(),
            arity: 2,
        });

        // Encoding/Decoding functions
        self.define("base64_encode".to_string(), Value::NativeFunction {
            name: "base64_encode".to_string(),
            arity: 1,
        });

        self.define("base64_decode".to_string(), Value::NativeFunction {
            name: "base64_decode".to_string(),
            arity: 1,
        });

        self.define("url_encode".to_string(), Value::NativeFunction {
            name: "url_encode".to_string(),
            arity: 1,
        });

        self.define("url_decode".to_string(), Value::NativeFunction {
            name: "url_decode".to_string(),
            arity: 1,
        });

        self.define("hex_encode".to_string(), Value::NativeFunction {
            name: "hex_encode".to_string(),
            arity: 1,
        });

        self.define("hex_decode".to_string(), Value::NativeFunction {
            name: "hex_decode".to_string(),
            arity: 1,
        });

        // Hash functions
        self.define("sha256".to_string(), Value::NativeFunction {
            name: "sha256".to_string(),
            arity: 1,
        });

        self.define("md5".to_string(), Value::NativeFunction {
            name: "md5".to_string(),
            arity: 1,
        });

        // Date/Time functions (enhanced)
        self.define("format_time".to_string(), Value::NativeFunction {
            name: "format_time".to_string(),
            arity: 2,
        });

        self.define("parse_time".to_string(), Value::NativeFunction {
            name: "parse_time".to_string(),
            arity: 2,
        });

        self.define("timestamp".to_string(), Value::NativeFunction {
            name: "timestamp".to_string(),
            arity: 0,
        });

        self.define("timezone".to_string(), Value::NativeFunction {
            name: "timezone".to_string(),
            arity: 0,
        });

        // Validation functions
        self.define("is_email".to_string(), Value::NativeFunction {
            name: "is_email".to_string(),
            arity: 1,
        });

        self.define("is_url".to_string(), Value::NativeFunction {
            name: "is_url".to_string(),
            arity: 1,
        });

        self.define("is_numeric".to_string(), Value::NativeFunction {
            name: "is_numeric".to_string(),
            arity: 1,
        });

        self.define("is_alpha".to_string(), Value::NativeFunction {
            name: "is_alpha".to_string(),
            arity: 1,
        });

        self.define("is_alphanumeric".to_string(), Value::NativeFunction {
            name: "is_alphanumeric".to_string(),
            arity: 1,
        });

        // Utility functions
        self.define("uuid".to_string(), Value::NativeFunction {
            name: "uuid".to_string(),
            arity: 0,
        });

        self.define("range".to_string(), Value::NativeFunction {
            name: "range".to_string(),
            arity: 3,
        });

        self.define("enumerate".to_string(), Value::NativeFunction {
            name: "enumerate".to_string(),
            arity: 1,
        });

        self.define("zip".to_string(), Value::NativeFunction {
            name: "zip".to_string(),
            arity: 2,
        });

        self.define("any".to_string(), Value::NativeFunction {
            name: "any".to_string(),
            arity: 1,
        });

        self.define("all".to_string(), Value::NativeFunction {
            name: "all".to_string(),
            arity: 1,
        });

        self.define("sum".to_string(), Value::NativeFunction {
            name: "sum".to_string(),
            arity: 1,
        });

        self.define("product".to_string(), Value::NativeFunction {
            name: "product".to_string(),
            arity: 1,
        });

        self.define("average".to_string(), Value::NativeFunction {
            name: "average".to_string(),
            arity: 1,
        });

        self.define("median".to_string(), Value::NativeFunction {
            name: "median".to_string(),
            arity: 1,
        });

        // Map and Set constructors
        self.define("Map".to_string(), Value::NativeFunction {
            name: "Map".to_string(),
            arity: 0,
        });

        self.define("Set".to_string(), Value::NativeFunction {
            name: "Set".to_string(),
            arity: 0,
        });

        // Promise constructor and utilities
        self.define("Promise".to_string(), Value::NativeFunction {
            name: "Promise".to_string(),
            arity: 1,
        });

        self.define("Promise_resolve".to_string(), Value::NativeFunction {
            name: "Promise_resolve".to_string(),
            arity: 1,
        });

        self.define("Promise_reject".to_string(), Value::NativeFunction {
            name: "Promise_reject".to_string(),
            arity: 1,
        });

        self.define("Promise_all".to_string(), Value::NativeFunction {
            name: "Promise_all".to_string(),
            arity: 1,
        });

        self.define("Promise_race".to_string(), Value::NativeFunction {
            name: "Promise_race".to_string(),
            arity: 1,
        });
    }

    pub fn get_all_variables(&self) -> std::collections::HashMap<String, Value> {
        self.vars.clone()
    }
}