rem I Language windows file extension associate tool.
rem Version: 1.0.0
rem
rem Copyright (c) 2023-present I Language Development.
rem
rem Permission is hereby granted, free of charge, to any person obtaining a
rem copy of this software and associated documentation files (the 'Software'),
rem to deal in the Software without restriction, including without limitation
rem the rights to use, copy, modify, merge, publish, distribute, sublicense,
rem and/or sell copies of the Software, and to permit persons to whom the
rem Software is furnished to do so, subject to the following conditions:
rem
rem The above copyright notice and this permission notice shall be included in
rem all copies or substantial portions of the Software.
rem
rem THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS
rem OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
rem FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
rem AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
rem LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
rem FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
rem DEALINGS IN THE SOFTWARE.

@echo off

rem Run as new process with administrator privileges
net session >nul 2>&1 || (powershell start -verb runas '"%~0"' %1 & exit /b)

rem Global registry path
set registry_path="HKEY_LOCAL_MACHINE\Software\CLASSES"

rem Add file extension to registry
reg add "%registry_path%\ILanguage" /d "I Language File" /f
reg add "%registry_path%\ILanguage" /v "AppUserModelID" /d "ILanguage.ILanguage" /f
reg add "%registry_path%\ILanguage\DefaultIcon" /d "%PROGRAMFILES%\ilanguage\icon.ico" /f
reg add "%registry_path%\ILanguage\Shell\open\command" /d "%PROGRAMFILES%\ilanguage\shell.exe" /f
reg add "%registry_path%\.il" /d "ILanguage" /f
reg add "%registry_path%\.il" /v "Content Type" /d "text/plain" /f
reg add "%registry_path%\.il\OpenWithProgids" /d "ILanguage" /f
