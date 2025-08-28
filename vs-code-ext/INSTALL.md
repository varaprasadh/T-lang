# Installing Telugu Language VS Code Extension

## Method 1: Manual Installation (Recommended)

1. **Copy Extension to VS Code Extensions Directory**
   
   Copy the entire `vscode-telugu-lang` folder to your VS Code extensions directory:

   **Windows:**
   ```
   %USERPROFILE%\.vscode\extensions\vscode-telugu-lang\
   ```

   **macOS:**
   ```
   ~/.vscode/extensions/vscode-telugu-lang/
   ```

   **Linux:**
   ```
   ~/.vscode/extensions/vscode-telugu-lang/
   ```

2. **Restart VS Code**
   Close and reopen VS Code to load the extension.

3. **Verify Installation**
   - Create a new file with `.tel` extension
   - You should see syntax highlighting for Telugu keywords

## Method 2: Using VS Code Command Line

```bash
# Navigate to the extension directory
cd vscode-telugu-lang

# Install the extension using VS Code CLI
code --install-extension .
```

## Method 3: Development Mode

For development and testing:

1. Open VS Code
2. Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS)
3. Type "Extensions: Install from VSIX"
4. Browse and select the extension folder

## Verification

1. Open VS Code
2. Go to Extensions view (`Ctrl+Shift+X`)
3. Search for "Telugu Programming Language"
4. You should see the extension listed

## Testing

Create a test file `test.tel`:

```telugu
// Test Telugu Language Syntax Highlighting
sankhya x = 10;
sankhya y = 20;

ayite (x < y) {
    cheppu "Hello Telugu World!";
} lekapothe {
    cheppu "Something else";
}

varaku (x < y) {
    cheppu "Loop iteration";
    x = x + 1;
}
```

You should see:
- Keywords highlighted in purple/blue
- Strings highlighted in green
- Numbers highlighted in orange
- Comments highlighted in gray

## Troubleshooting

### Extension Not Loading
- Make sure the folder is in the correct extensions directory
- Restart VS Code completely
- Check that the `package.json` file is present

### No Syntax Highlighting
- Make sure the file has `.tel` extension
- Check that the extension is enabled in Extensions view
- Try reloading the window (`Ctrl+R` or `Cmd+R`)

### Code Snippets Not Working
- Type the snippet prefix (e.g., `sankhya`)
- Press `Tab` to expand
- Make sure you're in a `.tel` file

## Uninstalling

To remove the extension:
1. Delete the `vscode-telugu-lang` folder from your extensions directory
2. Restart VS Code

## Next Steps

After installation:
1. Install the Telugu compiler (`telc`) if you haven't already
2. Try the example programs in the `examples/` directory
3. Start coding in Telugu!

For compiler installation, see the main README.md in the project root.