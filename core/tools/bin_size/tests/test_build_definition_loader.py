from __future__ import annotations

from pathlib import Path

import pytest

from ..src.bin_size.build_definition_loader import BuildDefinitionLoader

HERE = Path(__file__).resolve().parent
CORE_DIR = HERE.parent.parent.parent

FIRMWARE_ELF = CORE_DIR / "build/firmware/firmware.elf"


def test_loader():
    def_loader = BuildDefinitionLoader()
    if not FIRMWARE_ELF.exists():
        pytest.fail(f"{FIRMWARE_ELF} not found")
    def_loader.load(FIRMWARE_ELF)
    assert def_loader.get("unexisting") is None
    assert def_loader.get("nist256p1") == "vendor/trezor-crypto/nist256p1.c:26"
