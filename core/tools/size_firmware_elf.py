#!/usr/bin/env python3
"""
Getting information the size of symbols stored in firmware binary.
Using `build/firmware/firmware.elf` by default.

Example usage:
>>> size.py get -f /path/to/firmware.elf -o result.txt
- custom binary input and output to a file
>>> size.py get --apps-statistics
- shows overall size of each application
>>> size.py get --language Rust
- shows just Rust symbols
>>> size.py get --app ethereum --add-definitions
- shows each function for ethereum app, including line numbers of definitions
>>> size.py --module-name src/apps/bitcoin/sign_tx/bitcoin.py
- shows all functions from src/apps/bitcoin/sign_tx/bitcoin.py file
>>> size.py --module-name '*__init__.py'
- shows all functions from all __init__.py files
>>> size.py get --func-name step2_approve_outputs --add-definitions
- shows all the rows with this function, with a definition
>>> size.py get --func-name Bitcoin.step2_approve_outputs
- shows the function on an object, with a definition
>>> size.py get --no-aggregation --no-sort --no-processing
- suppressing some actions

>>> size.py compare /path/to/firmware.elf_version25 /path/to/firmware.elf_version1 -D -o diff.txt
- compares size differences in flash sections of these two binaries
- with all details, saved to file

>>> size.py build version25
- builds the binary and copies it as `build/firmware/firmware.elf_version25`
"""

from __future__ import annotations

import re
import shutil
import subprocess
from fnmatch import fnmatch
from pathlib import Path

import click

from bin_size import BinarySize, DataRow, StatisticsPlugin, show_binaries_diff

HERE = Path(__file__).parent
CORE_DIR = HERE.parent

BUILD_CMD = f"(cd {CORE_DIR} && make build_firmware)"
FIRMWARE_ELF = CORE_DIR / "build/firmware/firmware.elf"


def _build_binary(extra_suffix: str | None = None) -> None:
    print(f"building the binary... `{BUILD_CMD}`")
    build_result = subprocess.run(
        BUILD_CMD, stdout=subprocess.PIPE, text=True, shell=True
    )
    if build_result.returncode != 0:
        print("build failed - see output above")
        exit(1)

    # Optionally copying the binary to a special name, to be used later
    if extra_suffix is not None:
        new_path = Path(f"{FIRMWARE_ELF}_{extra_suffix}")
        shutil.copyfile(FIRMWARE_ELF, new_path)
        print(f"Binary copied as `{new_path}`")


def _handle_apps_statistics(BS: BinarySize, output_file: str) -> None:
    def apps_categories(row: DataRow) -> str | None:
        pattern = r"^src/apps/(\w+)/"  # dir name after apps/
        match = re.search(pattern, row.module_name)
        if not match:
            return None
        else:
            return match.group(1)

    StatisticsPlugin(BS, apps_categories).show(output_file or None)


#
# CLI PART
#


@click.group()
def cli() -> None:
    pass


@cli.command()
@click.option(
    "-f", "--custom-file", help="Analyze custom file instead of the default one"
)
@click.option("-o", "--output-file", help="Dump results to file instead of stdout")
@click.option("-a", "--app", help="App which to analyze - e.g. `ethereum`")
@click.option(
    "-l",
    "--language",
    type=click.Choice(("mpy", "Rust", "C")),
    help="Language choice - e.g. `Rust`",
)
@click.option(
    "-g",
    "--grep",
    help="Custom string to filter the row with, case-insensitive - e.g. `bitcoin`",
)
@click.option(
    "-M",
    "--module-name",
    "--mn",
    help="Check only specific file/module. Supports shell-style wildcards - e.g. `*/networks.py`",
)
@click.option(
    "-F",
    "--func-name",
    "--fn",
    help="Check only specific function - e.g. `get_tx_keys`",
)
@click.option("-b", "--build", is_flag=True, help="Perform build")
@click.option(
    "-A",
    "--all-sections",
    "--as",
    is_flag=True,
    help="Get all sections, not just flash",
)
@click.option(
    "-s", "--apps-statistics", is_flag=True, help="Overall statistics for all apps"
)
@click.option(
    "-D",
    "--add-definitions",
    "--ad",
    is_flag=True,
    help="Get line definitions for all functions",
)
@click.option(
    "-G",
    "--no-aggregation",
    "--na",
    is_flag=True,
    help="Do not aggregate symbols together",
)
@click.option("-S", "--no-sort", "--ns", is_flag=True, help="Do not sort by size")
@click.option("-P", "--no-processing", "--np", is_flag=True, help="See just raw data")
def get(
    custom_file: str,
    output_file: str,
    app: str,
    language: str,
    grep: str,
    module_name: str,
    func_name: str,
    build: bool,
    all_sections: bool,
    apps_statistics: bool,
    add_definitions: bool,
    no_processing: bool,
    no_sort: bool,
    no_aggregation: bool,
) -> None:
    """Analyze a single binary, `build/firmware/firmware.elf` by default"""

    if build:
        _build_binary()

    BS = BinarySize()

    file_to_analyze = custom_file or FIRMWARE_ELF
    sections = None if all_sections else (".flash", ".flash2")

    BS.load_file(file_to_analyze, sections=sections)

    if no_processing:
        BS.show()
        return

    BS.add_basic_info()

    if apps_statistics:
        return _handle_apps_statistics(BS, output_file)

    if not no_aggregation:
        BS.aggregate()
    if not no_sort:
        BS.sort(lambda row: row.size, reverse=True)

    if app:
        BS.filter(lambda row: row.module_name.startswith(f"src/apps/{app}/"))
    if language:
        BS.filter(lambda row: row.language == language)
    if module_name:
        BS.filter(lambda row: fnmatch(row.module_name, module_name))
    if func_name:
        # There could be an object or not ... Bitcoin.sign_tx vs sign_tx
        # If not, we need to account for the possible object in row.func_name
        if "." in func_name:
            BS.filter(lambda row: row.func_name.rstrip("()") == func_name)
        else:
            BS.filter(
                lambda row: row.func_name.rstrip("()").split(".")[-1] == func_name
            )
    if grep:
        BS.filter(lambda row: grep.lower() in str(row).lower())

    if add_definitions:
        BS.add_definitions()

    if output_file:
        BS.show(output_file)
    else:
        BS.show()


@cli.command()
@click.argument("bin1")
@click.argument("bin2")
@click.option("-o", "--output-file", help="Dump results to file instead of stdout")
@click.option(
    "-a", "--all-sections", is_flag=True, help="Compare all sections, not just flash"
)
@click.option(
    "-d", "--details", is_flag=True, help="Include all details (line definitions)"
)
def compare(
    bin1: str,
    bin2: str,
    output_file: str,
    all_sections: bool,
    details: bool,
) -> None:
    """Compare two binaries"""
    sections = None if all_sections else (".flash", ".flash2")
    file_to_save = output_file or None
    show_binaries_diff(
        bin1, bin2, all_details=details, sections=sections, file_to_save=file_to_save
    )


@cli.command()
@click.argument("name", required=False)
def build(
    name: str,
) -> None:
    """Build a binary and optionally give it a new name"""
    _build_binary(name)


if "__main__" == __name__:
    cli()
