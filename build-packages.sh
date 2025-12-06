#!/bin/bash

# ============================================================================
# A-lang Package Builder
# Builds Debian (.deb) and Windows (.exe) packages
# ============================================================================

set -e

VERSION="2.0.0"
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
NC='\033[0m' # No Color

echo -e "${BLUE}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     A-lang Package Builder v${VERSION}                        ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# ============================================================================
# 1. Build Release Binaries
# ============================================================================

echo -e "${YELLOW}[1/5] Building release binaries...${NC}"

# Build for Linux
echo "  → Building Linux binary..."
cargo build --release
strip target/release/alang 2>/dev/null || true

# Build for Windows
echo "  → Building Windows binary..."
cargo build --release --target x86_64-pc-windows-gnu
strip target/x86_64-pc-windows-gnu/release/alang.exe 2>/dev/null || true

echo -e "${GREEN}✓ Binaries built successfully${NC}"
echo ""

# ============================================================================
# 2. Create Debian Package Structure
# ============================================================================

echo -e "${YELLOW}[2/5] Creating Debian package...${NC}"

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
echo "║  🚀 A-lang v2.0.0 installed successfully!                     ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "Try it now:"
echo "  $ alang --version"
echo "  $ alang --help"
echo ""
echo "Documentation: https://alang.dev/docs"
echo "Examples: /usr/share/doc/alang/"
echo ""

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

  * Initial release
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
Copyright: 2024 A-lang Contributors
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
cat > ${DEB_ROOT}/usr/share/man/man1/${APP_NAME}.1 << 'EOF'
.TH ALANG 1 "December 2024" "A-lang 2.0.0" "User Commands"
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
Copyright © 2024 A-lang Contributors.
License MIT: <https://opensource.org/licenses/MIT>.
.SH SEE ALSO
Full documentation at: <https://alang.dev/docs>
EOF

gzip -9 ${DEB_ROOT}/usr/share/man/man1/${APP_NAME}.1

# Build the .deb package
echo "  → Building .deb package..."
dpkg-deb --build --root-owner-group ${DEB_ROOT}
mv ${DEB_ROOT}.deb ${DEB_DIR}/

# Calculate size
DEB_SIZE=$(du -h ${DEB_DIR}/${DEB_PKG}.deb | cut -f1)

echo -e "${GREEN}✓ Debian package created: ${DEB_DIR}/${DEB_PKG}.deb (${DEB_SIZE})${NC}"
echo ""

# ============================================================================
# 3. Create Windows Package
# ============================================================================

echo -e "${YELLOW}[3/5] Creating Windows package...${NC}"

WIN_DIR="build/windows"
WIN_PKG="${APP_NAME}-${VERSION}-windows-x64"
WIN_ROOT="${WIN_DIR}/${WIN_PKG}"

# Clean and create directory structure
rm -rf ${WIN_DIR}
mkdir -p ${WIN_ROOT}

# Copy binary
cp target/x86_64-pc-windows-gnu/release/alang.exe ${WIN_ROOT}/

# Create README.txt
cat > ${WIN_ROOT}/README.txt << EOF
A-lang v${VERSION} - The Revolutionary Scripting Language
==========================================================

Thank you for installing A-lang!

INSTALLATION
------------
1. Add this folder to your Windows PATH:
   - Open "Environment Variables"
   - Edit "Path" variable
   - Add: C:\path\to\${WIN_PKG}
   - Click OK

2. Verify installation:
   Open Command Prompt and type:
     alang --version

QUICK START
-----------
Create a file hello.al:
  name = "World"
  print("Hello, " + name + "!")

Run it:
  alang hello.al

FEATURES
--------
✓ Time-Travel Debugging
✓ Reactive Variables
✓ Runtime Syntax Extensions
✓ Smart Auto-Parallelization
✓ Context-Aware Type System
✓ Backend Framework (HTTP, WebSocket, MySQL)
✓ IoT Support (GPIO, I2C, SPI, UART)

DOCUMENTATION
-------------
Visit: https://alang.dev/docs

EXAMPLES
--------
Check examples/ folder for:
- hello.al - Basic syntax
- js_style.al - Complete feature demo
- reactive_counter.al - Reactive variables
- operators_demo.al - Operators and stdlib

SUPPORT
-------
- GitHub: https://github.com/alang/issues
- Discord: https://discord.gg/alang
- Email: support@alang.dev

LICENSE
-------
MIT License - Copyright © 2024 A-lang Contributors

Made with ❤️ and Rust 🦀
EOF

