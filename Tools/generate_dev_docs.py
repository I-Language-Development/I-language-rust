"""
I Language generate dev docs tool.
Version: 1.0.0

Copyright (c) 2023-present I Language Development.

Permission is hereby granted, free of charge, to any person obtaining a
copy of this software and associated documentation files (the 'Software'),
to deal in the Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
"""


###########
# IMPORTS #
###########

import pathlib
import re
import typing


#############
# CONSTANTS #
#############

FILE_TYPES: typing.Dict[str, str] = {
    ".bash": "Bash script",
    ".bat": "Batch script",  # cspell: disable (following line)
    "fileextensions": "File extension list",  # cspell: disable (following line)
    ".iss": "Inno setup file ",
    ".rs": "Rust source code",
}


####################
# HELPER FUNCTIONS #
####################

# pylint: disable=redefined-outer-name
def generate_paths(
    base_path: pathlib.Path = pathlib.Path("src"),
) -> typing.List[pathlib.Path]:
    """Generates paths to add to the dev docs.

    Args:
        base_path (pathlib.Path): Path to start from. Defaults to pathlib.Path("src").

    Returns:
        typing.List[pathlib.Path]: List of paths to add to the dev docs.
    """

    result = []
    for path in base_path.iterdir():
        if "__" not in str(path) and "Cargo" not in str(path) and "target" not in str(path.parent):
            if path.is_dir():
                result.extend(generate_paths(path))
            else:
                result.append(path)

    return result


def sort_paths(
    path: pathlib.Path,
) -> typing.Tuple[bool, typing.List[str], int, bool, str]:
    """Key to sort paths by their names.

    Args:
        path (pathlib.Path): Path to sort.

    Returns:
        typing.Tuple[bool, typing.List[str], int, bool, str]: Tuple to sort with.
            First value: Whether the path is in the current directory of not.
            Second value: List of parent paths.
            Third value: Flag to indicate whether the path is called "main", "lib", "mod"
                         ("main" being the most important one) or something else.
            Fourth value: Whether the path has a file extension or not.
            Fifth value: Lowercase name of the path including the parent paths.
    """

    return (
        str(path.parent) != ".",
        [str(x) for x in pathlib.Path(str(path).lower().replace("src\\", "")).parents],
        {"main": 0, "lib": 1, "mod": 2}.get(path.stem, 3),
        path.suffix == "",
        str(path).lower().replace("src\\", ""),
    )

def parse_files(text: str) -> typing.List[str]:
    """Parses a markdown table and returns the list of filenames in that table.

    Args:
        text (str): Text containing the table.

    Returns:
        typing.List[str]: List of file names in the table.
    """

    result = []
    for line in text.splitlines():
        match = re.match(r"^\| \[`(?P<name>.+)`\]", line)
        if match is not None:
            result.append(match.group("name"))

    return result


#################
# MAIN FUNCTION #
#################

# TODO (ElBe): Generate class and function documentation for each file
# TODO (ElBe): Generate descriptions and more from doc-comments
if __name__ == "__main__":
    output_folder = pathlib.Path("Docs", "Docs", "Dev")
    index_table = [
        "# Source code documentation",
        "",
        "!!! note",
        "",
        "    The source can be found [here](https://github.com/I-Language-Development/I-language-rust).",
        "",
        "The files contain the source code documentation, meaning documentation about public and private functions or classes, variables and more.",
        "",
        "## List",
        "",
        "| Name | Type | Description |",
        "|:-----|:-----|:------------|",
    ]

    if not output_folder.exists():
        output_folder.mkdir()

    paths = sorted([p.relative_to("src") for p in generate_paths()], key=sort_paths)

    counter = 0  # pylint: disable=invalid-name
    for file in paths:
        path = pathlib.Path(output_folder, str(file).replace("src", "."))
        path.parent.mkdir(parents=True, exist_ok=True)

        if not pathlib.Path(str(path) + ".md").exists():
            with pathlib.Path(str(path) + ".md").open("w", encoding="utf-8") as _file:
                _file.write(f"# `{file.name}`\n")
            counter += 1

        file_type = (
            FILE_TYPES.get(path.suffix) if path.suffix else FILE_TYPES.get(path.stem)
        )
        index_table.append(
            f"| [`{path.relative_to('Docs', 'Docs', 'Dev').as_posix()}`](./{path.relative_to('Docs', 'Docs', 'Dev').as_posix()}.md) | {file_type if file_type else 'unknown'} | DESCRIPTION |"
        )

    # index.md file with a table of all source code files and a description for each
    index = pathlib.Path(output_folder, "index.md")

    # Workaround for the file being cleared when opened in "w" mode
    with index.open("r", encoding="utf-8") as _file:
        text = _file.read()

    with index.open("w", encoding="utf-8") as _file:
        if set(parse_files(text)) != set(parse_files("\n".join(index_table) + "\n")):
            _file.write("\n".join(index_table) + "\n")
            counter += 1
        else:
            _file.write(text)

    print(f"Generated {counter} files for source code documentation")
