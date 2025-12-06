#!/bin/bash

# ============================================================================
# A-lang Package Builder v1.0-preview
# Builds Debian (.deb), Windows Installer (.exe), and Linux (.tar.gz)
# ============================================================================

set -e

VERSION="1.0-preview"
YEAR="2025"
ARCH="amd64"
APP_NAME="alang"
DESCRIPTION="A-lang: The Revolutionary Scripting Language"
MAINTAINER="A-lang Contributors <contact@alang.dev>"
HOMEPAGE="https://alang.dev"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     A-lang Package Builder v${VERSION}                     ║${NC}"
echo -e "${BLUE}║     Building for ${YEAR}                                       ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# ============================================================================
# Update Cargo.toml version
# ============================================================================

echo -e "${YELLOW}[0/6] Updating version to ${VERSION}...${NC}"
sed -i 's/version = ".*"/version = "1.0.0-preview"/' Cargo.toml
echo -e "${GREEN}✓ Version updated in Cargo.toml${NC}"
echo ""

# ============================================================================
# 1. Build Release Binaries
# ============================================================================

echo -e "${YELLOW}[1/6] Building release binaries...${NC}"

# Build for Linux
echo "  → Building Linux binary..."
cargo build --release --quiet 2>/dev/null || cargo build --release
strip target/release/alang 2>/dev/null || true

# Build for Windows
echo "  → Building Windows binary..."
cargo build --release --target x86_64-pc-windows-gnu --quiet 2>/dev/null || cargo build --release --target x86_64-pc-windows-gnu
strip target/x86_64-pc-windows-gnu/release/alang.exe 2>/dev/null || true

echo -e "${GREEN}✓ Binaries built successfully${NC}"
echo ""

# ============================================================================
# 2. Create Debian Package
# ============================================================================

echo -e "${YELLOW}[2/6] Creating Debian package...${NC}"

DEB_DIR="build/debian"
DEB_PKG="${APP_NAME}_${VERSION}_${ARCH}"
DEB_ROOT="${DEB_DIR}/${DEB_PKG}"

# Clean and create directory structure
rm -rf ${DEB_DIR}
mkdir -p ${DEB_ROOT}/DEBIAN
mkdir -p ${DEB_ROOT}/usr/bin
mkdir -p ${DEB_ROOT}/usr/share/doc/${APP_NAME}
mkdir -p ${DEB_ROOT}/usr/share/man/man1

# Copy binary
cp target/release/alang ${DEB_ROOT}/usr/bin/
chmod 755 ${DEB_ROOT}/usr/bin/alang

# Create control file
cat > ${DEB_ROOT}/DEBIAN/control << EOF
Package: ${APP_NAME}
Version: ${VERSION}
Section: interpreters
Priority: optional
Architecture: ${ARCH}
Depends: libc6 (>= 2.34)
Maintainer: ${MAINTAINER}
Homepage: ${HOMEPAGE}
Description: ${DESCRIPTION}
 A-lang is a revolutionary scripting language written in Rust that combines
 the simplicity of JavaScript with groundbreaking features:
 .
  - Time-Travel Debugging: Rewind execution and inspect historical states
  - Reactive Variables: Automatic change propagation like Vue.js/React
  - Runtime Syntax Extensions: Create new syntax during execution
  - Smart Auto-Parallelization: Automatic multi-core utilization
  - Context-Aware Type System: Types that adapt to usage context
 .
 Perfect for scripting, backend development, and IoT applications.
EOF

# Create postinst script
cat > ${DEB_ROOT}/DEBIAN/postinst << 'EOF'
#!/bin/bash
set -e

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║  🚀 A-lang v1.0-preview installed successfully!               ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "Try it now:"
echo "  $ alang --version"
echo "  $ alang --help"
echo ""
echo "Documentation: https://alang.dev/docs"
echo "Examples: /usr/share/doc/alang/"
echo ""
echo "This is a preview release for 2025!"

exit 0
EOF

chmod 755 ${DEB_ROOT}/DEBIAN/postinst

