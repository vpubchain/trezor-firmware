"""
Example usage of the bin_size tool.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

from bin_size.api import DataRow
from bin_size.binary_size import BinarySize
from plugins.statistics import StatisticsPlugin

HERE = Path(__file__).resolve().parent
CORE_DIR = HERE.parent.parent.parent

if len(sys.argv) > 1:
    BIN_TO_ANALYZE = sys.argv[1]
else:
    BIN_TO_ANALYZE = CORE_DIR / "build/firmware/firmware.elf"  # type: ignore
FILE_TO_SAVE = HERE / "size_binary_firmware_elf_results.txt"


def binary_size_example() -> None:
    BinarySize().load_file(
        BIN_TO_ANALYZE, sections=(".flash", ".flash2")
    ).add_basic_info().aggregate().filter(
        lambda row: row.size > 50 or row.language == "Rust"
    ).sort(
        lambda row: row.size, reverse=True
    ).add_definitions(
        lambda row: row.language != "C"
    ).show(
        FILE_TO_SAVE
    )


def binary_size_example_complete_data() -> None:
    (
        BinarySize()
        .load_file(BIN_TO_ANALYZE, sections=(".flash", ".flash2"))
        .use_map_file(
            CORE_DIR / "build/firmware/firmware.map", sections=(".flash", ".flash2")
        )
        .add_basic_info()
        .aggregate()
        .sort()
        .add_definitions()
        .show(HERE / "COMPLETE_DATA.txt", debug=True)
    )


def statistics_example_mpy_apps() -> None:
    def _apps_categories(row: DataRow) -> str | None:
        pattern = r"^src/apps/(\w+)/"  # dir name after apps/
        match = re.search(pattern, row.module_name)
        if not match:
            return None
        else:
            return match.group(1)

    BS = (
        BinarySize()
        .load_file(BIN_TO_ANALYZE, sections=(".flash", ".flash2"))
        .add_basic_info()
    )
    StatisticsPlugin(BS, _apps_categories).show()


def statistics_example_own_groups() -> None:
    def _categories_func(row: DataRow) -> str | None:
        if "ui" in row.source_definition.lower():
            return "UI"
        elif (
            "crypto" in row.source_definition.lower()
            or "vendor/secp256k1-zkp" in row.source_definition.lower()
        ):
            return "Crypto"
        elif "trezor" in row.source_definition.lower():
            return "Trezor"
        elif "micropython" in row.source_definition.lower():
            return "Micropython env"
        elif "bitcoin" in row.source_definition.lower():
            return "Bitcoin"
        elif "ethereum" in row.source_definition.lower():
            return "Ethereum"
        elif "apps/management" in row.source_definition.lower():
            return "Management"
        elif "src/apps/" in row.source_definition.lower():
            return "Other apps"
        elif row.language == "Rust":
            return "Rust"

    BS = (
        BinarySize()
        .load_file(BIN_TO_ANALYZE, sections=(".flash", ".flash2"))
        .use_map_file(
            CORE_DIR / "build/firmware/firmware.map", sections=(".flash", ".flash2")
        )
        .add_basic_info()
        .aggregate()
        .add_definitions()
    )
    StatisticsPlugin(BS, _categories_func).show(include_none=True)


if __name__ == "__main__":
    binary_size_example()
    binary_size_example_complete_data()
    statistics_example_mpy_apps()
    statistics_example_own_groups()
