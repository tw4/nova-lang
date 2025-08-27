use std::collections::HashMap;
use crate::value::*;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub path: Option<String>,
    pub exports: HashMap<String, Value>,
    pub dependencies: Vec<String>,
}

impl Module {
    pub fn new(name: String) -> Self {
        Module {
            name,
            path: None,
            exports: HashMap::new(),
            dependencies: Vec::new(),
        }
    }

    pub fn with_path(name: String, path: String) -> Self {
        Module {
            name,
            path: Some(path),
            exports: HashMap::new(),
            dependencies: Vec::new(),
        }
    }

    pub fn export(&mut self, name: String, value: Value) {
        self.exports.insert(name, value);
    }

    pub fn add_dependency(&mut self, module_name: String) {
        if !self.dependencies.contains(&module_name) {
            self.dependencies.push(module_name);
        }
    }

    pub fn get_export(&self, name: &str) -> Option<&Value> {
        self.exports.get(name)
    }
}

#[derive(Debug)]
pub struct ModuleRegistry {
    modules: HashMap<String, Module>,
    module_paths: HashMap<String, String>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        ModuleRegistry {
            modules: HashMap::new(),
            module_paths: HashMap::new(),
        }
    }

    pub fn register_module(&mut self, module: Module) -> Result<(), String> {
        let name = module.name.clone();
        
        if self.modules.contains_key(&name) {
            return Err(format!("Module '{}' already registered", name));
        }

        if let Some(path) = &module.path {
            self.module_paths.insert(name.clone(), path.clone());
        }

        self.modules.insert(name, module);
        Ok(())
    }

    pub fn get_module(&self, name: &str) -> Option<&Module> {
        self.modules.get(name)
    }

    pub fn get_module_mut(&mut self, name: &str) -> Option<&mut Module> {
        self.modules.get_mut(name)
    }

    pub fn resolve_import(&self, module_name: &str, import_name: &str) -> Option<Value> {
        self.get_module(module_name)
            .and_then(|module| module.get_export(import_name))
            .cloned()
    }

    pub fn list_modules(&self) -> Vec<&String> {
        self.modules.keys().collect()
    }

    pub fn get_module_path(&self, name: &str) -> Option<&String> {
        self.module_paths.get(name)
    }

    pub fn create_standard_modules(&mut self) {
        self.create_math_module();
        self.create_string_module();
        self.create_array_module();
        self.create_io_module();
        self.create_json_module();
        self.create_http_module();
        self.create_fs_module();
    }

    fn create_math_module(&mut self) {
        let mut math_module = Module::new("math".to_string());
        
        math_module.export("PI".to_string(), Value::Number(std::f64::consts::PI));
        math_module.export("E".to_string(), Value::Number(std::f64::consts::E));
        
        math_module.export("abs".to_string(), Value::NativeFunction {
            name: "abs".to_string(),
            arity: 1,
        });
        
        math_module.export("sqrt".to_string(), Value::NativeFunction {
            name: "sqrt".to_string(),
            arity: 1,
        });
        
        math_module.export("pow".to_string(), Value::NativeFunction {
            name: "pow".to_string(),
            arity: 2,
        });
        
        math_module.export("sin".to_string(), Value::NativeFunction {
            name: "sin".to_string(),
            arity: 1,
        });
        
        math_module.export("cos".to_string(), Value::NativeFunction {
            name: "cos".to_string(),
            arity: 1,
        });
        
        math_module.export("tan".to_string(), Value::NativeFunction {
            name: "tan".to_string(),
            arity: 1,
        });
        
        math_module.export("floor".to_string(), Value::NativeFunction {
            name: "floor".to_string(),
            arity: 1,
        });
        
        math_module.export("ceil".to_string(), Value::NativeFunction {
            name: "ceil".to_string(),
            arity: 1,
        });
        
        math_module.export("round".to_string(), Value::NativeFunction {
            name: "round".to_string(),
            arity: 1,
        });
        
        math_module.export("min".to_string(), Value::NativeFunction {
            name: "min".to_string(),
            arity: 2,
        });
        
        math_module.export("max".to_string(), Value::NativeFunction {
            name: "max".to_string(),
            arity: 2,
        });
        
        math_module.export("random".to_string(), Value::NativeFunction {
            name: "random".to_string(),
            arity: 0,
        });

        self.register_module(math_module).expect("Failed to register math module");
    }

    fn create_string_module(&mut self) {
        let mut string_module = Module::new("string".to_string());
        
        string_module.export("upper".to_string(), Value::NativeFunction {
            name: "upper".to_string(),
            arity: 1,
        });
        
        string_module.export("lower".to_string(), Value::NativeFunction {
            name: "lower".to_string(),
            arity: 1,
        });
        
        string_module.export("trim".to_string(), Value::NativeFunction {
            name: "trim".to_string(),
            arity: 1,
        });
        
        string_module.export("split".to_string(), Value::NativeFunction {
            name: "split".to_string(),
            arity: 2,
        });
        
        string_module.export("join".to_string(), Value::NativeFunction {
            name: "join".to_string(),
            arity: 2,
        });
        
        string_module.export("contains".to_string(), Value::NativeFunction {
            name: "contains".to_string(),
            arity: 2,
        });
        
        string_module.export("substring".to_string(), Value::NativeFunction {
            name: "substring".to_string(),
            arity: 3,
        });
        
        string_module.export("replace".to_string(), Value::NativeFunction {
            name: "replace".to_string(),
            arity: 3,
        });
        
        string_module.export("indexOf".to_string(), Value::NativeFunction {
            name: "indexOf".to_string(),
            arity: 2,
        });

        self.register_module(string_module).expect("Failed to register string module");
    }

    fn create_array_module(&mut self) {
        let mut array_module = Module::new("array".to_string());
        
        array_module.export("push".to_string(), Value::NativeFunction {
            name: "push".to_string(),
            arity: 2,
        });
        
        array_module.export("pop".to_string(), Value::NativeFunction {
            name: "pop".to_string(),
            arity: 1,
        });
        
        array_module.export("length".to_string(), Value::NativeFunction {
            name: "length".to_string(),
            arity: 1,
        });
        
        array_module.export("reverse".to_string(), Value::NativeFunction {
            name: "reverse".to_string(),
            arity: 1,
        });
        
        array_module.export("sort".to_string(), Value::NativeFunction {
            name: "sort".to_string(),
            arity: 1,
        });
        
        array_module.export("filter".to_string(), Value::NativeFunction {
            name: "filter".to_string(),
            arity: 2,
        });
        
        array_module.export("map".to_string(), Value::NativeFunction {
            name: "map".to_string(),
            arity: 2,
        });
        
        array_module.export("reduce".to_string(), Value::NativeFunction {
            name: "reduce".to_string(),
            arity: 3,
        });
        
        array_module.export("forEach".to_string(), Value::NativeFunction {
            name: "forEach".to_string(),
            arity: 2,
        });

        self.register_module(array_module).expect("Failed to register array module");
    }

    fn create_io_module(&mut self) {
        let mut io_module = Module::new("io".to_string());
        
        io_module.export("print".to_string(), Value::NativeFunction {
            name: "print".to_string(),
            arity: 1,
        });
        
        io_module.export("println".to_string(), Value::NativeFunction {
            name: "println".to_string(),
            arity: 1,
        });
        
        io_module.export("input".to_string(), Value::NativeFunction {
            name: "input".to_string(),
            arity: 1,
        });

        self.register_module(io_module).expect("Failed to register io module");
    }

    fn create_json_module(&mut self) {
        let mut json_module = Module::new("json".to_string());
        
        json_module.export("parse".to_string(), Value::NativeFunction {
            name: "json_parse".to_string(),
            arity: 1,
        });
        
        json_module.export("stringify".to_string(), Value::NativeFunction {
            name: "json_stringify".to_string(),
            arity: 1,
        });

        self.register_module(json_module).expect("Failed to register json module");
    }

    fn create_http_module(&mut self) {
        let mut http_module = Module::new("http".to_string());
        
        http_module.export("get".to_string(), Value::NativeFunction {
            name: "http_get".to_string(),
            arity: 1,
        });
        
        http_module.export("post".to_string(), Value::NativeFunction {
            name: "http_post".to_string(),
            arity: 2,
        });
        
        http_module.export("put".to_string(), Value::NativeFunction {
            name: "http_put".to_string(),
            arity: 2,
        });
        
        http_module.export("delete".to_string(), Value::NativeFunction {
            name: "http_delete".to_string(),
            arity: 1,
        });

        self.register_module(http_module).expect("Failed to register http module");
    }

    fn create_fs_module(&mut self) {
        let mut fs_module = Module::new("fs".to_string());
        
        fs_module.export("readFile".to_string(), Value::NativeFunction {
            name: "read_file".to_string(),
            arity: 1,
        });
        
        fs_module.export("writeFile".to_string(), Value::NativeFunction {
            name: "write_file".to_string(),
            arity: 2,
        });
        
        fs_module.export("exists".to_string(), Value::NativeFunction {
            name: "exists".to_string(),
            arity: 1,
        });
        
        fs_module.export("mkdir".to_string(), Value::NativeFunction {
            name: "mkdir".to_string(),
            arity: 1,
        });
        
        fs_module.export("rmdir".to_string(), Value::NativeFunction {
            name: "rmdir".to_string(),
            arity: 1,
        });
        
        fs_module.export("ls".to_string(), Value::NativeFunction {
            name: "ls".to_string(),
            arity: 1,
        });

        self.register_module(fs_module).expect("Failed to register fs module");
    }
}

