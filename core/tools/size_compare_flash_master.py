#!/usr/bin/env python3
"""
Compares the current firmware build with a master one
and prints the differences.

Fails if the current changes are increasing the size by a lot.

Also generates a thorough report of the current state of the binary
with all the functions and their definitions.
"""

from __future__ import annotations

import atexit
import shutil
import sys
from io import BytesIO
from pathlib import Path
from zipfile import ZipFile

import requests

from bin_size import BinarySize, get_flash_sections_sizes_kb, show_binaries_diff

HERE = Path(__file__).parent
CORE_DIR = HERE.parent

if len(sys.argv) > 1:
    CURR_BIN_TO_ANALYZE = sys.argv[1]
else:
    CURR_BIN_TO_ANALYZE = CORE_DIR / "build/firmware/firmware.elf"  # type: ignore

REPORT_FILE = HERE / "size_report_firmware_elf.txt"

MAX_KB_ADDITION_TO_SUCCEED = 5

EXIT_CODE = 0


def get_master_bin() -> str | Path:
    """Locates the master firmware binary."""
    # TODO: this is just temporary, as in current master there is no way to get
    # firmware.elf from the CI.
    # So for the time being, using the one we have copied in the repository.
    # WARNING: do not forget to delete it!
    # Also, this CI job should run only for non-master branches
    # (or maybe not, it could also verify the master build did not change somehow)
    master_path = CORE_DIR / "tmp_master_core_v2.5.1_nondebug_firmware.elf"
    # master_path = download_and_get_latest_master_firmware_elf()
    return master_path


def download_and_get_latest_master_firmware_elf() -> Path:
    url = "https://gitlab.com/satoshilabs/trezor/trezor-firmware/-/jobs/artifacts/master/download?job=core%20fw%20regular%20build"
    req = requests.get(url)
    tmp_dir = HERE / "tmp_for_master_elf"
    zip_file = ZipFile(BytesIO(req.content))
    zip_file.extractall(tmp_dir)

    atexit.register(lambda: shutil.rmtree(tmp_dir))

    return tmp_dir / "firmware.elf"


def generate_report_file() -> None:
    # TODO: consider if we want to add definitons
    # It can take up to 30 minutes, when it is not cached
    # (can the cache be stored on CI servers?)
    BinarySize().load_file(
        CURR_BIN_TO_ANALYZE, sections=(".flash", ".flash2")
    ).add_basic_info().aggregate().sort(lambda row: row.size, reverse=True).show(
        REPORT_FILE
    )


if __name__ == "__main__":
    print(f"Analyzing {CURR_BIN_TO_ANALYZE}")

    curr_flash, curr_flash_2 = get_flash_sections_sizes_kb(CURR_BIN_TO_ANALYZE)

    master_bin = get_master_bin()
    master_flash, master_flash_2 = get_flash_sections_sizes_kb(master_bin)

    show_binaries_diff(
        old=master_bin, new=CURR_BIN_TO_ANALYZE, sections=(".flash", ".flash2")
    )

    print()
    print(f"Current: flash={curr_flash}K flash2={curr_flash_2}K")
    print(f"Master:  flash={master_flash}K flash2={master_flash_2}K")

    size_diff = (curr_flash + curr_flash_2) - (master_flash + master_flash_2)
    print(f"Size_diff: {size_diff} K")
    if size_diff > MAX_KB_ADDITION_TO_SUCCEED:
        print(f"Size of flash sections increased by {size_diff} K.")
        print(f"More than allowed {MAX_KB_ADDITION_TO_SUCCEED} K. Failing.")
        EXIT_CODE = 1  # type: ignore

    # TODO: maybe make this a separate script?
    print()
    print(f"Generating report file under {REPORT_FILE}")
    generate_report_file()

    sys.exit(EXIT_CODE)
