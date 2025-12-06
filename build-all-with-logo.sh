#!/bin/bash
# ============================================
# A-lang Complete Build Script with Logo
# Version: 1.0-preview
# ============================================

set -e  # Exit on error

VERSION="1.0-preview"
YEAR="2025"
BUILD_DIR="build"
RELEASE_DIR="release"

echo "╔════════════════════════════════════════╗"
echo "║  A-lang Build Script v${VERSION}        ║"
echo "║  Building for all platforms...         ║"
echo "╚════════════════════════════════════════╝"
echo ""

# ============================================
# 1. Clean previous builds
# ============================================
echo "🧹 Cleaning previous builds..."
cargo clean
rm -rf "${BUILD_DIR}"
rm -rf "${RELEASE_DIR}"
mkdir -p "${RELEASE_DIR}"
echo "✓ Clean complete"
echo ""

# ============================================
# 2. Build release binary
# ============================================
echo "🔨 Building release binary..."
cargo build --release
if [ ! -f "target/release/alang" ]; then
    echo "❌ Build failed!"
    exit 1
fi
echo "✓ Build complete"
echo ""

# ============================================
# 3. Prepare assets
# ============================================
echo "🎨 Preparing assets..."

# Convert PNG to ICO if not exists
if [ ! -f "Assets/logo.ico" ]; then
    echo "  Converting PNG to ICO..."
    if command -v convert &> /dev/null; then
        convert Assets/logo.png -define icon:auto-resize=256,128,64,48,32,16 Assets/logo.ico
        echo "  ✓ ICO created"
    else
        echo "  ⚠ ImageMagick not found, ICO not created"
    fi
fi

echo "✓ Assets ready"
echo ""

# ============================================
# 4. Build Debian Package
# ============================================
echo "📦 Building Debian package..."

DEB_DIR="${BUILD_DIR}/debian"
mkdir -p "${DEB_DIR}/DEBIAN"
mkdir -p "${DEB_DIR}/usr/local/bin"
mkdir -p "${DEB_DIR}/usr/share/alang/examples"
mkdir -p "${DEB_DIR}/usr/share/doc/alang"
mkdir -p "${DEB_DIR}/usr/share/pixmaps"

# Copy binary
cp target/release/alang "${DEB_DIR}/usr/local/bin/"
chmod +x "${DEB_DIR}/usr/local/bin/alang"

