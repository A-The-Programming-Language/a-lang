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
