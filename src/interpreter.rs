use crate::ast::*;
use std::collections::HashMap;
use serde_json::Value as JsonValue;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, String)>, // (type, name)
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Sankhya(f64),
    Aksharalu(String),
    Nijam,
    Abaddham,
    Sunya,
    Array(Vec<Value>),
    HttpResponse(String), // JSON response as string
}

impl Value {
    fn to_string(&self) -> String {
        match self {
            Value::Sankhya(n) => n.to_string(),
            Value::Aksharalu(s) => s.clone(),
            Value::Nijam => "nijam".to_string(),
            Value::Abaddham => "abaddham".to_string(),
            Value::Sunya => "sunya".to_string(),
            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", elements.join(", "))
            }
            Value::HttpResponse(response) => response.clone(),
        }
    }
    
    fn is_truthy(&self) -> bool {
        match self {
            Value::Abaddham => false,
            Value::Sunya => false,
            Value::Array(arr) => !arr.is_empty(),
            Value::HttpResponse(response) => !response.is_empty(),
            _ => true,
        }
    }
}

pub struct Interpreter {
    globals: HashMap<String, Value>,
    locals: Vec<HashMap<String, Value>>,
    functions: HashMap<String, Function>,
}

#[derive(Debug)]
pub enum RuntimeError {
    UndefinedVariable(String),
    TypeError(String),
    Return(Value),
    Break,
    Continue,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            globals: HashMap::new(),
            locals: Vec::new(),
            functions: HashMap::new(),
        }
    }
    
    pub fn interpret(&mut self, program: &Program) -> Result<(), RuntimeError> {
        for statement in &program.statements {
            self.execute_statement(statement)?;
        }
        Ok(())
    }
    
    fn execute_statement(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate_expression(expr)?;
                Ok(())
            }
            
            Stmt::VarDecl { var_type: _, name, initializer } => {
                let value = if let Some(init) = initializer {
                    self.evaluate_expression(init)?
                } else {
                    Value::Sunya
                };
                self.set_variable(name.clone(), value);
                Ok(())
            }
            
            Stmt::Ayite { condition, then_branch, else_branch } => {
                let condition_value = self.evaluate_expression(condition)?;
                
                if condition_value.is_truthy() {
                    self.execute_statement(then_branch)?;
                } else if let Some(else_stmt) = else_branch {
                    self.execute_statement(else_stmt)?;
                }
                Ok(())
            }
            
            Stmt::Varaku { condition, body } => {
                loop {
                    let condition_value = self.evaluate_expression(condition)?;
                    if !condition_value.is_truthy() {
                        break;
                    }
                    
                    match self.execute_statement(body) {
                        Err(RuntimeError::Break) => break,
                        Err(RuntimeError::Continue) => continue,
                        Err(e) => return Err(e),
                        Ok(_) => {}
                    }
                }
                Ok(())
            }
            
            Stmt::Prathi { variable, iterable, body } => {
                // Evaluate the iterable expression
                let iter_value = self.evaluate_expression(iterable)?;
                
                // For now, we'll support numeric ranges (e.g., 1..10 represented as two numbers)
                // This is a simplified implementation - in a full language you'd support arrays, etc.
                match iter_value {
                    Value::Sankhya(end) => {
                        // Simple range from 0 to end
                        let start = 0.0;
                        let mut i = start;
                        
                        while i < end {
                            // Set the loop variable
                            self.set_variable(variable.clone(), Value::Sankhya(i));
                            
                            // Execute the body
                            match self.execute_statement(body) {
                                Err(RuntimeError::Break) => break,
                                Err(RuntimeError::Continue) => {
                                    i += 1.0;
                                    continue;
                                }
                                Err(e) => return Err(e),
                                Ok(_) => {}
                            }
                            
                            i += 1.0;
                        }
                    }
                    _ => return Err(RuntimeError::TypeError(
                        "For loop iterable must be a number (range)".to_string()
                    )),
                }
                Ok(())
            }
            
            Stmt::Block(statements) => {
                self.locals.push(HashMap::new());
                
                let result = statements.iter().try_for_each(|stmt| self.execute_statement(stmt));
                
                self.locals.pop();
                result
            }
            
            Stmt::Pani { name, params, return_type: _, body } => {
                let function = Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                };
                self.functions.insert(name.clone(), function);
                Ok(())
            }
            
            Stmt::TirigiPampu(expr) => {
                let value = if let Some(e) = expr {
                    self.evaluate_expression(e)?
                } else {
                    Value::Sunya
                };
                Err(RuntimeError::Return(value))
            }
            
            Stmt::Aagu => Err(RuntimeError::Break),
            
            Stmt::Kalugu => Err(RuntimeError::Continue),
            
            Stmt::Cheppu(expr) => {
                let value = self.evaluate_expression(expr)?;
                println!("{}", value.to_string());
                Ok(())
            }
            
            Stmt::Adugu { prompt, variable } => {
                if let Some(p) = prompt {
                    print!("{}", p);
                }
                
                use std::io::{self, Write};
                io::stdout().flush().unwrap();
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim().to_string();
                
                // Try to parse as number first, otherwise store as string
                let value = if let Ok(num) = input.parse::<f64>() {
                    Value::Sankhya(num)
                } else {
                    Value::Aksharalu(input)
                };
                
                self.set_variable(variable.clone(), value);
                Ok(())
            }
            
            _ => Ok(()),
        }
    }
    
    fn evaluate_expression(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Sankhya(n) => Ok(Value::Sankhya(*n)),
            Expr::Aksharalu(s) => Ok(Value::Aksharalu(s.clone())),
            Expr::Nijam => Ok(Value::Nijam),
            Expr::Abaddham => Ok(Value::Abaddham),
            Expr::Sunya => Ok(Value::Sunya),
            
            Expr::Variable(name) => self.get_variable(name),
            
            Expr::Binary { left, operator, right } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                
                match (operator, &left_val, &right_val) {
                    (BinaryOp::Plus, Value::Sankhya(a), Value::Sankhya(b)) => {
                        Ok(Value::Sankhya(a + b))
                    }
                    (BinaryOp::Plus, Value::Aksharalu(a), Value::Aksharalu(b)) => {
                        Ok(Value::Aksharalu(format!("{}{}", a, b)))
                    }
                    (BinaryOp::Plus, Value::Aksharalu(a), Value::Sankhya(b)) => {
                        Ok(Value::Aksharalu(format!("{}{}", a, b)))
                    }
                    (BinaryOp::Plus, Value::Sankhya(a), Value::Aksharalu(b)) => {
                        Ok(Value::Aksharalu(format!("{}{}", a, b)))
                    }
                    (BinaryOp::Plus, Value::Aksharalu(a), Value::Nijam) => {
                        Ok(Value::Aksharalu(format!("{}nijam", a)))
                    }
                    (BinaryOp::Plus, Value::Aksharalu(a), Value::Abaddham) => {
                        Ok(Value::Aksharalu(format!("{}abaddham", a)))
                    }
                    (BinaryOp::Plus, Value::Nijam, Value::Aksharalu(b)) => {
                        Ok(Value::Aksharalu(format!("nijam{}", b)))
                    }
                    (BinaryOp::Plus, Value::Abaddham, Value::Aksharalu(b)) => {
                        Ok(Value::Aksharalu(format!("abaddham{}", b)))
                    }
                    (BinaryOp::Plus, Value::Sankhya(a), Value::Nijam) => {
                        Ok(Value::Aksharalu(format!("{}nijam", a)))
                    }
                    (BinaryOp::Plus, Value::Sankhya(a), Value::Abaddham) => {
                        Ok(Value::Aksharalu(format!("{}abaddham", a)))
                    }
                    (BinaryOp::Plus, Value::Nijam, Value::Sankhya(b)) => {
                        Ok(Value::Aksharalu(format!("nijam{}", b)))
                    }
                    (BinaryOp::Plus, Value::Abaddham, Value::Sankhya(b)) => {
                        Ok(Value::Aksharalu(format!("abaddham{}", b)))
                    }
                    (BinaryOp::Plus, Value::Aksharalu(a), Value::HttpResponse(b)) => {
                        Ok(Value::Aksharalu(format!("{}{}", a, b)))
                    }
                    (BinaryOp::Plus, Value::HttpResponse(a), Value::Aksharalu(b)) => {
                        Ok(Value::Aksharalu(format!("{}{}", a, b)))
                    }
                    (BinaryOp::Minus, Value::Sankhya(a), Value::Sankhya(b)) => {
                        Ok(Value::Sankhya(a - b))
                    }
                    (BinaryOp::Multiply, Value::Sankhya(a), Value::Sankhya(b)) => {
                        Ok(Value::Sankhya(a * b))
                    }
                    (BinaryOp::Divide, Value::Sankhya(a), Value::Sankhya(b)) => {
                        if *b == 0.0 {
                            Err(RuntimeError::TypeError("Division by zero".to_string()))
                        } else {
                            Ok(Value::Sankhya(a / b))
                        }
                    }
                    (BinaryOp::Modulo, Value::Sankhya(a), Value::Sankhya(b)) => {
                        Ok(Value::Sankhya(a % b))
                    }
                    (BinaryOp::Less, Value::Sankhya(a), Value::Sankhya(b)) => {
                        Ok(if a < b { Value::Nijam } else { Value::Abaddham })
                    }
                    (BinaryOp::Greater, Value::Sankhya(a), Value::Sankhya(b)) => {
                        Ok(if a > b { Value::Nijam } else { Value::Abaddham })
                    }
                    (BinaryOp::LessEqual, Value::Sankhya(a), Value::Sankhya(b)) => {
                        Ok(if a <= b { Value::Nijam } else { Value::Abaddham })
                    }
                    (BinaryOp::GreaterEqual, Value::Sankhya(a), Value::Sankhya(b)) => {
                        Ok(if a >= b { Value::Nijam } else { Value::Abaddham })
                    }
                    (BinaryOp::Equal, _, _) => {
                        Ok(if self.values_equal(&left_val, &right_val) {
                            Value::Nijam
                        } else {
                            Value::Abaddham
                        })
                    }
                    (BinaryOp::NotEqual, _, _) => {
                        Ok(if !self.values_equal(&left_val, &right_val) {
                            Value::Nijam
                        } else {
                            Value::Abaddham
                        })
                    }
                    (BinaryOp::Mariyu, _, _) => {
                        Ok(if left_val.is_truthy() && right_val.is_truthy() {
                            Value::Nijam
                        } else {
                            Value::Abaddham
                        })
                    }
                    (BinaryOp::Leda, _, _) => {
                        Ok(if left_val.is_truthy() || right_val.is_truthy() {
                            Value::Nijam
                        } else {
                            Value::Abaddham
                        })
                    }
                    _ => Err(RuntimeError::TypeError(
                        format!("Invalid operation {:?} for types", operator)
                    )),
                }
            }
            
            Expr::Unary { operator, operand } => {
                let value = self.evaluate_expression(operand)?;
                
                match (operator, value) {
                    (UnaryOp::Minus, Value::Sankhya(n)) => Ok(Value::Sankhya(-n)),
                    (UnaryOp::Kadu, v) => {
                        Ok(if v.is_truthy() { Value::Abaddham } else { Value::Nijam })
                    }
                    _ => Err(RuntimeError::TypeError(
                        "Invalid unary operation".to_string()
                    )),
                }
            }
            
            Expr::Assignment { name, value } => {
                let val = self.evaluate_expression(value)?;
                self.set_variable(name.clone(), val.clone());
                Ok(val)
            }
            
            Expr::Call { name, args } => {
                self.call_function(name, args)
            }
            
            Expr::Array(elements) => {
                let mut values = Vec::new();
                for element in elements {
                    values.push(self.evaluate_expression(element)?);
                }
                Ok(Value::Array(values))
            }
            
            Expr::Index { array, index } => {
                let array_value = self.evaluate_expression(array)?;
                let index_value = self.evaluate_expression(index)?;
                
                match (array_value, index_value) {
                    (Value::Array(arr), Value::Sankhya(idx)) => {
                        let index = idx as usize;
                        if index < arr.len() {
                            Ok(arr[index].clone())
                        } else {
                            Err(RuntimeError::TypeError(format!("Array index {} out of bounds", index)))
                        }
                    }
                    _ => Err(RuntimeError::TypeError("Invalid array indexing".to_string()))
                }
            }
        }
    }
    
    fn get_variable(&self, name: &str) -> Result<Value, RuntimeError> {
        // Check locals first (in reverse order)
        for scope in self.locals.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        
        // Then check globals
        self.globals.get(name)
            .cloned()
            .ok_or_else(|| RuntimeError::UndefinedVariable(name.to_string()))
    }
    
    fn set_variable(&mut self, name: String, value: Value) {
        // First check if variable exists in any local scope (from innermost to outermost)
        for scope in self.locals.iter_mut().rev() {
            if scope.contains_key(&name) {
                scope.insert(name, value);
                return;
            }
        }
        
        // Then check if it exists in globals
        if self.globals.contains_key(&name) {
            self.globals.insert(name, value);
            return;
        }
        
        // If not found anywhere, create new variable in current scope
        if let Some(scope) = self.locals.last_mut() {
            scope.insert(name, value);
        } else {
            self.globals.insert(name, value);
        }
    }
    
    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Sankhya(x), Value::Sankhya(y)) => x == y,
            (Value::Aksharalu(x), Value::Aksharalu(y)) => x == y,
            (Value::Nijam, Value::Nijam) => true,
            (Value::Abaddham, Value::Abaddham) => true,
            (Value::Sunya, Value::Sunya) => true,
            (Value::Array(a), Value::Array(b)) => {
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| self.values_equal(x, y))
            }
            (Value::HttpResponse(a), Value::HttpResponse(b)) => a == b,
            _ => false,
        }
    }
    
    fn call_function(&mut self, name: &str, args: &[Expr]) -> Result<Value, RuntimeError> {
        // Check for built-in HTTP functions first
        match name {
            "fetch" => return self.http_fetch(args),
            "fetch_json" => return self.http_fetch_json(args),
            "post" => return self.http_post(args),
            "put" => return self.http_put(args),
            "delete" => return self.http_delete(args),
            _ => {}
        }
        
        // Look up user-defined function
        let function = self.functions.get(name).cloned()
            .ok_or_else(|| RuntimeError::TypeError(format!("Undefined function '{}'", name)))?;
        
        // Evaluate arguments
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.evaluate_expression(arg)?);
        }
        
        // Check parameter count
        if arg_values.len() != function.params.len() {
            return Err(RuntimeError::TypeError(
                format!("Function '{}' expects {} arguments, got {}", 
                       name, function.params.len(), arg_values.len())
            ));
        }
        
        // Create new scope for function execution
        self.locals.push(HashMap::new());
        
        // Bind parameters to arguments
        for (i, (_, param_name)) in function.params.iter().enumerate() {
            self.set_variable(param_name.clone(), arg_values[i].clone());
        }
        
        // Execute function body
        let result = match self.execute_statement(&function.body) {
            Ok(_) => Ok(Value::Sunya), // No explicit return, return null
            Err(RuntimeError::Return(value)) => Ok(value), // Explicit return
            Err(e) => Err(e), // Other errors
        };
        
        // Clean up scope
        self.locals.pop();
        
        result
    }
    
    // HTTP Methods for REST API functionality
    fn http_fetch(&mut self, args: &[Expr]) -> Result<Value, RuntimeError> {
        if args.len() != 1 {
            return Err(RuntimeError::TypeError(
                "fetch() expects exactly 1 argument (URL)".to_string()
            ));
        }
        
        let url_value = self.evaluate_expression(&args[0])?;
        let url = match url_value {
            Value::Aksharalu(s) => s,
            _ => return Err(RuntimeError::TypeError(
                "fetch() URL must be a string".to_string()
            )),
        };
        
        // Make HTTP GET request using blocking reqwest
        match reqwest::blocking::get(&url) {
            Ok(response) => {
                match response.text() {
                    Ok(text) => Ok(Value::HttpResponse(text)),
                    Err(_) => Err(RuntimeError::TypeError(
                        "Failed to read response text".to_string()
                    )),
                }
            }
            Err(_) => Err(RuntimeError::TypeError(
                format!("Failed to fetch from URL: {}", url)
            )),
        }
    }
    
    fn http_fetch_json(&mut self, args: &[Expr]) -> Result<Value, RuntimeError> {
        if args.len() != 1 {
            return Err(RuntimeError::TypeError(
                "fetch_json() expects exactly 1 argument (URL)".to_string()
            ));
        }
        
        let url_value = self.evaluate_expression(&args[0])?;
        let url = match url_value {
            Value::Aksharalu(s) => s,
            _ => return Err(RuntimeError::TypeError(
                "fetch_json() URL must be a string".to_string()
            )),
        };
        
        // Make HTTP GET request and parse as JSON
        match reqwest::blocking::get(&url) {
            Ok(response) => {
                match response.json::<JsonValue>() {
                    Ok(json) => {
                        let json_string = serde_json::to_string_pretty(&json)
                            .unwrap_or_else(|_| "{}".to_string());
                        Ok(Value::HttpResponse(json_string))
                    }
                    Err(_) => Err(RuntimeError::TypeError(
                        "Failed to parse JSON response".to_string()
                    )),
                }
            }
            Err(_) => Err(RuntimeError::TypeError(
                format!("Failed to fetch JSON from URL: {}", url)
            )),
        }
    }
    
    fn http_post(&mut self, args: &[Expr]) -> Result<Value, RuntimeError> {
        if args.len() < 1 || args.len() > 2 {
            return Err(RuntimeError::TypeError(
                "post() expects 1-2 arguments (URL, optional body)".to_string()
            ));
        }
        
        let url_value = self.evaluate_expression(&args[0])?;
        let url = match url_value {
            Value::Aksharalu(s) => s,
            _ => return Err(RuntimeError::TypeError(
                "post() URL must be a string".to_string()
            )),
        };
        
        let client = reqwest::blocking::Client::new();
        let mut request = client.post(&url);
        
        // Add body if provided
        if args.len() == 2 {
            let body_value = self.evaluate_expression(&args[1])?;
            let body = match body_value {
                Value::Aksharalu(s) => s,
                Value::HttpResponse(s) => s,
                _ => return Err(RuntimeError::TypeError(
                    "post() body must be a string".to_string()
                )),
            };
            request = request.header("Content-Type", "application/json").body(body);
        }
        
        match request.send() {
            Ok(response) => {
                match response.text() {
                    Ok(text) => Ok(Value::HttpResponse(text)),
                    Err(_) => Err(RuntimeError::TypeError(
                        "Failed to read POST response".to_string()
                    )),
                }
            }
            Err(_) => Err(RuntimeError::TypeError(
                format!("Failed to POST to URL: {}", url)
            )),
        }
    }
    
    fn http_put(&mut self, args: &[Expr]) -> Result<Value, RuntimeError> {
        if args.len() < 1 || args.len() > 2 {
            return Err(RuntimeError::TypeError(
                "put() expects 1-2 arguments (URL, optional body)".to_string()
            ));
        }
        
        let url_value = self.evaluate_expression(&args[0])?;
        let url = match url_value {
            Value::Aksharalu(s) => s,
            _ => return Err(RuntimeError::TypeError(
                "put() URL must be a string".to_string()
            )),
        };
        
        let client = reqwest::blocking::Client::new();
        let mut request = client.put(&url);
        
        // Add body if provided
        if args.len() == 2 {
            let body_value = self.evaluate_expression(&args[1])?;
            let body = match body_value {
                Value::Aksharalu(s) => s,
                Value::HttpResponse(s) => s,
                _ => return Err(RuntimeError::TypeError(
                    "put() body must be a string".to_string()
                )),
            };
            request = request.header("Content-Type", "application/json").body(body);
        }
        
        match request.send() {
            Ok(response) => {
                match response.text() {
                    Ok(text) => Ok(Value::HttpResponse(text)),
                    Err(_) => Err(RuntimeError::TypeError(
                        "Failed to read PUT response".to_string()
                    )),
                }
            }
            Err(_) => Err(RuntimeError::TypeError(
                format!("Failed to PUT to URL: {}", url)
            )),
        }
    }
    
    fn http_delete(&mut self, args: &[Expr]) -> Result<Value, RuntimeError> {
        if args.len() != 1 {
            return Err(RuntimeError::TypeError(
                "delete() expects exactly 1 argument (URL)".to_string()
            ));
        }
        
        let url_value = self.evaluate_expression(&args[0])?;
        let url = match url_value {
            Value::Aksharalu(s) => s,
            _ => return Err(RuntimeError::TypeError(
                "delete() URL must be a string".to_string()
            )),
        };
        
        let client = reqwest::blocking::Client::new();
        
        match client.delete(&url).send() {
            Ok(response) => {
                match response.text() {
                    Ok(text) => Ok(Value::HttpResponse(text)),
                    Err(_) => Err(RuntimeError::TypeError(
                        "Failed to read DELETE response".to_string()
                    )),
                }
            }
            Err(_) => Err(RuntimeError::TypeError(
                format!("Failed to DELETE URL: {}", url)
            )),
        }
    }
}