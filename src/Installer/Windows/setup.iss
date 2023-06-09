; I Language windows installer.
; Version: 1.1.0

; Copyright (c) 2023-present I Language Development.

; Permission is hereby granted, free of charge, to any person obtaining a
; copy of this software and associated documentation files (the 'Software'),
; to deal in the Software without restriction, including without limitation
; the rights to use, copy, modify, merge, publish, distribute, sublicense,
; and/or sell copies of the Software, and to permit persons to whom the
; Software is furnished to do so, subject to the following conditions:

; The above copyright notice and this permission notice shall be included in
; all copies or substantial portions of the Software.

; THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS
; OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
; FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
; AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
; LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
; FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
; DEALINGS IN THE SOFTWARE.

[Setup]
AppName=I Language
AppId=ILanguage
AppVersion=1.0.0
AppPublisher=I Language Development
AppPublisherURL=https://www.i-language-development.github.io/
AppSupportURL=https://www.i-language-development.github.io/
AppUpdatesURL=https://www.i-language-development.github.io/
VersionInfoCopyright=Copyright (c) 2023-present I Language Development.
AppMutex=ILanguageSetupMutex,Global\ILanguageSetupMutex
SetupMutex=ILanguageSetupMutex,Global\ILanguageSetupMutex
WizardStyle=modern
DefaultDirName="{autopf}/I Language"
DefaultGroupName=I Language
PrivilegesRequiredOverridesAllowed=commandline
AllowNoIcons=yes
Compression=lzma2/max
SolidCompression=yes
;Uninstallable=not PortableCheck
UninstallDisplayIcon={app}/Uninstall.exe
UsePreviousLanguage=no
LicenseFile="../../../LICENSE.txt"
ShowLanguageDialog=yes

[Languages]
Name: english; MessagesFile: "../../../Translations/Installer/default.isl"

[Tasks]
Name: associateFileExtension; Description: "{cm:AssociateFileExtension,.il,I Language}"
Name: addToPath; Description: "{cm:AddToPath,I Language}"
Name: createDesktopIcon; Description: "{cm:CreateDesktopIcon}"; Flags: unchecked
Name: installForAllUsers; Description: "{cm:InstallForAllUsers}"; Flags: unchecked

[InstallDelete]
Type: files; Name: {autodesktop}/I Language.ink; Tasks: not createDesktopIcon
Type: filesandordirs; Name: "{app}/src/Installer"
Type: filesandordirs; Name: "{app}/Translations/Installer"
Type: filesandordirs; Name: "{app}/.github"
Type: filesandordirs; Name: "{app}/.vscode"

[Files]
Source: "../../../LICENSE.txt"; DestDir: "{app}"; Flags: ignoreversion touch
Source: "../../../.github/logo.ico"; DestDir: "{app}"; Flags: ignoreversion touch
; Todo (ElBe): Clone README as txt
Source: "../../../.github/README.md"; DestDir: "{app}/Docs"; Flags: ignoreversion touch isreadme
Source: "../../../Modules/*"; DestDir: "{app}/Modules/stdlib"; Flags: ignoreversion touch
Source: "../../../Docs/*"; DestDir: "{app}/Docs"; Flags: ignoreversion touch
Source: "../../../Tools/*"; DestDir: "{app}/Tools"; Flags: ignoreversion touch
Source: "*.bat"; DestDir: "{app}/Tools"; Flags: ignoreversion touch

[INI]
Filename: "{app}/config.ini"; Section: "Settings"; Key: "UseUnstableFeatures"; String: "yes"

[UninstallDelete]
Type: files; Name: "{app}/config.ini"
Type: filesandordirs; Name: "{app}/Modules"

[Icons]
; Todo (ElBe): Add icons
; Coming soon...

[Run]
Filename: "{app}/Tools/add_desktop_icon.bat"; StatusMsg: "{cm:CreatingDesktopIcon}"; Parameters: {app}; Tasks: createDesktopIcon; Flags: runhidden
Filename: "{app}/Tools/associate_file_extension.bat"; StatusMsg: "{cm:AssociatingFileExtension,.il,I Language}"; Tasks: associateFileExtension; Flags: runhidden
Filename: "{app}/Tools/add_to_path.bat"; StatusMsg: "{cm:AddingToPath,I Language}"; Parameters: {app}; Tasks: addToPath; Flags: runhidden

[UninstallRun]
Filename: "{app}/Tools/unassociate_file_extension.bat"; RunOnceId: "{cm:UnassociatingFileExtension,.il,I Language}"; Tasks: associateFileExtension; Flags: runhidden
