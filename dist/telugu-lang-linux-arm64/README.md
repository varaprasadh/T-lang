# Telugu Programming Language

A programming language with Telugu keywords, designed to make coding more accessible to Telugu speakers.

## Features

- Variables and arithmetic operations
- Conditional statements (if/else)
- Loops (while and for)
- Arrays with literals and indexing
- Functions with parameters and return values
- Built-in REST API support (HTTP requests)
- JSON parsing and handling
- String and number support
- Built-in print functionality
- Telugu language keywords

## Telugu Keywords

| Telugu Keyword | English Equivalent | Description |
|----------------|-------------------|-------------|
| `sankhya` | number/int | Declare numeric variables |
| `cheppu` | print | Print output to console |
| `ayite` | if | Conditional statement |
| `lekapothe` | else | Else condition |
| `varaku` | while | While loop |
| `prathi` | for | For loop |
| `nunchi` | from | Used in for loops |
| `pani` | function | Function declaration |
| `tirigi_pampu` | return | Return from function |
| `nijam` | true | Boolean true value |
| `abaddham` | false | Boolean false value |
| `mariyu` | and | Logical AND operator |
| `leda` | or | Logical OR operator |
| `kadu` | not | Logical NOT operator |
| `fetch` | fetch | HTTP GET request |
| `fetch_json` | fetch_json | HTTP GET with JSON parsing |
| `post` | post | HTTP POST request |
| `put` | put | HTTP PUT request |
| `delete` | delete | HTTP DELETE request |

## Installation

### Prerequisites
- Rust 1.70+ installed on your system

### Install Telugu Compiler
```bash
# Clone and build
git clone <repository-url>
cd telugu-lang
./install.sh
```

### Manual Installation
```bash
cargo build --release
cp target/release/telc /usr/local/bin/  # or any directory in your PATH
```

## Usage

### Command Line Interface
```bash
# Basic usage (like javac for Java)
telc filename.tel

# With verbose output
telc filename.tel --verbose

# Show help
telc --help
```

### Example Programs

#### Hello World (`examples/hello.tel`)
```telugu
cheppu "Namaste! Telugu programming language ki swagathamu!";
cheppu "Welcome to Telugu Programming Language!";
```

#### Calculator (`examples/calculator.tel`)
```telugu
sankhya a = 15;
sankhya b = 25;

cheppu "Calculator Demo";
cheppu "================";

cheppu "First number: " + a;
cheppu "Second number: " + b;

sankhya sum = a + b;
cheppu "Addition: " + sum;
```

#### Loops (`examples/loops.tel`)
```telugu
// While loop
sankhya i = 1;
cheppu "Counting from 1 to 10:";
varaku (i <= 10) {
    cheppu "Count: " + i;
    i = i + 1;
}

// For loop
cheppu "For loop from 0 to 4:";
prathi (i nunchi 5) {
    cheppu "Count: " + i;
}
```

#### Conditionals (`examples/conditions.tel`)
```telugu
sankhya score = 85;
ayite (score >= 90) {
    cheppu "Grade: A+ (Excellent!)";
} lekapothe {
    ayite (score >= 80) {
        cheppu "Grade: A (Very Good!)";
    } lekapothe {
        cheppu "Grade: B (Good!)";
    }
}
```

#### Functions (`examples/functions-working.tel`)
```telugu
// Function definition
pani add(sankhya a, sankhya b) {
    sankhya result = a + b;
    tirigi_pampu result;
}

// Function call
sankhya sum = add(5, 3);
cheppu "Sum: " + sum; // Output: Sum: 8

// Function with conditional logic
pani max(sankhya a, sankhya b) {
    ayite (a > b) {
        tirigi_pampu a;
    } lekapothe {
        tirigi_pampu b;
    }
}
```

#### Real-world REST API Example (`examples/rest-api-complete-example.tel`)
```telugu
// Complete User Management System
sankhya baseUrl = "https://jsonplaceholder.typicode.com";

// Utility function to build API URLs
pani buildUrl(sankhya base, sankhya endpoint, sankhya id) {
    sankhya url = base + endpoint;
    ayite (id > 0) {
        url = url + "/" + id;
    }
    tirigi_pampu url;
}

// Get user by ID
pani getUserById(sankhya userId) {
    sankhya url = buildUrl(baseUrl, "/users", userId);
    cheppu "GET " + url;
    sankhya response = fetch_json(url);
    tirigi_pampu response;
}

// Create new user
pani createUser(sankhya name, sankhya email) {
    sankhya url = buildUrl(baseUrl, "/users", 0);
    sankhya userData = "{\"name\": \"" + name + "\", \"email\": \"" + email + "\"}";
    sankhya response = post(url, userData);
    tirigi_pampu response;
}

// Usage
sankhya user = getUserById(1);
sankhya newUser = createUser("Telugu Developer", "dev@telugu.com");
```

## Language Syntax

### Variables
```telugu
sankhya x = 10;        // Number variable
sankhya name = "Hello"; // String variable (auto-detected)
```

