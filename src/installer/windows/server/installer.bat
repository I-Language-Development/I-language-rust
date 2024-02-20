: I Language windows server installer.
: Version: 1.0.0

: Copyright (c) 2023-present I Language Development.

: Permission is hereby granted, free of charge, to any person obtaining a
: copy of this software and associated documentation files (the 'Software'),
: to deal in the Software without restriction, including without limitation
: the rights to use, copy, modify, merge, publish, distribute, sublicense,
: and/or sell copies of the Software, and to permit persons to whom the
: Software is furnished to do so, subject to the following conditions:

: The above copyright notice and this permission notice shall be included in
: all copies or substantial portions of the Software.

: THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS
: OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
: FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
: AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
: LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
: FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
: DEALINGS IN THE SOFTWARE.

@echo off

: Run as new process with administrator privileges
net session >nul 2>&1 || (powershell start -verb runas '%~0' %1 & exit /b)

: Show header and ask weather to continue
echo ###    #
echo  #     #         ##   #    #  ####  #    #   ##    ####  ######
echo  #     #        #  #  ##   # #    # #    #  #  #  #    # #
echo  #     #       #    # # #  # #      #    # #    # #      #####
echo  #     #       ###### #  # # #  ### #    # ###### #  ### #
echo  #     #       #    # #   ## #    # #    # #    # #    # #
echo ###    ####### #    # #    #  ####   ####  #    #  ####  ######
echo:
echo:
echo     This will install the I programming language on your computer.
echo     Existing installations will be replaced and configuration will be overwritten.
echo:
set /p _="%NUL%    Continue? (Ctrl-C to cancel)"

: Raise error when the OS is not windows
IF NOT "%OS%"=="Windows_NT" (
	call :os_error
)

: Test for python and get edition
py --version > NUL 2>&1

IF ERRORLEVEL 1 (
	call :not_installed_error "Python", "python"
)

: Ask where to install the I programming language
:installLocationLoop
echo:
set /p installLocation="    Where do you want to install the I programming language? (Default: "%PROGRAMFILES%") "

IF "%instalLocation%"=="" (
	set installLocation="%PROGRAMFILES%"
)

IF NOT EXIST %installLocation% (
	echo:
	echo Error:
	echo     %installLocation% does not exist.
	echo     Please create the directory if wanted and try again.

	goto :installLocationLoop
)

: Install the latest release
echo:
echo Installing

curl -s "https://raw.githubusercontent.com/I-Language-Development/I-language-rust/main/Tools/get_release_version.py" > %TEMP%\ILanguageInstaller_get_release_version.py
curl -s "https://raw.githubusercontent.com/I-Language-Development/I-language-rust/main/Cargo.toml" > %TEMP%\Cargo.toml
cd %TEMP%
python %TEMP%\ILanguageInstaller_get_release_version.py > %TEMP%\ILanguageInstaller.tmp

set /p latestReleaseVersion=<%TEMP%\ILanguageInstaller.tmp
del /q %TEMP%\ILanguageInstaller.tmp
del /q %TEMP%\ILanguageInstaller_get_release_version.py

set latestReleaseUrl="https://raw.githubusercontent.com/I-Language-Development/I-language-rust/releases/%latestReleaseVersion%.exe"

echo     Installing latest release from %latestReleaseUrl% (v%latestReleaseVersion%)

cd %installLocation%
mkdir "I Language"
cd "I Language"
curl -s %latestReleaseUrl% > ilang.exe
set PATH=%PATH%;%installLocation%\I Language

pause
exit /b 0

: Errors
:error
	echo:
	echo Error:
	echo     %~1
	echo     %~2

	pause
	exit /b %~3

:not_installed_error
	call :error "%~1 seems not to be installed properly.", "Please install %~2 and try again.", 1

:os_error
	call :error "Only windows is supported by this installer.", "Please use the installer for your operating system instead.", 2
