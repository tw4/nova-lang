use crate::ast::Expr;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
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
            Value::Array(arr) => !arr.is_empty(),
            Value::Object(obj) => !obj.is_empty(),
            Value::Function { .. } | Value::NativeFunction { .. } => true,
            Value::Class { .. } => true,
            Value::Instance { .. } => true,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Function { .. } => "function",
            Value::NativeFunction { .. } => "function",
            Value::Class { .. } => "class",
            Value::Instance { .. } => "instance",
            Value::Null => "null",
        }
    }

    pub fn is_callable(&self) -> bool {
        matches!(self, Value::Function { .. } | Value::NativeFunction { .. } | Value::Class { .. })
    }

    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Value::Number(n) => serde_json::json!(*n),
            Value::String(s) => serde_json::json!(s),
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
            Value::Function { params, .. } => {
                write!(f, "<function({})>", params.join(", "))
            }
            Value::NativeFunction { name, arity } => {
                write!(f, "<native function {}({} args)>", name, arity)
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
    }

    pub fn get_all_variables(&self) -> std::collections::HashMap<String, Value> {
        self.vars.clone()
    }
}