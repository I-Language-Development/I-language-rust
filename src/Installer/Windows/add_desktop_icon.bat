: I Language windows desktop icon tool.
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

: Copied from https://superuser.com/a/392082
SETLOCAL ENABLEDELAYEDEXPANSION
SET LinkName=I Language Shell
SET Esc_LinkDest=%%HOMEDRIVE%%%%HOMEPATH%%\Desktop\!LinkName!.lnk
SET Esc_LinkTarget=%%SYSTEMROOT%%\notepad.exe
SET cSctVBS=CreateShortcut.vbs
SET LOG=".\%~N0_runtime.log"
((
	echo Set oWS = WScript.CreateObject^("WScript.Shell"^)
	echo sLinkFile = oWS.ExpandEnvironmentStrings^("!Esc_LinkDest!"^)
	echo Set oLink = oWS.CreateShortcut^(sLinkFile^)
	echo oLink.TargetPath = oWS.ExpandEnvironmentStrings^("!Esc_LinkTarget!"^)
	echo oLink.Save
)1>!cSctVBS!
cscript //nologo .\!cSctVBS!
DEL !cSctVBS! /f /q
)1>>!LOG! 2>>&1
