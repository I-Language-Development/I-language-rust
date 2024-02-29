# installer.bat

The installer for windows servers/windows operating systems without GUI support.

## Errors

### [`error`](https://github.com/I-Language-Development/I-language-rust/blob/main/src/Installer/Windows/Server/installer.bat#L101)

> Introduced in `v1.0.0-alpha3`

Base error function used for all other errors.

```batch title="installer.bat"

call :error "Something went wrong.", "Please report this error.", 1
: Will return following error message:
: (Empty line)
: Error:
:     Something went wrong.
:     Please report this error.
: (exit with code 1)

```

#### Arguments

`1` - First line of the error message. Does not include indentation at the beginning. Should be under 60 characters in length.

`2` - Second line of the error message. Does not include indentation at the beginning. Should be under 60 characters in length.

`3` - Exit code. See [here](#exit-codes).

#### Returns

Exit with the code specified as [`3`](#arguments).

### [`not_installed_error`](https://github.com/I-Language-Development/I-language-rust/blob/main/src/Installer/Windows/Server/installer.bat#L111)

Error raised, when something necessary for the installer to work is not installed.

```batch title="installer.bat"

call :not_installed_error "Something", "something"
: Will return following error message:
: (Empty line)
: Error:
:     Something seems not to be installed properly.
:     Please install something and try again.
: (exit with code 1)

```

#### Arguments

`1` - The name of the thing not installed. First letter should be uppercase.

`2` - The name of the thing not installed. First letter should be lowercase.

#### Returns

Exit with error code 1.

### [`os_error`](https://github.com/I-Language-Development/I-language-rust/blob/main/src/Installer/Windows/Server/installer.bat#L114)

Error raised, when the OS the installer is ran on is not windows.

```batch title="installer.bat"

call :os_error
: Will return following error message:
: (Empty line)
: Error:
:     Only windows is supported by this installer.
:     Please use the installer for your operating system instead.
: (exit with code 2)

```

#### Returns

Exit with error code 2.

## Exit codes

| Exit code | Description                                       | Example                  |
| --------: | :------------------------------------------------ | :----------------------- |
|         1 | See [`not_installed_error`](#not_installed_error) | Python is not installed. |
|         2 | See [`os_error`](#os_error)                       | OS is not windows.       |