#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub module_name: String,
    pub imports: ImportSpecification,
    pub alias: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ImportSpecification {
    All,
    Named(Vec<String>),
    Default(String),
}

impl ImportStatement {
    pub fn all(module_name: String) -> Self {
        ImportStatement {
            module_name,
            imports: ImportSpecification::All,
            alias: None,
        }
    }

    pub fn named(module_name: String, names: Vec<String>) -> Self {
        ImportStatement {
            module_name,
            imports: ImportSpecification::Named(names),
            alias: None,
        }
    }

    pub fn default(module_name: String, default_name: String) -> Self {
        ImportStatement {
            module_name,
            imports: ImportSpecification::Default(default_name),
            alias: None,
        }
    }

    pub fn with_alias(mut self, alias: String) -> Self {
        self.alias = Some(alias);
        self
    }
}

#[derive(Debug)]
pub struct ModuleLoader {
    registry: ModuleRegistry,
    search_paths: Vec<String>,
}

impl ModuleLoader {
    pub fn new() -> Self {
        let mut loader = ModuleLoader {
            registry: ModuleRegistry::new(),
            search_paths: vec!["./".to_string(), "./modules/".to_string()],
        };
        
        loader.registry.create_standard_modules();
        loader
    }

    pub fn add_search_path(&mut self, path: String) {
        if !self.search_paths.contains(&path) {
            self.search_paths.push(path);
        }
    }