# Create prerm script
cat > ${DEB_ROOT}/DEBIAN/prerm << 'EOF'
#!/bin/bash
set -e
exit 0
EOF

chmod 755 ${DEB_ROOT}/DEBIAN/prerm

# Copy documentation
cat > ${DEB_ROOT}/usr/share/doc/${APP_NAME}/README << EOF
A-lang - The Revolutionary Scripting Language
==============================================

Version: ${VERSION}
Year: ${YEAR}
Homepage: ${HOMEPAGE}

A-lang is a modern scripting language written in Rust with unique features:

✓ Time-Travel Debugging
✓ Reactive Variables
✓ Runtime Syntax Extensions
✓ Smart Auto-Parallelization
✓ Context-Aware Type System

Installation
------------
Already installed! Try:
  $ alang --version
  $ alang examples/hello.al

Documentation
-------------
Visit: https://alang.dev/docs

Quick Start
-----------
Create a file hello.al:
  name = "World"
  print("Hello, " + name + "!")

Run it:
  $ alang hello.al

License
-------
MIT License - See https://alang.dev/license

EOF

# Copy changelog
cat > ${DEB_ROOT}/usr/share/doc/${APP_NAME}/changelog << EOF
alang (${VERSION}) unstable; urgency=medium

  * Preview release for ${YEAR}
  * Time-Travel Debugging implemented
  * Reactive Variables system
  * Backend framework (HTTP, WebSocket, MySQL)
  * IoT support (GPIO, I2C, SPI, UART)
  * Complete standard library

 -- ${MAINTAINER}  $(date -R)
EOF

gzip -9 ${DEB_ROOT}/usr/share/doc/${APP_NAME}/changelog

# Copy copyright
cat > ${DEB_ROOT}/usr/share/doc/${APP_NAME}/copyright << EOF
Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: A-lang
Upstream-Contact: ${MAINTAINER}
Source: ${HOMEPAGE}

Files: *
Copyright: ${YEAR} A-lang Contributors
License: MIT
 Permission is hereby granted, free of charge, to any person obtaining a
 copy of this software and associated documentation files (the "Software"),
 to deal in the Software without restriction, including without limitation
 the rights to use, copy, modify, merge, publish, distribute, sublicense,
 and/or sell copies of the Software, and to permit persons to whom the
 Software is furnished to do so, subject to the following conditions:
 .
 The above copyright notice and this permission notice shall be included
 in all copies or substantial portions of the Software.
 .
 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 DEALINGS IN THE SOFTWARE.
EOF

# Create man page
cat > ${DEB_ROOT}/usr/share/man/man1/${APP_NAME}.1 << EOF
.TH ALANG 1 "January ${YEAR}" "A-lang ${VERSION}" "User Commands"
.SH NAME
alang \- The Revolutionary Scripting Language
.SH SYNOPSIS
.B alang
[\fIOPTIONS\fR] [\fIFILE\fR]
.SH DESCRIPTION
A-lang is a revolutionary scripting language written in Rust with unique features like Time-Travel Debugging, Reactive Variables, and Smart Auto-Parallelization.
.SH OPTIONS
.TP
\fB\-h\fR, \fB\-\-help\fR
Display help information
.TP
\fB\-v\fR, \fB\-\-version\fR
Display version information
.TP
\fB\-\-repl\fR
Start interactive REPL
.SH EXAMPLES
.TP
Run a script:
.B alang script.al
.TP
Start REPL:
.B alang --repl
.SH FILES
.TP
.I ~/.alang/
User configuration directory
.SH AUTHOR
Written by A-lang Contributors.
.SH REPORTING BUGS
Report bugs to: https://github.com/alang/issues
.SH COPYRIGHT
Copyright © ${YEAR} A-lang Contributors.
License MIT: <https://opensource.org/licenses/MIT>.
.SH SEE ALSO
Full documentation at: <https://alang.dev/docs>
EOF

gzip -9 ${DEB_ROOT}/usr/share/man/man1/${APP_NAME}.1

