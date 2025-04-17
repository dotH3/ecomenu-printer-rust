!define APP_NAME "Ecomenu Printer"
!define APP_EXE "ecomenu-printer.exe"
!define BAT_NAME "run-hidden.bat"
!define VERSION "v0.0.6-alpha" ; Define la versión aquí

OutFile "EcomenuPrinter_Installer_${VERSION}.exe"  ; Incluye la versión en el nombre del archivo

InstallDir "$PROGRAMFILES\${APP_NAME}"
RequestExecutionLevel admin

Section "Instalar"
    SetOutPath $INSTDIR

    File "target\release\${APP_EXE}"
    File "${BAT_NAME}"
    File "gs10050w64.exe"

    CreateShortCut "$SMSTARTUP\${APP_NAME}.lnk" "$INSTDIR\${BAT_NAME}" "" "$INSTDIR\${BAT_NAME}" 0 SW_SHOWNORMAL
    CreateShortCut "$DESKTOP\${APP_NAME}.lnk" "$INSTDIR\${BAT_NAME}"

    ; Elimina la pregunta al usuario y ejecuta directamente el instalador de Ghostscript
    ExecWait '"$INSTDIR\gs10050w64.exe"'
    goto end

end:
SectionEnd