# Copy examples
mkdir -p ${WIN_ROOT}/examples
cp examples/*.al ${WIN_ROOT}/examples/ 2>/dev/null || true

# Create installer script
cat > ${WIN_ROOT}/install.bat << 'EOF'
@echo off
echo ========================================
echo A-lang Installer
echo ========================================
echo.

set "INSTALL_DIR=%ProgramFiles%\A-lang"
set "BIN_PATH=%INSTALL_DIR%\bin"

echo Installing A-lang to: %INSTALL_DIR%
echo.

REM Create directory
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"
if not exist "%BIN_PATH%" mkdir "%BIN_PATH%"

REM Copy files
copy /Y alang.exe "%BIN_PATH%\" >nul
xcopy /E /I /Y examples "%INSTALL_DIR%\examples\" >nul
copy /Y README.txt "%INSTALL_DIR%\" >nul

echo Files copied successfully!
echo.

REM Add to PATH
echo Adding to PATH...
setx PATH "%PATH%;%BIN_PATH%" /M 2>nul

if %ERRORLEVEL% EQU 0 (
    echo ✓ Installation complete!
    echo.
    echo Please restart your Command Prompt and run:
    echo   alang --version
) else (
    echo ⚠ Could not add to PATH automatically.
    echo Please add manually: %BIN_PATH%
)

echo.
pause
EOF

# Create uninstaller
cat > ${WIN_ROOT}/uninstall.bat << 'EOF'
@echo off
echo ========================================
echo A-lang Uninstaller
echo ========================================
echo.

set "INSTALL_DIR=%ProgramFiles%\A-lang"

if exist "%INSTALL_DIR%" (
    echo Removing A-lang from: %INSTALL_DIR%
    rmdir /S /Q "%INSTALL_DIR%"
    echo ✓ A-lang uninstalled successfully!
) else (
    echo A-lang is not installed.
)

echo.
pause
EOF

# Create ZIP package
echo "  → Creating ZIP archive..."
cd ${WIN_DIR}
zip -r ${WIN_PKG}.zip ${WIN_PKG}/ > /dev/null
cd ../..

ZIP_SIZE=$(du -h ${WIN_DIR}/${WIN_PKG}.zip | cut -f1)

echo -e "${GREEN}✓ Windows package created: ${WIN_DIR}/${WIN_PKG}.zip (${ZIP_SIZE})${NC}"
echo ""

# ============================================================================
# 4. Create Portable Linux Package
# ============================================================================

echo -e "${YELLOW}[4/5] Creating portable Linux package...${NC}"

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

## Documentation

Visit: https://alang.dev/docs

## Support

- GitHub: https://github.com/alang/issues
- Discord: https://discord.gg/alang

Made with ❤️ and Rust 🦀
EOF

# Copy examples
mkdir -p ${LINUX_ROOT}/examples
cp examples/*.al ${LINUX_ROOT}/examples/ 2>/dev/null || true

# Create install script
cat > ${LINUX_ROOT}/install.sh << 'EOF'
#!/bin/bash

echo "=========================================="
echo "A-lang Installer"
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
EOF

chmod +x ${LINUX_ROOT}/install.sh

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

echo -e "${YELLOW}[5/5] Generating checksums...${NC}"

CHECKSUMS_FILE="build/checksums.txt"

{
    echo "A-lang v${VERSION} - Package Checksums"
    echo "======================================"
    echo ""
    echo "SHA256:"
    echo ""
    sha256sum ${DEB_DIR}/${DEB_PKG}.deb
    sha256sum ${WIN_DIR}/${WIN_PKG}.zip
    sha256sum ${LINUX_DIR}/${LINUX_PKG}.tar.gz
} > ${CHECKSUMS_FILE}

echo -e "${GREEN}✓ Checksums generated: ${CHECKSUMS_FILE}${NC}"
echo ""

# ============================================================================
# Summary
# ============================================================================

echo -e "${BLUE}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                    BUILD COMPLETE! ✅                         ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "Packages created:"
echo ""
echo -e "${GREEN}📦 Debian:${NC}"
echo "   ${DEB_DIR}/${DEB_PKG}.deb (${DEB_SIZE})"
echo "   Install: sudo dpkg -i ${DEB_PKG}.deb"
echo ""
echo -e "${GREEN}📦 Windows:${NC}"
echo "   ${WIN_DIR}/${WIN_PKG}.zip (${ZIP_SIZE})"
echo "   Extract and run install.bat"
echo ""
echo -e "${GREEN}📦 Linux Portable:${NC}"
echo "   ${LINUX_DIR}/${LINUX_PKG}.tar.gz (${TAR_SIZE})"
echo "   Extract and run ./install.sh"
echo ""
echo -e "${GREEN}📋 Checksums:${NC}"
echo "   ${CHECKSUMS_FILE}"
echo ""
echo "Upload to GitHub Releases: https://github.com/alang/releases"
echo ""
echo -e "${BLUE}Made with ❤️ and Rust 🦀${NC}"
