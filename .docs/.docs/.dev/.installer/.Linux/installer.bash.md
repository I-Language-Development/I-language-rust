# installer.bash

The installer for Linux operating systems without GUI support.

## Errors

### [`error`](https://github.com/I-Language-Development/I-language-rust/blob/main/src/Installer/Linux/installer.bash#L50)

> Introduced in `v1.0.0-alpha3`

Base error function used for all other errors.

```bash title="installer.bash"

error "Something went wrong." "Please report this error." "Do this here, not there." 1
# Will return following error message:
# (Empty line)
# Error: (Red and bold text)
#     Something went wrong.
#     Please report this error.
#     Do this here, not there.
# (exit with code 1)

```

#### Arguments

`1-penultimate Argument` - Lines of the error message. Does not include indentation at the beginning. Should be under 60 characters in length. For each

`last Argument` - Exit code. See [here](#exit-codes).

#### Returns

Exit with the code specified as [`last Argument`](#arguments).

### [`not_installed_error`](https://github.com/I-Language-Development/I-language-rust/blob/main/src/Installer/Linux/installer.bash#L63)

Error raised, when something necessary for the installer to work is not installed.

```bash title="installer.bash"

not_installed_error "something"
# Will return following error message:
# (Empty line)
# Error: (Red and bold text)
#     Something seems not to be installed properly.
#     Please install something and try again.
# (exit with code 1)

```

#### Arguments

`1` - The name of the thing not installed.

#### Returns

Exit with error code 1.

### [`os_error`](https://github.com/I-Language-Development/I-language-rust/blob/main/src/Installer/Linux/installer.bash#L68)

Error raised, when the OS the installer is ran on is not Linux.

```bash title="installer.bash"

os_error
# Will return following error message:
# (Empty line)
# Error: (Red and bold text)
#     Currently only linux is supported by this installer.
#     Please use the installer for your operating system instead.
# (exit with code 2)

```

#### Returns

Exit with error code 2.

## Exit codes

| Exit code | Description                                       | Example                  |
| --------: | :------------------------------------------------ | :----------------------- |
|         1 | See [`not_installed_error`](#not_installed_error) | Python is not installed. |
|         2 | See [`os_error`](#os_error)                       | OS is not linux.       |
