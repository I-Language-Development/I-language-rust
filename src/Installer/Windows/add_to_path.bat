: I Language windows add to path tool.
: Version: 1.0.0
rem
: Copyright (c) 2023-present I Language Development.
rem
: Permission is hereby granted, free of charge, to any person obtaining a
: copy of this software and associated documentation files (the 'Software'),
: to deal in the Software without restriction, including without limitation
: the rights to use, copy, modify, merge, publish, distribute, sublicense,
: and/or sell copies of the Software, and to permit persons to whom the
: Software is furnished to do so, subject to the following conditions:
rem
: The above copyright notice and this permission notice shall be included in
: all copies or substantial portions of the Software.
rem
: THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS
: OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
: FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
: AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
: LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
: FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
: DEALINGS IN THE SOFTWARE.

@echo off

: Run as new process with administrator privileges
net session >nul 2>&1 || (powershell start -verb runas '"%~0"' %1 & exit /b)

set PATH=%PATH%;%1
