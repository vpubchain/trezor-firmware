#!/usr/bin/env python3
"""
Checking the size of the flash sections in firmware binary.

Prints the info and fails if there is not enough free space.
"""

from __future__ import annotations

import sys
from pathlib import Path

from bin_size import get_flash_sections_sizes_kb

HERE = Path(__file__).parent
CORE_DIR = HERE.parent

if len(sys.argv) > 1:
    BIN_TO_ANALYZE = sys.argv[1]
else:
    BIN_TO_ANALYZE = CORE_DIR / "build/firmware/firmware.elf"  # type: ignore

# Comes from `core/embed/firmware/memory_T.ld`
FLASH_SIZE_KB = 768
FLASH_2_SIZE_KB = 896

MIN_KB_FREE_TO_SUCCEED = 15

EXIT_CODE = 0


def report_section(name: str, size: int, max_size: int) -> None:
    percentage = 100 * size / max_size
    free = max_size - size

    print(f"{name}: {size}K / {max_size}K ({percentage:.2f}%) - {free}K free")

    global EXIT_CODE
    if free < MIN_KB_FREE_TO_SUCCEED:
        print(
            f"Less free space in {name} ({free}K) than expected ({MIN_KB_FREE_TO_SUCCEED}K). Failing"
        )
        EXIT_CODE = 1  # type: ignore


if __name__ == "__main__":
    print(f"Analyzing {BIN_TO_ANALYZE}")

    flash, flash_2 = get_flash_sections_sizes_kb(BIN_TO_ANALYZE)

    report_section(".flash", flash, FLASH_SIZE_KB)
    report_section(".flash2", flash_2, FLASH_2_SIZE_KB)

    sys.exit(EXIT_CODE)
