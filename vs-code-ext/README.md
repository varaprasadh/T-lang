# Telugu Programming Language Extension

This VS Code extension provides syntax highlighting and language support for the Telugu Programming Language.

## Features

- **Syntax Highlighting**: Beautiful color coding for Telugu keywords and syntax
- **Code Snippets**: Quick templates for common Telugu language constructs
- **Auto-completion**: IntelliSense support for Telugu keywords
- **Bracket Matching**: Automatic bracket and quote pairing
- **Indentation**: Smart indentation rules for Telugu code

## Telugu Keywords Supported

| Keyword | English | Description |
|---------|---------|-------------|
| `sankhya` | number/int | Variable declaration |
| `cheppu` | print | Print statement |
| `ayite` | if | Conditional statement |
| `lekapothe` | else | Else condition |
| `varaku` | while | While loop |
| `prathi` | for | For loop |
| `nunchi` | from | Used in for loops |
| `pani` | function | Function declaration |
| `tirigi_pampu` | return | Return statement |
| `adugu` | input | Input statement |
| `nijam` | true | Boolean true |
| `abaddham` | false | Boolean false |
| `sunya` | null | Null value |

## Code Snippets

Type these prefixes and press Tab to expand:

- `sankhya` - Variable declaration
- `cheppu` - Print statement
- `ayite` - If statement
- `ayite-lekapothe` - If-else statement
- `varaku` - While loop
- `prathi` - For loop
- `pani` - Function declaration
- `hello` - Hello World program
- `template` - Basic program template
- `calc` - Calculator template
- `for-loop` - For loop simulation

## Installation

### From Source
1. Copy the `vscode-telugu-lang` folder to your VS Code extensions directory:
   - **Windows**: `%USERPROFILE%\.vscode\extensions\`
   - **macOS**: `~/.vscode/extensions/`
   - **Linux**: `~/.vscode/extensions/`

2. Restart VS Code

3. Open any `.tel` file to see syntax highlighting

### Using VS Code Command Line
```bash
# Navigate to the extension directory
cd vscode-telugu-lang

# Install the extension
code --install-extension .
```

## Usage

1. Create a new file with `.tel` extension
2. Start typing Telugu language code
3. Enjoy syntax highlighting and code snippets!

### Example Telugu Code

```telugu
// Hello World Program
cheppu "Namaste! Telugu programming language ki swagathamu!";

// Variables and calculations
sankhya x = 10;
sankhya y = 20;

// Conditional statements
ayite (x < y) {
    cheppu "x is smaller than y";
} lekapothe {
    cheppu "x is greater than or equal to y";
}

// Loops
sankhya count = 0;
varaku (count < 5) {
    cheppu "Count: " + count;
    count = count + 1;
}
```

## Color Scheme

The extension provides beautiful syntax highlighting with the following color coding:

- **Keywords** (ayite, varaku, etc.): Purple/Blue
- **Strings**: Green
- **Numbers**: Orange
- **Comments**: Gray/Green
- **Operators**: Red/Orange
- **Identifiers**: Default text color

## Requirements

- VS Code 1.74.0 or higher
- Telugu Language Compiler (`telc`) for running the code

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## Release Notes

### 1.0.0

- Initial release
- Basic syntax highlighting for Telugu keywords
- Code snippets for common constructs
- Language configuration for brackets and indentation
- Support for `.tel` file extension

## Support

For issues and feature requests, please visit the [GitHub repository](https://github.com/your-org/telugu-lang).

---

**Enjoy coding in Telugu! 🎉**