    pub fn load_module(&mut self, module_name: &str) -> Result<(), String> {
        if self.registry.get_module(module_name).is_some() {
            return Ok(()); // Already loaded
        }

        let module_path = self.find_module_file(module_name)?;
        let module_content = std::fs::read_to_string(&module_path)
            .map_err(|e| format!("Failed to read module '{}': {}", module_name, e))?;

        let module = self.parse_module(module_name, &module_path, &module_content)?;
        self.registry.register_module(module)?;

        Ok(())
    }

    fn find_module_file(&self, module_name: &str) -> Result<String, String> {
        for search_path in &self.search_paths {
            let potential_path = format!("{}/{}.nova", search_path, module_name);
            if std::path::Path::new(&potential_path).exists() {
                return Ok(potential_path);
            }
        }

        Err(format!("Module '{}' not found in search paths", module_name))
    }

    fn parse_module(&self, module_name: &str, module_path: &str, _content: &str) -> Result<Module, String> {
        // This would integrate with the existing parser
        // For now, we'll create a basic module
        let module = Module::with_path(module_name.to_string(), module_path.to_string());
        
        // TODO: Parse the actual content and extract exports
        // This would involve parsing the Nova code and identifying export statements
        
        Ok(module)
    }

    pub fn get_registry(&self) -> &ModuleRegistry {
        &self.registry
    }

    pub fn get_registry_mut(&mut self) -> &mut ModuleRegistry {
        &mut self.registry
    }

    pub fn resolve_import(&mut self, import: &ImportStatement) -> Result<HashMap<String, Value>, String> {
        self.load_module(&import.module_name)?;
        
        let mut resolved = HashMap::new();
        
        let module = self.registry.get_module(&import.module_name)
            .ok_or_else(|| format!("Module '{}' not found", import.module_name))?;

        match &import.imports {
            ImportSpecification::All => {
                for (name, value) in &module.exports {
                    resolved.insert(name.clone(), value.clone());
                }
            },
            ImportSpecification::Named(names) => {
                for name in names {
                    if let Some(value) = module.get_export(name) {
                        resolved.insert(name.clone(), value.clone());
                    } else {
                        return Err(format!("Export '{}' not found in module '{}'", name, import.module_name));
                    }
                }
            },
            ImportSpecification::Default(name) => {
                if let Some(value) = module.get_export("default") {
                    resolved.insert(name.clone(), value.clone());
                } else {
                    return Err(format!("No default export found in module '{}'", import.module_name));
                }
            }
        }

        Ok(resolved)
    }
}