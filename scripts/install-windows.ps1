# Telugu Language Compiler - Windows Installation Script
# PowerShell installation script for Windows

param(
    [string]$InstallDir = "$env:ProgramFiles\TeluguLang",
    [string]$Version = "0.1.0",
    [switch]$AddToPath = $true
)

$ErrorActionPreference = "Stop"

# Repository information
$RepoUrl = "https://github.com/YOUR_USERNAME/telugu-lang"
$BinaryName = "telc.exe"

# Colors for output
function Write-Success {
    param([string]$Message)
    Write-Host $Message -ForegroundColor Green
}

function Write-Error {
    param([string]$Message)
    Write-Host $Message -ForegroundColor Red
}

function Write-Info {
    param([string]$Message)
    Write-Host $Message -ForegroundColor Yellow
}

# Check if running as administrator
function Test-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

# Install Rust if not present
function Install-Rust {
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Info "Rust not found. Installing Rust..."
        
        # Download rustup-init
        $rustupUrl = "https://win.rustup.rs/x86_64"
        $rustupPath = "$env:TEMP\rustup-init.exe"
        
        Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath
        
        # Install Rust
        Start-Process -FilePath $rustupPath -ArgumentList "-y" -Wait -NoNewWindow
        
        # Add to current session PATH
        $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
        
        if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
            Write-Error "Failed to install Rust. Please install manually from https://rustup.rs"
            exit 1
        }
    }
}

# Build from source
function Build-FromSource {
    param([string]$SourceDir)
    
    Write-Info "Building Telugu Language Compiler..."
    
    Push-Location $SourceDir
    try {
        & cargo build --release
        if ($LASTEXITCODE -ne 0) {
            throw "Build failed"
        }
    }
    finally {
        Pop-Location
    }
}

# Add to system PATH
function Add-ToPath {
    param([string]$Directory)
    
    $currentPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::Machine)
    
    if ($currentPath -notlike "*$Directory*") {
        Write-Info "Adding $Directory to system PATH..."
        
        $newPath = "$currentPath;$Directory"
        [Environment]::SetEnvironmentVariable("Path", $newPath, [EnvironmentVariableTarget]::Machine)
        
        # Update current session
        $env:PATH = "$env:PATH;$Directory"
        
        Write-Success "Added to PATH successfully"
    } else {
        Write-Info "$Directory is already in PATH"
    }
}

# Main installation function
function Install-TeluguLang {
    Write-Host "Telugu Language Compiler v$Version - Windows Installation" -ForegroundColor Cyan
    Write-Host "========================================================" -ForegroundColor Cyan
    
    # Check administrator privileges
    if (-not (Test-Administrator)) {
        Write-Error "This script requires administrator privileges."
        Write-Info "Please run PowerShell as Administrator and try again."
        exit 1
    }
    
    # Install Rust if needed
    Install-Rust
    
    # Create temporary directory
    $tempDir = New-TemporaryFile | %{ Remove-Item $_; New-Item -ItemType Directory -Path $_ }
    
    try {
        # Download source
        Write-Info "Downloading source code..."
        $archiveUrl = "$RepoUrl/archive/refs/tags/v$Version.zip"
        $archivePath = Join-Path $tempDir "telugu-lang.zip"
        
        Invoke-WebRequest -Uri $archiveUrl -OutFile $archivePath
        
        # Extract
        Write-Info "Extracting source code..."
        Expand-Archive -Path $archivePath -DestinationPath $tempDir -Force
        
        $sourceDir = Join-Path $tempDir "telugu-lang-$Version"
        
        # Build
        Build-FromSource -SourceDir $sourceDir
        
        # Create installation directory
        Write-Info "Installing to $InstallDir..."
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
        New-Item -ItemType Directory -Path "$InstallDir\bin" -Force | Out-Null
        New-Item -ItemType Directory -Path "$InstallDir\examples" -Force | Out-Null
        New-Item -ItemType Directory -Path "$InstallDir\doc" -Force | Out-Null
        
        # Copy files
        Copy-Item -Path "$sourceDir\target\release\telc.exe" -Destination "$InstallDir\bin\telc.exe" -Force
        Copy-Item -Path "$sourceDir\examples\*" -Destination "$InstallDir\examples\" -Recurse -Force
        Copy-Item -Path "$sourceDir\README.md" -Destination "$InstallDir\doc\" -Force
        
        # Add to PATH if requested
        if ($AddToPath) {
            Add-ToPath -Directory "$InstallDir\bin"
        }
        
        # Create uninstaller
        $uninstallerContent = @"
# Telugu Language Compiler Uninstaller
param([switch]`$Silent)

if (-not `$Silent) {
    `$result = [System.Windows.Forms.MessageBox]::Show(
        "Are you sure you want to uninstall Telugu Language Compiler?",
        "Uninstall Telugu Language Compiler",
        [System.Windows.Forms.MessageBoxButtons]::YesNo
    )
    if (`$result -ne 'Yes') { exit }
}

# Remove from PATH
`$currentPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::Machine)
`$newPath = `$currentPath -replace ";?$InstallDir\\bin;?", ""
[Environment]::SetEnvironmentVariable("Path", `$newPath, [EnvironmentVariableTarget]::Machine)

# Remove files
Remove-Item -Path "$InstallDir" -Recurse -Force

Write-Host "Telugu Language Compiler has been uninstalled." -ForegroundColor Green
"@
        
        Set-Content -Path "$InstallDir\uninstall.ps1" -Value $uninstallerContent
        
        Write-Success "✅ Telugu Language Compiler installed successfully!"
        Write-Host ""
        Write-Host "Installation Details:" -ForegroundColor Cyan
        Write-Host "- Installed to: $InstallDir"
        Write-Host "- Binary: $InstallDir\bin\telc.exe"
        Write-Host "- Examples: $InstallDir\examples"
        Write-Host ""
        Write-Host "Usage Examples:" -ForegroundColor Cyan
        Write-Host "  telc $InstallDir\examples\hello.tel"
        Write-Host "  telc --help"
        Write-Host ""
        Write-Host "To uninstall, run:" -ForegroundColor Yellow
        Write-Host "  powershell -ExecutionPolicy Bypass -File `"$InstallDir\uninstall.ps1`""
        
    }
    finally {
        # Cleanup
        Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

# Run installation
Install-TeluguLang