"""
I Language replace version tool.
Version: 1.2.0

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

import os
import sys
from typing import (
    Any,
    Dict,
    Optional,
)

sys.path.append(
    os.path.dirname(__file__)
)  # When using MkDocs, the code is accessed outside of
# this directory, making it not able to import the necessary tool.
# This ensures "get_release_version" can be imported
import get_release_version  # pylint: disable=C0413  # noqa


########
# MAIN #
########


def on_page_markdown(markdown: str, **kwargs: Optional[Dict[Any, Any]]) -> str:
    """Replaces variables defined as `{{NAME}}` with their values.

    Args:
        markdown (str): Original markdown.

    Returns:
        str: Modified markdown.
    """

    return markdown.replace("{{ VERSION }}", get_release_version.main())