### Arrays
```telugu
sankhya nums = [1, 2, 3, 4, 5];  // Array literal
sankhya first = nums[0];          // Array indexing
sankhya size = 5;                 // Array size (manual tracking)

// Arrays in functions
pani processArray(sankhya arr, sankhya size) {
    sankhya i = 0;
    varaku (i < size) {
        cheppu "Element: " + arr[i];
        i = i + 1;
    }
}
```

### Boolean Values
```telugu
sankhya isTrue = nijam;      // Boolean true
sankhya isFalse = abaddham;  // Boolean false

// Boolean operations
sankhya andResult = nijam mariyu abaddham;  // false
sankhya orResult = nijam leda abaddham;     // true
sankhya notResult = kadu nijam;             // false

// Boolean in conditions
ayite (nijam) {
    cheppu "This will execute";
}

ayite (abaddham) {
    cheppu "This won't execute";
} lekapothe {
    cheppu "This will execute instead";
}
```

### Arithmetic Operations
```telugu
sankhya result = a + b;  // Addition
sankhya result = a - b;  // Subtraction
sankhya result = a * b;  // Multiplication
sankhya result = a / b;  // Division
sankhya result = a % b;  // Modulo
```

### Comparison Operations
```telugu
ayite (x < y) { }        // Less than
ayite (x > y) { }        // Greater than
ayite (x <= y) { }       // Less than or equal
ayite (x >= y) { }       // Greater than or equal
ayite (x == y) { }       // Equal to
```

### Control Flow
```telugu
// If-else statement
ayite (condition) {
    // code block
} lekapothe {
    // else block
}

// While loop
varaku (condition) {
    // loop body
}

// For loop
prathi (variable nunchi range) {
    // loop body
}
```

### String Concatenation
```telugu
cheppu "Hello " + "World";    // String + String
cheppu "Number: " + 42;       // String + Number
```

### REST API Operations
```telugu
// GET request
sankhya response = fetch("https://api.example.com/data");
sankhya jsonData = fetch_json("https://api.example.com/users/1");

// POST request
sankhya postBody = "{\"name\": \"Telugu User\", \"email\": \"user@example.com\"}";
sankhya postResponse = post("https://api.example.com/users", postBody);

// PUT request
sankhya putBody = "{\"id\": 1, \"name\": \"Updated User\"}";
sankhya putResponse = put("https://api.example.com/users/1", putBody);

// DELETE request
sankhya deleteResponse = delete("https://api.example.com/users/1");

// Working with API responses
cheppu "API Response: " + jsonData;
```

## Running Examples

```bash
# Basic examples
telc examples/hello.tel
telc examples/calculator.tel
telc examples/conditions.tel

# Loop examples
telc examples/loops.tel
telc examples/for-loops.tel

# Array examples
telc examples/test-arrays.tel

# Boolean examples
telc examples/booleans-basic.tel             # Basic boolean values and operations
telc examples/booleans-logic.tel             # Logic operations (AND, OR, NOT)
telc examples/booleans-conditionals.tel      # Advanced conditional logic
telc examples/booleans-functions-simple.tel  # Functions with boolean parameters

# REST API examples
telc examples/rest-api-basic.tel             # Basic HTTP requests (GET, JSON)
telc examples/rest-api-demo.tel              # Quick HTTP methods demo
telc examples/rest-api-advanced.tel          # CRUD operations with functions
telc examples/rest-api-complete-example.tel  # Complete user management system

# Pyramid pattern examples
telc examples/pyramid-simple.tel
telc examples/pyramid-advanced.tel

# Two Sum algorithm examples (refactored)
telc examples/two-sum-basic.tel              # Simple implementation
telc examples/two-sum-optimized.tel          # Optimized with helper functions
telc examples/two-sum-complete.tel           # Comprehensive demo
telc examples/two-sum-with-arrays.tel        # Using real arrays

# Function examples
telc examples/functions-working.tel

# Algorithm examples
telc examples/algorithms.tel
telc examples/algorithms-with-functions.tel
```

## File Extension

Telugu language files use the `.tel` extension.

## VS Code Extension

For the best coding experience, install the VS Code extension for syntax highlighting and code snippets:

```bash
# Install VS Code extension
./install-vscode-extension.sh
```

The extension provides:
- Syntax highlighting for Telugu keywords
- Code snippets and auto-completion
- Smart indentation and bracket matching
- File association for `.tel` files

## Development

### Project Structure
```
newlang/
├── telugu-lang/            # Main compiler
│   ├── src/
│   │   ├── main.rs         # CLI interface
│   │   ├── lexer.rs        # Tokenization
│   │   ├── parser.rs       # Syntax analysis
│   │   ├── interpreter.rs  # Execution engine
│   │   ├── ast.rs         # Abstract syntax tree
│   │   └── token.rs       # Token definitions
│   ├── examples/          # Sample programs
│   ├── Cargo.toml        # Rust project config
│   └── README.md
└── vscode-telugu-lang/    # VS Code extension
    ├── syntaxes/          # Syntax highlighting
    ├── snippets/          # Code snippets
    ├── themes/            # Color themes
    └── package.json       # Extension config
```

### Building from Source
```bash
cargo build --release
```

### Running Tests
```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project is open source. Please check the LICENSE file for details.