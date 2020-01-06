#setup vars
Name "Rocks DB"
OutFile "build\setup_rocks_db.exe"
InstallDir "$PROGRAMFILES\RocksDb"
RequestExecutionLevel highest
SetCompressor /SOLID lzma

#version stuff
VIAddVersionKey "ProductName" "Rocks DB"
VIAddVersionKey "CompanyName" "Gwilyn Saunders"
VIAddVersionKey "LegalTrademarks" "MIT"
VIAddVersionKey "LegalCopyright" "© Gwilyn Saunders"
VIAddVersionKey "FileDescription" "Setup installer for Rocks DB"
VIAddVersionKey "FileVersion" "1.0"
VIAddVersionKey "ProductVersion" "1.0.0.0"
VIProductVersion "1.0.0.0"

LicenseData "..\LICENSE.txt"
DirText "Select an install directory"

#imports
!include "LogicLib.nsh"
!include "Sections.nsh"

#layout
Page license
Page directory
Page instfiles
UninstPage uninstConfirm
UninstPage instfiles

Section "Common Files (Required)"
    SectionIn RO
    SetOutPath $INSTDIR
    
    #copy files
    File "build\release\RocksDB.exe"
    File "..\lib\target\release\rocks.dll"
    File "..\description-database.txt"
    
    File "$%QTDIR%\bin\Qt5Core.dll"
    File "$%QTDIR%\bin\Qt5Gui.dll"
    File "$%QTDIR%\bin\Qt5Widgets.dll"
    
    File "$%QTDIR%\bin\libgcc_s_dw2-1.dll"
    File "$%QTDIR%\bin\libstdc++-6.dll"
    File "$%QTDIR%\bin\libwinpthread-1.dll"
    File "$%QTDIR%\bin\icuin52.dll"
    File "$%QTDIR%\bin\icudt52.dll"
    File "$%QTDIR%\bin\icuuc52.dll"
    
    File "..\LICENSE.txt"
    
    #create uninstaller
    WriteUninstaller "$INSTDIR\uninstall.exe"
    
    #create shortcuts
    CreateDirectory "$SMPROGRAMS\Rocks DB"
    CreateShortCut "$SMPROGRAMS\Rocks DB\Rocks DB.lnk" "$INSTDIR\RocksDB.exe"
    CreateShortCut "$SMPROGRAMS\Rocks DB\Uninstall.lnk" "$INSTDIR\uninstall.exe"
SectionEnd

Section "Uninstall"
    #remove program files
    Delete "$INSTDIR\RocksDB.exe"
    Delete "$INSTDIR\description-database.txt"
    Delete "$INSTDIR\*.dll"
    Delete "$INSTDIR\LICENSE.txt"
    Delete "$INSTDIR\uninstall.exe"
    RMDir "$INSTDIR"
    
    #remove shortcut files
    Delete "$SMPROGRAMS\Rocks DB\Rocks DB.lnk"
    Delete "$SMPROGRAMS\Rocks DB\Uninstall.lnk"
    RMDir "$SMPROGRAMS\Rocks DB"
SectionEnd
