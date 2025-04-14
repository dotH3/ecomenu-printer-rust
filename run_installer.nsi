!define APP_NAME "Ecomenu Printer"
!define APP_EXE "ecomenu-printer.exe"
!define BAT_NAME "run-hidden.bat"

OutFile "EcomenuPrinter_Installer.exe"
InstallDir "$PROGRAMFILES\${APP_NAME}"
RequestExecutionLevel admin

Section "Instalar"
    SetOutPath $INSTDIR

    File "target\release\${APP_EXE}"
    File "${BAT_NAME}"
    File "gs10050w64.exe"

    CreateShortCut "$SMSTARTUP\${APP_NAME}.lnk" "$INSTDIR\${BAT_NAME}" "" "$INSTDIR\${BAT_NAME}" 0 SW_SHOWNORMAL
    CreateShortCut "$DESKTOP\${APP_NAME}.lnk" "$INSTDIR\${BAT_NAME}"

    MessageBox MB_YESNO|MB_ICONQUESTION "Deseas instalar Ghostscript?" IDYES InstallGS IDNO NoInstallGS
    goto end

InstallGS:
    ; Si el usuario desea instalar Ghostscript, se ejecuta el instalador
    ExecWait '"$INSTDIR\gs10050w64.exe"'
    goto end

NoInstallGS:
    ; Si el usuario no desea instalar Ghostscript, no hace nada
    goto end

end:
SectionEnd