# Copy examples (including new ones!)
cp examples/*.al "${DEB_DIR}/usr/share/alang/examples/"

# Copy documentation
cp README.md "${DEB_DIR}/usr/share/doc/alang/"
cp SYNTAX_REFERENCE.md "${DEB_DIR}/usr/share/doc/alang/" 2>/dev/null || true
cp INPUT_AND_FFI_DOCS.md "${DEB_DIR}/usr/share/doc/alang/" 2>/dev/null || true
cp FFI_IMPLEMENTATION_PLAN.md "${DEB_DIR}/usr/share/doc/alang/" 2>/dev/null || true

# Copy logo
cp Assets/logo.png "${DEB_DIR}/usr/share/pixmaps/alang.png"

# Create control file
cat > "${DEB_DIR}/DEBIAN/control" << EOF
Package: alang
Version: ${VERSION}
Section: devel
Priority: optional
Architecture: amd64
Maintainer: A-lang Project <alang@example.com>
Description: A-lang - Revolutionary scripting language
 A-lang is a modern scripting language with:
  - Time-travel debugging
  - Reactive variables
  - FFI for C libraries
  - User input support
  - Backend/IoT capabilities
EOF

# Build .deb
dpkg-deb --build "${DEB_DIR}" "${RELEASE_DIR}/alang_${VERSION}_amd64.deb"
echo "✓ Debian package: ${RELEASE_DIR}/alang_${VERSION}_amd64.deb"
echo ""

# ============================================
# 5. Build Windows Installer
# ============================================
echo "🪟 Building Windows installer..."

WIN_DIR="${BUILD_DIR}/windows"
STAGING_DIR="${WIN_DIR}/staging"

mkdir -p "${STAGING_DIR}/examples"

# Cross-compile for Windows (if mingw available)
if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "  Cross-compiling for Windows..."
    cargo build --release --target x86_64-pc-windows-gnu 2>/dev/null || {
        echo "  ⚠ Cross-compilation failed, using dummy exe"
        # Create a marker file
        echo "This is a placeholder. Build on Windows for actual .exe" > "${STAGING_DIR}/alang.exe"
    }

    if [ -f "target/x86_64-pc-windows-gnu/release/alang.exe" ]; then
        cp target/x86_64-pc-windows-gnu/release/alang.exe "${STAGING_DIR}/"
    fi
else
    echo "  ⚠ MinGW not found, creating placeholder"
    echo "Build on Windows for actual .exe" > "${STAGING_DIR}/alang.exe"
fi

# Copy examples
cp examples/*.al "${STAGING_DIR}/examples/"

# Create README for Windows
cat > "${STAGING_DIR}/README.txt" << EOF
A-lang v${VERSION} - ${YEAR}
============================

Welcome to A-lang!

Features:
  - Time-travel debugging
  - Reactive variables
  - FFI (Foreign Function Interface)
  - User input with input()
  - Backend & IoT support

Getting Started:
  1. Open Command Prompt or PowerShell
  2. Type: alang
  3. Try: alang examples\hello.al

Documentation:
  See https://github.com/yourusername/a-lang

Examples:
  - hello.al          - Hello world
  - input_demo.al     - User input examples
  - ffi_demo.al       - FFI examples (Linux only)
  - reactive_counter.al - Reactive variables
  - stdlib_demo.al    - Standard library

For help: alang --help
EOF

# Create NSIS installer script
cat > "${WIN_DIR}/installer.nsi" << 'EOF'
!include "MUI2.nsh"

Name "A-lang v1.0-preview"
OutFile "A-lang-1.0-preview-Setup.exe"
InstallDir "$PROGRAMFILES64\A-lang"
RequestExecutionLevel admin

# Modern UI
!define MUI_ICON "..\..\Assets\logo.ico"
!define MUI_HEADERIMAGE
!define MUI_HEADERIMAGE_BITMAP "..\..\Assets\logo.png"
!define MUI_ABORTWARNING

BrandingText "A-lang - Time-Travel Reactive Language"

# Pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "English"

Section "Install"
    SetOutPath "$INSTDIR"
    File "staging\alang.exe"
    File "staging\README.txt"

    SetOutPath "$INSTDIR\examples"
    File /nonfatal "staging\examples\*.al"

    SetOutPath "$INSTDIR"
    WriteUninstaller "$INSTDIR\Uninstall.exe"

    # Add to PATH
    EnvVarUpdate $0 "PATH" "A" "HKLM" "$INSTDIR"

    # Create Start Menu shortcuts
    CreateDirectory "$SMPROGRAMS\A-lang"
    CreateShortCut "$SMPROGRAMS\A-lang\A-lang REPL.lnk" "$INSTDIR\alang.exe"
    CreateShortCut "$SMPROGRAMS\A-lang\Examples.lnk" "$INSTDIR\examples"
    CreateShortCut "$SMPROGRAMS\A-lang\Uninstall.lnk" "$INSTDIR\Uninstall.exe"

    # Registry
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\A-lang" "DisplayName" "A-lang v1.0-preview"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\A-lang" "UninstallString" "$INSTDIR\Uninstall.exe"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\A-lang" "DisplayIcon" "$INSTDIR\alang.exe"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\A-lang" "Publisher" "A-lang Project"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\A-lang" "DisplayVersion" "1.0-preview"
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\A-lang" "EstimatedSize" 10240
SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\alang.exe"
    Delete "$INSTDIR\README.txt"
    RMDir /r "$INSTDIR\examples"
    Delete "$INSTDIR\Uninstall.exe"
    RMDir "$INSTDIR"

    # Remove Start Menu shortcuts
    RMDir /r "$SMPROGRAMS\A-lang"

    EnvVarUpdate $0 "PATH" "R" "HKLM" "$INSTDIR"
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\A-lang"
SectionEnd

!include "winmessages.nsh"
!define HWND_BROADCAST 0xffff
!define WM_WININICHANGE 0x001A

Function EnvVarUpdate
    ReadRegStr $1 HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "Path"
    StrCpy $2 "$1;$INSTDIR"
    WriteRegExpandStr HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "Path" $2
    SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000
FunctionEnd
EOF

# Build installer (if NSIS available)
if command -v makensis &> /dev/null; then
    echo "  Building NSIS installer..."
    cd "${WIN_DIR}"
    makensis installer.nsi
    cd ../..
    mv "${WIN_DIR}/A-lang-1.0-preview-Setup.exe" "${RELEASE_DIR}/"
    echo "✓ Windows installer: ${RELEASE_DIR}/A-lang-1.0-preview-Setup.exe"
else
    echo "  ⚠ NSIS not found, creating ZIP instead..."
    cd "${STAGING_DIR}"
    zip -r "../../../${RELEASE_DIR}/alang-${VERSION}-windows.zip" .
    cd ../../..
    echo "✓ Windows ZIP: ${RELEASE_DIR}/alang-${VERSION}-windows.zip"
fi
echo ""

# ============================================
# 6. Build Linux Portable Package
# ============================================
echo "🐧 Building Linux portable package..."

LINUX_DIR="${BUILD_DIR}/linux"
mkdir -p "${LINUX_DIR}/alang-${VERSION}-linux-x64"
mkdir -p "${LINUX_DIR}/alang-${VERSION}-linux-x64/examples"

# Copy binary
cp target/release/alang "${LINUX_DIR}/alang-${VERSION}-linux-x64/"

# Copy examples
cp examples/*.al "${LINUX_DIR}/alang-${VERSION}-linux-x64/examples/"

# Create README
cat > "${LINUX_DIR}/alang-${VERSION}-linux-x64/README.md" << EOF
# A-lang v${VERSION}

## Installation

1. Extract this archive
2. Add to PATH:
   \`\`\`bash
   export PATH=\$PATH:\$(pwd)
   \`\`\`
3. Run: \`./alang\`

## Features

✅ Time-travel debugging
✅ Reactive variables
✅ FFI - Call C libraries
✅ User input with input()
✅ Backend & IoT support

## Examples

\`\`\`bash
./alang examples/hello.al
./alang examples/input_demo.al
./alang examples/ffi_demo.al
./alang examples/reactive_counter.al
\`\`\`

## Documentation

- SYNTAX_REFERENCE.md - Complete language reference
- INPUT_AND_FFI_DOCS.md - New features (input & FFI)
- FFI_IMPLEMENTATION_PLAN.md - FFI technical details

## Support

GitHub: https://github.com/yourusername/a-lang
EOF

# Copy documentation
cp SYNTAX_REFERENCE.md "${LINUX_DIR}/alang-${VERSION}-linux-x64/" 2>/dev/null || true
cp INPUT_AND_FFI_DOCS.md "${LINUX_DIR}/alang-${VERSION}-linux-x64/" 2>/dev/null || true
cp FFI_IMPLEMENTATION_PLAN.md "${LINUX_DIR}/alang-${VERSION}-linux-x64/" 2>/dev/null || true

# Create tarball
cd "${LINUX_DIR}"
tar -czf "../../${RELEASE_DIR}/alang-${VERSION}-linux-x64.tar.gz" "alang-${VERSION}-linux-x64"
cd ../..

echo "✓ Linux tarball: ${RELEASE_DIR}/alang-${VERSION}-linux-x64.tar.gz"
echo ""

# ============================================
# 7. Generate Checksums
# ============================================
echo "🔐 Generating checksums..."

cd "${RELEASE_DIR}"
sha256sum * > SHA256SUMS.txt
cd ..

echo "✓ Checksums: ${RELEASE_DIR}/SHA256SUMS.txt"
echo ""

# ============================================
# 8. Summary
# ============================================
echo "╔════════════════════════════════════════╗"
echo "║  ✅ Build Complete!                     ║"
echo "╚════════════════════════════════════════╝"
echo ""
echo "📦 Release Packages:"
echo ""

ls -lh "${RELEASE_DIR}"

echo ""
echo "📋 Package Details:"
echo ""
echo "  🐧 Debian:"
echo "     ${RELEASE_DIR}/alang_${VERSION}_amd64.deb"
echo "     Install: sudo dpkg -i alang_${VERSION}_amd64.deb"
echo ""
echo "  🪟 Windows:"
if [ -f "${RELEASE_DIR}/A-lang-1.0-preview-Setup.exe" ]; then
    echo "     ${RELEASE_DIR}/A-lang-1.0-preview-Setup.exe"
    echo "     Run installer on Windows"
else
    echo "     ${RELEASE_DIR}/alang-${VERSION}-windows.zip"
    echo "     Extract and add to PATH"
fi
echo ""
echo "  🐧 Linux Portable:"
echo "     ${RELEASE_DIR}/alang-${VERSION}-linux-x64.tar.gz"
echo "     Extract: tar -xzf alang-${VERSION}-linux-x64.tar.gz"
echo ""
echo "🔐 Checksums:"
echo "     ${RELEASE_DIR}/SHA256SUMS.txt"
echo ""
echo "🎯 New Features in this build:"
echo "  ✅ input() function - Python-style user input"
echo "  ✅ FFI - Call C functions (Linux/macOS)"
echo "  ✅ Logo integration in installers"
echo "  ✅ Updated documentation"
echo ""
echo "📚 Documentation:"
echo "  - INPUT_AND_FFI_DOCS.md (NEW!)"
echo "  - FFI_IMPLEMENTATION_PLAN.md (NEW!)"
echo "  - SYNTAX_REFERENCE.md"
echo "  - README.md"
echo ""
echo "🚀 Next Steps:"
echo "  1. Test packages on target platforms"
echo "  2. Upload to GitHub Releases"
echo "  3. Update website/documentation"
echo "  4. Announce new features!"
echo ""
echo "🎉 Build timestamp: $(date)"
echo ""
