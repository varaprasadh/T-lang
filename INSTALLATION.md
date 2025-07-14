# Telugu Language Installation Guide

Telugu Language can be installed easily on any platform, similar to how you would install Java or other programming languages.

## Quick Install

### macOS (Homebrew)

```bash
brew tap YOUR_USERNAME/telugu-lang
brew install telugu-lang
```

### Linux

```bash
curl -sSL https://raw.githubusercontent.com/YOUR_USERNAME/telugu-lang/main/scripts/install-linux.sh | bash
```

### Windows (PowerShell as Administrator)

```powershell
irm https://raw.githubusercontent.com/YOUR_USERNAME/telugu-lang/main/scripts/install-windows.ps1 | iex
```

## Version Manager (TelVM)

For managing multiple versions of Telugu Language (similar to SDKMAN for Java):

```bash
curl -s https://raw.githubusercontent.com/YOUR_USERNAME/telugu-lang/main/scripts/telvm-install.sh | bash
```

Then use:
```bash
telvm install 0.1.0
telvm use 0.1.0
telvm list
```

## Manual Installation

1. Download the appropriate binary for your platform from [Releases](https://github.com/YOUR_USERNAME/telugu-lang/releases)
2. Extract the archive
3. Add the `telc` binary to your PATH

## Platform-Specific Instructions

### macOS

After installing via Homebrew, the `telc` command will be available globally.

### Linux

The installer supports:
- Debian/Ubuntu (apt)
- Fedora/RHEL (dnf/yum)
- Arch Linux (pacman)
- openSUSE (zypper)

The binary is installed to `/usr/local/bin/telc`.

### Windows

The PowerShell installer:
- Installs to `C:\Program Files\TeluguLang`
- Automatically adds to system PATH
- Includes an uninstaller

## Verify Installation

```bash
telc --version
```

## Getting Started

```bash
# Run a Telugu program
telc examples/hello.tel

# View help
telc --help
```

## Updating

### With Homebrew
```bash
brew upgrade telugu-lang
```

### With TelVM
```bash
telvm install <new-version>
telvm use <new-version>
```

### Manual Update
Download and install the new version following the manual installation steps.

## Uninstalling

### macOS (Homebrew)
```bash
brew uninstall telugu-lang
```

### Linux
```bash
sudo rm /usr/local/bin/telc
sudo rm -rf /usr/local/share/telugu-lang
```

### Windows
Run the uninstaller from Control Panel or:
```powershell
powershell -ExecutionPolicy Bypass -File "C:\Program Files\TeluguLang\uninstall.ps1"
```

### TelVM
```bash
telvm uninstall <version>
```