# Build the .deb package
echo "  → Building .deb package..."
dpkg-deb --build --root-owner-group ${DEB_ROOT} > /dev/null 2>&1
mv ${DEB_ROOT}.deb ${DEB_DIR}/

DEB_SIZE=$(du -h ${DEB_DIR}/${DEB_PKG}.deb | cut -f1)

echo -e "${GREEN}✓ Debian package created: ${DEB_DIR}/${DEB_PKG}.deb (${DEB_SIZE})${NC}"
echo ""

# ============================================================================
# 3. Create Windows Installer with NSIS
# ============================================================================

echo -e "${YELLOW}[3/6] Creating Windows installer with NSIS...${NC}"

WIN_DIR="build/windows"
WIN_STAGING="${WIN_DIR}/staging"

# Clean and create directory structure
rm -rf ${WIN_DIR}
mkdir -p ${WIN_STAGING}

# Copy binary and examples
cp target/x86_64-pc-windows-gnu/release/alang.exe ${WIN_STAGING}/
mkdir -p ${WIN_STAGING}/examples
cp examples/*.al ${WIN_STAGING}/examples/ 2>/dev/null || true

# Create README
cat > ${WIN_STAGING}/README.txt << EOF
A-lang v${VERSION} - The Revolutionary Scripting Language
==========================================================

Thank you for installing A-lang!

FEATURES
--------
✓ Time-Travel Debugging
✓ Reactive Variables
✓ Runtime Syntax Extensions
✓ Smart Auto-Parallelization
✓ Context-Aware Type System
✓ Backend Framework (HTTP, WebSocket, MySQL)
✓ IoT Support (GPIO, I2C, SPI, UART)

QUICK START
-----------
Open Command Prompt and type:
  alang --version

Create a file hello.al:
  name = "World"
  print("Hello, " + name + "!")

Run it:
  alang hello.al

DOCUMENTATION
-------------
Visit: https://alang.dev/docs

SUPPORT
-------
- GitHub: https://github.com/alang/issues
- Discord: https://discord.gg/alang
- Email: support@alang.dev

LICENSE
-------
MIT License - Copyright © ${YEAR} A-lang Contributors

Made with ❤️ and Rust 🦀
EOF

# Create NSIS installer script
cat > ${WIN_DIR}/installer.nsi << 'NSIEOF'
; A-lang Installer Script for NSIS
; Modern UI with advanced features

!include "MUI2.nsh"
!include "FileFunc.nsh"

; ============================================================================
; Installer Configuration
; ============================================================================

Name "A-lang"
OutFile "A-lang-1.0-preview-Setup.exe"
InstallDir "$PROGRAMFILES64\A-lang"
InstallDirRegKey HKLM "Software\A-lang" "InstallDir"
RequestExecutionLevel admin

; Version Information
VIProductVersion "1.0.0.0"
VIAddVersionKey "ProductName" "A-lang"
VIAddVersionKey "CompanyName" "A-lang Contributors"
VIAddVersionKey "LegalCopyright" "© 2025 A-lang Contributors"
VIAddVersionKey "FileDescription" "A-lang Installer"
VIAddVersionKey "FileVersion" "1.0.0.0"

; ============================================================================
; Modern UI Configuration
; ============================================================================

!define MUI_ABORTWARNING
!define MUI_ICON "${NSISDIR}\Contrib\Graphics\Icons\modern-install.ico"
!define MUI_UNICON "${NSISDIR}\Contrib\Graphics\Icons\modern-uninstall.ico"

!define MUI_WELCOMEPAGE_TITLE "Welcome to A-lang Setup"
!define MUI_WELCOMEPAGE_TEXT "This wizard will guide you through the installation of A-lang v1.0-preview.$\r$\n$\r$\nA-lang is a revolutionary scripting language with Time-Travel Debugging, Reactive Variables, and more!$\r$\n$\r$\nClick Next to continue."

!define MUI_FINISHPAGE_TITLE "A-lang Installation Complete"
!define MUI_FINISHPAGE_TEXT "A-lang has been successfully installed!$\r$\n$\r$\nYou can now use 'alang' from the command line.$\r$\n$\r$\nTry: alang --version"
!define MUI_FINISHPAGE_RUN
!define MUI_FINISHPAGE_RUN_TEXT "Open Command Prompt"
!define MUI_FINISHPAGE_RUN_FUNCTION "LaunchCmd"

; ============================================================================
; Pages
; ============================================================================

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "staging\README.txt"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

!insertmacro MUI_LANGUAGE "English"

; ============================================================================
; Installer Section
; ============================================================================

Section "A-lang" SecMain
    SetOutPath "$INSTDIR"

    ; Copy files
    File "staging\alang.exe"
    File "staging\README.txt"

    ; Copy examples
    SetOutPath "$INSTDIR\examples"
    File /nonfatal "staging\examples\*.al"

    SetOutPath "$INSTDIR"

    ; Create uninstaller
    WriteUninstaller "$INSTDIR\Uninstall.exe"

    ; Add to PATH
    ${EnvVarUpdate} $0 "PATH" "A" "HKLM" "$INSTDIR"

    ; Write registry keys
    WriteRegStr HKLM "Software\A-lang" "InstallDir" "$INSTDIR"
    WriteRegStr HKLM "Software\A-lang" "Version" "1.0-preview"

    ; Add uninstall information
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\A-lang" "DisplayName" "A-lang v1.0-preview"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\A-lang" "UninstallString" "$\"$INSTDIR\Uninstall.exe$\""
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\A-lang" "DisplayIcon" "$INSTDIR\alang.exe"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\A-lang" "Publisher" "A-lang Contributors"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\A-lang" "DisplayVersion" "1.0-preview"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\A-lang" "HelpLink" "https://alang.dev"
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\A-lang" "NoModify" 1
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\A-lang" "NoRepair" 1

    ; Calculate install size
    ${GetSize} "$INSTDIR" "/S=0K" $0 $1 $2
    IntFmt $0 "0x%08X" $0
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\A-lang" "EstimatedSize" "$0"

    ; Create start menu shortcuts
    CreateDirectory "$SMPROGRAMS\A-lang"
    CreateShortcut "$SMPROGRAMS\A-lang\A-lang Command Prompt.lnk" "cmd.exe" '/k "echo A-lang v1.0-preview && echo. && alang --version"' "$INSTDIR\alang.exe"
    CreateShortcut "$SMPROGRAMS\A-lang\Uninstall.lnk" "$INSTDIR\Uninstall.exe"
    CreateShortcut "$SMPROGRAMS\A-lang\Examples.lnk" "$INSTDIR\examples"

    ; Success message
    DetailPrint "A-lang installed successfully!"
    DetailPrint "Location: $INSTDIR"
    DetailPrint "Added to PATH: Yes"

SectionEnd

; ============================================================================
; Uninstaller Section
; ============================================================================

Section "Uninstall"
    ; Remove from PATH
    ${un.EnvVarUpdate} $0 "PATH" "R" "HKLM" "$INSTDIR"

    ; Remove files
    Delete "$INSTDIR\alang.exe"
    Delete "$INSTDIR\README.txt"
    Delete "$INSTDIR\Uninstall.exe"
    RMDir /r "$INSTDIR\examples"
    RMDir "$INSTDIR"

    ; Remove start menu shortcuts
    RMDir /r "$SMPROGRAMS\A-lang"

    ; Remove registry keys
    DeleteRegKey HKLM "Software\A-lang"
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\A-lang"

    DetailPrint "A-lang uninstalled successfully!"

SectionEnd

; ============================================================================
; Functions
; ============================================================================

Function LaunchCmd
    Exec 'cmd.exe /k "echo A-lang v1.0-preview installed! && echo. && alang --version"'
FunctionEnd

; EnvVarUpdate function (from NSIS wiki)
!define Environ 'HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"'
!macro _EnvVarUpdate ResultVar EnvVarName Action Regloc PathString
    Push "${PathString}"
    Push "${Regloc}"
    Push "${Action}"
    Push "${EnvVarName}"
    Call EnvVarUpdate
    Pop "${ResultVar}"
!macroend

!define EnvVarUpdate '!insertmacro "_EnvVarUpdate"'

Function EnvVarUpdate
    Exch $0
    Exch
    Exch $1
    Exch
    Exch 2
    Exch $2
    Exch 2
    Exch 3
    Exch $3

    Push $4
    Push $5
    Push $6

    ReadRegStr $5 ${Environ} $1

    StrCmp $0 "A" add
    StrCmp $0 "R" remove
    Goto end

    add:
        StrCpy $6 "$5;$3"
        WriteRegExpandStr ${Environ} $1 $6
        SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000
        Goto end

    remove:
        Push $5
        Push $3
        Call un.RemoveFromPath
        Pop $6
        WriteRegExpandStr ${Environ} $1 $6
        SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000
        Goto end

    end:
        StrCpy $0 $6

    Pop $6
    Pop $5
    Pop $4
    Pop $3
    Pop $2
    Pop $1
    Exch $0
FunctionEnd

Function un.EnvVarUpdate
    Call EnvVarUpdate
FunctionEnd

Function un.RemoveFromPath
    Exch $0
    Exch
    Exch $1
    Push $2
    Push $3
    Push $4
    Push $5
    Push $6

    StrLen $2 $0
    StrCpy $5 ""
    StrCpy $3 0

    loop:
        IntOp $3 $3 + 1
        StrCpy $4 $1 $2 $3
        StrCmp $4 "" done
        StrCmp $4 $0 found
        StrCpy $6 $1 1 $3
        StrCpy $5 "$5$6"
        Goto loop

    found:
        IntOp $3 $3 + $2
        Goto loop

    done:
        StrCpy $0 $5

    Pop $6
    Pop $5
    Pop $4
    Pop $3
    Pop $2
    Pop $1
    Exch $0
FunctionEnd
NSIEOF

# Build the installer
echo "  → Compiling NSIS installer..."
cd ${WIN_DIR}
makensis -V2 installer.nsi > /dev/null 2>&1 || makensis installer.nsi
cd ../..

WIN_SIZE=$(du -h ${WIN_DIR}/A-lang-1.0-preview-Setup.exe | cut -f1)

echo -e "${GREEN}✓ Windows installer created: ${WIN_DIR}/A-lang-1.0-preview-Setup.exe (${WIN_SIZE})${NC}"
echo ""

# ============================================================================
# 4. Create Portable Linux Package
# ============================================================================

echo -e "${YELLOW}[4/6] Creating portable Linux package...${NC}"

LINUX_DIR="build/linux"
LINUX_PKG="${APP_NAME}-${VERSION}-linux-x64"
LINUX_ROOT="${LINUX_DIR}/${LINUX_PKG}"

# Clean and create directory structure
rm -rf ${LINUX_DIR}
mkdir -p ${LINUX_ROOT}

# Copy binary
cp target/release/alang ${LINUX_ROOT}/

# Create README
cat > ${LINUX_ROOT}/README.md << EOF
# A-lang v${VERSION} - Linux Portable

## Installation

1. Extract this archive anywhere
2. Add to PATH (optional):
   \`\`\`bash
   echo 'export PATH="\$PATH:$(pwd)"' >> ~/.bashrc
   source ~/.bashrc
   \`\`\`

3. Or run directly:
   \`\`\`bash
   ./alang --version
   \`\`\`

## Quick Start

\`\`\`bash
# Create hello.al
echo 'print("Hello, A-lang!")' > hello.al

# Run it
./alang hello.al
\`\`\`

## Features

✓ Time-Travel Debugging
✓ Reactive Variables
✓ Runtime Syntax Extensions
✓ Smart Auto-Parallelization
✓ Context-Aware Type System
✓ Backend Framework
✓ IoT Support

## Documentation

Visit: https://alang.dev/docs

## Year

${YEAR} Preview Release

Made with ❤️ and Rust 🦀
EOF

# Copy examples
mkdir -p ${LINUX_ROOT}/examples
cp examples/*.al ${LINUX_ROOT}/examples/ 2>/dev/null || true

# Create install script
cat > ${LINUX_ROOT}/install.sh << 'EOF'
#!/bin/bash

echo "=========================================="
echo "A-lang v1.0-preview Installer"
echo "=========================================="
echo ""

INSTALL_DIR="/usr/local/bin"

if [ "$EUID" -ne 0 ]; then
    echo "⚠ Please run as root (sudo ./install.sh)"
    exit 1
fi

echo "Installing A-lang to: ${INSTALL_DIR}"
cp alang ${INSTALL_DIR}/
chmod 755 ${INSTALL_DIR}/alang

echo "✓ Installation complete!"
echo ""
echo "Try: alang --version"
echo ""
echo "A-lang v1.0-preview - 2025 Release"
EOF

chmod +x ${LINUX_ROOT}/install.sh
chmod +x ${LINUX_ROOT}/alang

# Create tarball
echo "  → Creating tarball..."
cd ${LINUX_DIR}
tar czf ${LINUX_PKG}.tar.gz ${LINUX_PKG}/
cd ../..

TAR_SIZE=$(du -h ${LINUX_DIR}/${LINUX_PKG}.tar.gz | cut -f1)

echo -e "${GREEN}✓ Linux package created: ${LINUX_DIR}/${LINUX_PKG}.tar.gz (${TAR_SIZE})${NC}"
echo ""

# ============================================================================
# 5. Generate Checksums
# ============================================================================

echo -e "${YELLOW}[5/6] Generating checksums...${NC}"

CHECKSUMS_FILE="build/checksums.txt"

{
    echo "A-lang v${VERSION} - Package Checksums (${YEAR})"
    echo "================================================"
    echo ""
    echo "SHA256:"
    echo ""
    sha256sum ${DEB_DIR}/${DEB_PKG}.deb
    sha256sum ${WIN_DIR}/A-lang-1.0-preview-Setup.exe
    sha256sum ${LINUX_DIR}/${LINUX_PKG}.tar.gz
} > ${CHECKSUMS_FILE}

echo -e "${GREEN}✓ Checksums generated: ${CHECKSUMS_FILE}${NC}"
echo ""

# ============================================================================
# 6. Create Installation Guide
# ============================================================================

echo -e "${YELLOW}[6/6] Creating installation guide...${NC}"

cat > build/INSTALL_GUIDE.md << EOF
# 📦 A-lang v${VERSION} Installation Guide

**Year**: ${YEAR}
**Date**: $(date +"%B %d, %Y")

## Available Packages

✅ **Debian/Ubuntu** (.deb) - ${DEB_SIZE}
✅ **Windows Installer** (.exe) - ${WIN_SIZE}
✅ **Linux Portable** (.tar.gz) - ${TAR_SIZE}

---

## 🐧 Debian/Ubuntu Installation

\`\`\`bash
# Install the package
sudo dpkg -i ${DEB_PKG}.deb

# Verify installation
alang --version
# Output: A-lang v${VERSION}

# Run example
alang examples/hello.al
\`\`\`

---

## 🪟 Windows Installation

### Using the Installer (Recommended)

1. **Run the installer**: \`A-lang-1.0-preview-Setup.exe\`
2. **Follow the wizard**:
   - Click "Next" through the welcome screen
   - Choose installation directory (default: C:\\Program Files\\A-lang)
   - Click "Install"
3. **Automatic setup**:
   - ✅ Binary installed
   - ✅ Added to PATH automatically
   - ✅ Start Menu shortcuts created
   - ✅ Uninstaller registered
4. **Verify installation**:
   - Open Command Prompt
   - Type: \`alang --version\`

### Features:
- ✅ Full installation wizard with Modern UI
- ✅ Automatic PATH configuration
- ✅ Start Menu integration
- ✅ Uninstaller in Control Panel
- ✅ Examples included
- ✅ Admin rights handled automatically

---

## 🐧 Linux Portable Installation

\`\`\`bash
# Extract
tar xzf ${LINUX_PKG}.tar.gz
cd ${LINUX_PKG}

# Run directly
./alang --version

# Or install system-wide
sudo ./install.sh

# Verify
alang --version
\`\`\`

---

## 🚀 Quick Start

### Hello World

Create \`hello.al\`:
\`\`\`javascript
name = "World"
print("Hello, " + name + "!")
\`\`\`

Run it:
\`\`\`bash
alang hello.al
# Output: Hello, World!
\`\`\`

### Reactive Variables

\`\`\`javascript
reactive counter = 0
reactive doubled = counter * 2

counter = 5
print(doubled)  // Outputs: 10
\`\`\`

### Time-Travel Debugging

\`\`\`javascript
checkpoint "start"
x = 10
snapshot
x = 20
rewind to "start"
print(x)  // Outputs: 10
\`\`\`

---

## 📊 Package Checksums (SHA256)

\`\`\`
$(cat build/checksums.txt | grep -A 10 "SHA256")
\`\`\`

Verify with:
\`\`\`bash
sha256sum -c checksums.txt
\`\`\`

---

## 🗑️ Uninstallation

### Debian/Ubuntu
\`\`\`bash
sudo apt remove alang
# or
sudo dpkg -r alang
\`\`\`

### Windows
- Open "Add or Remove Programs"
- Search for "A-lang"
- Click "Uninstall"
- Or run: \`C:\\Program Files\\A-lang\\Uninstall.exe\`

### Linux Portable
\`\`\`bash
sudo rm /usr/local/bin/alang
\`\`\`

---

## 🆘 Troubleshooting

### Windows: "alang is not recognized"
- Restart Command Prompt after installation
- Or logout and login again
- PATH is added automatically by installer

### Linux: "Permission denied"
\`\`\`bash
chmod +x alang
\`\`\`

### Debian: Missing dependencies
\`\`\`bash
sudo apt install libc6
\`\`\`

---

## 📚 Documentation

- **Syntax Reference**: See \`SYNTAX_REFERENCE.md\`
- **Website**: https://alang.dev (coming soon)
- **GitHub**: https://github.com/alang
- **Discord**: https://discord.gg/alang

---

**Made with ❤️ and Rust 🦀**

🚀 **A-lang: Where the future of scripting begins!**

**${YEAR} Preview Release**
EOF

echo -e "${GREEN}✓ Installation guide created${NC}"
echo ""

# ============================================================================
# Summary
# ============================================================================

echo -e "${PURPLE}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${PURPLE}║                    BUILD COMPLETE! ✅                         ║${NC}"
echo -e "${PURPLE}║                  A-lang v${VERSION} (${YEAR})                    ║${NC}"
echo -e "${PURPLE}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "📦 Packages created:"
echo ""
echo -e "${GREEN}📦 Debian/Ubuntu:${NC}"
echo "   ${DEB_DIR}/${DEB_PKG}.deb (${DEB_SIZE})"
echo "   Install: sudo dpkg -i ${DEB_PKG}.deb"
echo ""
echo -e "${GREEN}📦 Windows Installer:${NC}"
echo "   ${WIN_DIR}/A-lang-1.0-preview-Setup.exe (${WIN_SIZE})"
echo "   Run installer with admin rights"
echo ""
echo -e "${GREEN}📦 Linux Portable:${NC}"
echo "   ${LINUX_DIR}/${LINUX_PKG}.tar.gz (${TAR_SIZE})"
echo "   Extract and run ./install.sh"
echo ""
echo -e "${GREEN}📋 Checksums:${NC}"
echo "   ${CHECKSUMS_FILE}"
echo ""
echo -e "${BLUE}🎉 Ready for ${YEAR} release!${NC}"
echo ""
echo "Next steps:"
echo "  1. Test installations on each platform"
echo "  2. Create GitHub Release"
echo "  3. Upload packages"
echo "  4. Update documentation"
echo ""
echo -e "${PURPLE}Made with ❤️ and Rust 🦀${NC}"
