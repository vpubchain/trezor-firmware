"""
Grouping symbols in binary into coherent categories.
"""

from __future__ import annotations

import sys
from pathlib import Path
from typing import Callable

from bin_size import BinarySize, DataRow, StatisticsPlugin

HERE = Path(__file__).resolve().parent
CORE_DIR = HERE.parent

if len(sys.argv) > 1:
    BIN_TO_ANALYZE = sys.argv[1]
else:
    BIN_TO_ANALYZE = CORE_DIR / "build/firmware/firmware.elf"  # type: ignore
FILE_TO_SAVE = HERE / "size_binary_firmware_elf_results.txt"


def _categories_func(row: DataRow) -> str | None:
    # Defined inside the function so it can be seen in the function definition
    # (which is optionally printed)
    CATEGORIES: dict[str, Callable[[DataRow], bool]] = {
        "UI": lambda row: (
            row.source_definition.startswith("src/trezor/ui/")
            or row.source_definition.startswith("embed/extmod/modtrezorui/")
        ),
        "Crypto": lambda row: (
            row.source_definition.startswith("vendor/trezor-crypto/")
            or row.source_definition.startswith("src/trezor/crypto/")
            or row.source_definition.startswith("embed/extmod/modtrezorcrypto/")
        ),
        "Secp256": lambda row: (
            row.source_definition.startswith("vendor/secp256k1-zkp/")
        ),
        "Storage": lambda row: (
            row.source_definition.startswith("src/storage/")
            or row.source_definition.startswith("vendor/trezor-storage/")
        ),
        "Micropython": lambda row: row.source_definition.startswith(
            "vendor/micropython/"
        ),
        "Bitcoin app": lambda row: row.source_definition.startswith(
            "src/apps/bitcoin/"
        ),
        "Ethereum app": lambda row: row.source_definition.startswith(
            "src/apps/ethereum/"
        ),
        "Monero app": lambda row: row.source_definition.startswith("src/apps/monero/"),
        "Cardano app": lambda row: row.source_definition.startswith(
            "src/apps/cardano/"
        ),
        "Management app": lambda row: row.source_definition.startswith(
            "src/apps/management/"
        ),
        "Common apps": lambda row: row.source_definition.startswith("src/apps/common/"),
        "Webauthn app": lambda row: row.source_definition.startswith(
            "src/apps/webauthn/"
        ),
        "Altcoin apps": lambda row: (
            row.source_definition.startswith("src/apps/nem/")
            or row.source_definition.startswith("src/apps/stellar/")
            or row.source_definition.startswith("src/apps/eos/")
            or row.source_definition.startswith("src/apps/tezos/")
            or row.source_definition.startswith("src/apps/ripple/")
            or row.source_definition.startswith("src/apps/zcash/")
            or row.source_definition.startswith("src/apps/binance/")
        ),
        "Other apps": lambda row: row.source_definition.startswith("src/apps/"),
        "Rest of src/": lambda row: row.source_definition.startswith("src/"),
        "Embed unix": lambda row: row.source_definition.startswith("embed/unix/"),
        "Embed firmware": lambda row: row.source_definition.startswith(
            "embed/firmware/"
        ),
        "Trezorhal": lambda row: row.source_definition.startswith("embed/trezorhal/"),
        "Trezorio": lambda row: row.source_definition.startswith(
            "embed/extmod/modtrezorio/"
        ),
        "Trezorconfig": lambda row: row.source_definition.startswith(
            "embed/extmod/modtrezorconfig/"
        ),
        "Trezorutils": lambda row: row.source_definition.startswith(
            "embed/extmod/modtrezorutils/"
        ),
        "Rust UI": lambda row: row.source_definition.startswith("embed/rust/src/ui"),
        "Rust": lambda row: (
            row.language == "Rust"
            or row.source_definition.startswith("embed/rust/")
            or row.source_definition.startswith("/cargo/registry")
        ),
    }

    for category, func in CATEGORIES.items():
        if func(row):
            return category
    return None


def show_categories_statistics(
    STATS: StatisticsPlugin, include_categories_func: bool = False
) -> None:
    STATS.show(include_none=True, include_categories_func=include_categories_func)


def show_data_with_categories(
    STATS: StatisticsPlugin, file_to_save: str | Path | None = None
) -> None:
    STATS.show_data_with_categories(file_to_save, include_none=True)


def show_only_one_category(
    BS: BinarySize, category: str | None, file_to_save: str | Path | None = None
) -> None:
    BS.filter(lambda row: _categories_func(row) == category).show(
        file_to_save, debug=True
    )


def show_raw_bloaty_data() -> None:
    BinarySize().load_file(BIN_TO_ANALYZE, sections=(".flash", ".flash2")).show(
        f"{FILE_TO_SAVE}_no_aggregation"
    )


if __name__ == "__main__":
    BS = (
        BinarySize()
        .load_file(BIN_TO_ANALYZE, sections=(".flash", ".flash2"))
        .add_basic_info()
        .aggregate()
        .sort()
        .add_definitions()
    )
    STATS = StatisticsPlugin(BS, _categories_func)

    show_categories_statistics(STATS, include_categories_func=True)
    show_data_with_categories(STATS, FILE_TO_SAVE)
    show_only_one_category(BS, None, f"{FILE_TO_SAVE}_None")
