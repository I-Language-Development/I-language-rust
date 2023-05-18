#!/bin/bash

# I Language linux installer.
# Version: 1.0.0
#
# Copyright (c) 2023-present I Language Development.
#
# Permission is hereby granted, free of charge, to any person obtaining a
# copy of this software and associated documentation files (the 'Software'),
# to deal in the Software without restriction, including without limitation
# the rights to use, copy, modify, merge, publish, distribute, sublicense,
# and/or sell copies of the Software, and to permit persons to whom the
# Software is furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in
# all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS
# OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
# FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
# DEALINGS IN THE SOFTWARE.

# Show header and ask weather to continue
echo -e -n "\033[34;1m"
echo "###    #"
echo " #     #         ##   #    #  ####  #    #   ##    ####  ######"
echo " #     #        #  #  ##   # #    # #    #  #  #  #    # #"
echo " #     #       #    # # #  # #      #    # #    # #      #####"
echo " #     #       ###### #  # # #  ### #    # ###### #  ### #"
echo " #     #       #    # #   ## #    # #    # #    # #    # #"
echo "###    ####### #    # #    #  ####   ####  #    #  ####  ######"
echo -e "\033[0m"
echo ""
echo "    This will install the I programming language on your computer."
echo "    Existing installations will be replaced and configuration will be overwritten."
echo ""
echo -e -n "    \033[30;107m[ ]\033[0m Continue? (Ctrl-C to cancel)"
read -n 1 -s
echo -e -n "\e[2K\r"
echo -e "    \033[30;107m[x]\033[0m Continue? (Ctrl-C to cancel)"

# Raise error when the OS is not linux
if [[ ! "$OSTYPE" == "linux-gnu"* ]]; then
	echo ""
	echo -e "\033[31;1mError\033[0m"
	echo "    Currently only linux is supported."
	echo "    Please use the installer for your operating system instead."
	exit 2
fi

# Test for git
if ! git --version &> /dev/null; then
	echo ""
	echo -e "\033[31;1mError\033[0m"
	echo "    Git seems not to be installed properly."
	echo "    Please install git and try again."
	exit 1
fi

# Test for python and get edition
if ! python3 --version &> /dev/null; then
	if ! python --version &> /dev/null; then
		echo ""
		echo -e "\033[31;1mError\033[0m"
		echo "    Python seems not to be installed properly."
		echo "    Please install python and try again."
		exit 1
	else
		python=python
	fi
else
	python=python3
fi

# Ask where to install the I programming language
while :; do
	echo ""
	read -p "    Where do you want to install the I programming language? (Default: \"/usr/bin\") " installLocation

	if [ "$installLocation." == "." ]; then
		installLocation="/usr/bin"
		break
	fi

	if [ ! -d "$installLocation" ]; then
		echo ""
		echo -e "\033[31;1mError\033[0m"
		echo "    $installLocation does not exist."
		echo "    Please create the directory if wanted and try again."
		sleep 3
		for _ in {1..6}; do printf "\e[A\e[K"; done
		continue
	fi

	break
done

# Install latest release
echo -e "\033[32;1mInstalling\033[0m"

cargoTomlUrl="https://raw.githubusercontent.com/I-Language-Development/I-language-rust/main/Cargo.toml"
latestRelease="https://raw.githubusercontent.com/I-Language-Development/I-language-rust/releases/$($python -c "exec(\"import tomllib\nfrom urllib import request\nwith request.urlopen('$cargoTomlUrl') as file: print(tomllib.loads(file.read().decode())['package']['version'])\")")"

echo -e -n "    [\033[34;1m...\033[0m] Installing latest release from $latestRelease (v$($python -c "exec(\"import tomllib\nfrom urllib import request\nwith request.urlopen('$cargoTomlUrl') as file: print(tomllib.loads(file.read().decode())['package']['version'])\")"))"

cd "$installLocation" || exit 3
curl -s "https://raw.githubusercontent.com/I-Language-Development/I-language-rust/releases/$latestRelease" > ilang
echo -e -n "\e[2K\r"
echo -e "    [\033[32;1m✓\033[0m] Installing latest release from $latestRelease (v$($python -c "exec(\"import tomllib\nfrom urllib import request\nwith request.urlopen('$cargoTomlUrl') as file: print(tomllib.loads(file.read().decode())['package']['version'])\")"))"
