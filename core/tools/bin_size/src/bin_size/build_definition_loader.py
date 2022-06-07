"""
Gets info about the build definitions from a binary.

Using `nm` tool for getting those - https://linux.die.net/man/1/nm
"""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

from .api import BuildDefinitionLoaderAPI


class BuildDefinitionLoader(BuildDefinitionLoaderAPI):
    def __init__(self) -> None:
        self.symbol_build_definitions: dict[str, str] = {}

    def load(self, bin_file: str | Path) -> None:
        nm_output = self._get_nm_output(bin_file)
        for line in nm_output.splitlines():
            split_line = line.split()
            if len(split_line) < 4:
                continue
            else:
                _size, _mode, symbol, sym_def = split_line
                # Sanitating the path to be relative to the core dir
                if "trezor-firmware/core/" in sym_def:
                    sym_def = sym_def.split("trezor-firmware/core/", maxsplit=1)[1]
                self.symbol_build_definitions[symbol] = sym_def

    def get(self, symbol_name: str) -> str | None:
        return self.symbol_build_definitions.get(symbol_name, None)

    @staticmethod
    def _get_nm_output(bin_file: str | Path) -> str:
        nm_cmd = f"arm-none-eabi-nm --line-numbers --radix=dec --size-sort {bin_file}"
        print(f"Running CMD: `{nm_cmd}`")
        result = subprocess.run(nm_cmd, stdout=subprocess.PIPE, text=True, shell=True)

        if result.returncode != 0:
            print("command failed, see output above")
            sys.exit(1)

        return result.stdout
