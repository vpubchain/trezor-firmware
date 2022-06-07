"""
Defining handler logic for the Rust rows.
"""

from __future__ import annotations

import subprocess
from pathlib import Path
from typing import TYPE_CHECKING

from .row_handler_common import CORE_DIR, INVALID_FILE_PREFIX, CommonRow

if TYPE_CHECKING:  # pragma: no cover
    from .api import DataRow
    from .source_definition_cache import SourceDefinitionCache


class RustRow(CommonRow):
    language = "Rust"

    def __init__(self, source_def_cache: SourceDefinitionCache | None = None) -> None:
        super().__init__(source_def_cache)

    def _get_module_and_function(self, symbol_name: str) -> tuple[str, str]:
        # It usually ends with a strange hex
        symbol_name = self._get_rid_of_hex_suffix(symbol_name)
        items = symbol_name.split("::")

        # Filtering nonrelevant/strange items from there
        items = self.filter_nonrelevant_items(items)

        # There could be nothing left - leave it unrecognized
        if len(items) < 2:
            return "", ""

        # Function and possible struct are at the end
        function_name = items.pop()
        if items[-1][0].isupper():
            struct_name = items.pop()
        else:
            struct_name = ""

        file_path = "embed/rust/src"
        for item in items:
            file_path += f"/{item}"

        file_path = f"{file_path}.rs"

        if not Path(CORE_DIR / file_path).exists():
            file_path = f"{INVALID_FILE_PREFIX}{file_path}"

        if struct_name:
            struct_and_function = f"{struct_name}::{function_name}()"
        else:
            struct_and_function = f"{function_name}()"

        return file_path, struct_and_function

    def _get_definition(self, row: DataRow) -> str:
        # There is only a module, no need to locate any function definition
        if not row.func_name:
            return row.module_name

        line_num = self._get_line_num(row.module_name, row.func_name)
        if line_num:
            return f"{row.module_name}:{line_num}"
        else:
            return ""

    @staticmethod
    def _get_line_num(module_name: str, func_name: str) -> str:
        module_location = CORE_DIR / module_name
        func_name = func_name.replace("()", "").split("::")[-1]

        to_search = f"fn {func_name}[(<]"
        cmd = f'grep -m1 -n0 "{to_search}" {module_location} | cut -d: -f1'

        result = subprocess.run(
            cmd, stdout=subprocess.PIPE, text=True, shell=True
        ).stdout.strip()
        return result

    @staticmethod
    def filter_nonrelevant_items(items: list[str]) -> list[str]:
        def is_ok(item: str) -> bool:
            # Contained at the beginning and also in some strange strings
            # (_$LT$impl$u20$trezor_lib..micropython..ffi.._mp_obj_list_t$GT$)
            if "trezor_lib" in item:
                return False
            # Strings like _$u7b$$u7b$closure$u7d$$u7d$ or __BindgenBitfieldUnit$LT$Storage$GT$
            if "$" in item:
                return False
            # Things like TYPE, which is a variable inside a function
            if all(char.isupper() for char in item):
                return False

            return True

        return [item for item in items if is_ok(item)]

    @staticmethod
    def _get_rid_of_hex_suffix(name: str) -> str:
        # If there is a hex suffix, get rid of it
        # Doing that to improve readability and also to match the possibly
        # duplicated functions once being in "+" and once in "-", just with
        # different suffixes
        hex_length = 16
        try:
            int(name[-hex_length:].lower(), 16)
            without_hex = name[:-hex_length]
            # Also possibly get rid of ending "::h"
            if without_hex.endswith("::h"):
                return without_hex[:-3]
            else:
                return without_hex
        except ValueError:
            return name
