from __future__ import annotations

import re
import textwrap
import typing as t
from dataclasses import dataclass, field

import cxxfilt
import click

REPLACEMENTS = {
    "$LT$": "<",
    "$GT$": ">",
    "$u20$": " ",
    "$RF$": "&",
    "..": "::",
}

LINKER_SCRIPT_START = "Linker script and memory map"

DEFAULT_SECTIONS = (".flash", ".data")


def rust_demangle(symbol: str) -> str:
    match = re.match(r"^(\..*?)(_Z.*)$", symbol)
    if not match:
        return symbol
    prefix, mangled = match.groups()
    demangled = cxxfilt.demangle(mangled)
    for k, v in REPLACEMENTS.items():
        demangled = demangled.replace(k, v)
    last_segment = demangled.rindex("::")
    return prefix + "#Rust#" + demangled[:last_segment]
    #symbol = re.sub(r"17h.*E$", "", demangled)
    return prefix + demangled


@dataclass
class Section:
    name: str
    entries: list[Entry] = field(default_factory=list)

    def total_size(self) -> int:
        return sum(e.size for e in self.entries)


@dataclass
class Entry:
    section: Section
    address: int
    size: int
    comment: str


def is_section_start(elems: list[str]) -> Section | None:
    if not elems:  # empty line
        return None
    if elems[0].startswith("0x"):  # address entry
        return None
    name = rust_demangle(elems[0])
    return Section(name)


def make_entry(current_section: Section, elems: list[str]) -> Entry:
    address_hex, size_hex, *comment_parts = elems
    address = int(address_hex, 16)
    comment = " ".join(comment_parts)
    if size_hex.startswith("0x"):
        size = int(size_hex, 16)
    else:
        comment = f"{size_hex} {comment}"
        size = 0
    return Entry(current_section, address, size, comment)


def find_sections(lines: t.Iterable[str]) -> dict[str, list[Section]]:
    sections_dict: dict[str, list[Section]] = {}
    lines = iter(lines)

    # skip until linker script start
    for line in lines:
        if line.strip() == LINKER_SCRIPT_START:
            break

    current_supersection: list[Section] = []
    current_section: Section | None = None
    for line in lines:
        elems = line.strip().split()
        if line.startswith("."):
            # super-section starts with its name, mini-sections are indented
            current_supersection = []
            sections_dict[elems[0]] = current_supersection
        elif line.startswith(" "):
            section = is_section_start(elems)
            if section is not None:
                current_supersection.append(section)
                current_section = section
                elems.pop(0)
            if elems and current_section is not None:
                current_section.entries.append(make_entry(current_section, elems))

    return sections_dict


def build_tree(sections: list[Section]) -> dict:
    section_tree = {}
    # build per-char tree
    for section in sections:
        current_subtree = section_tree
        for char in section.name:
            current_subtree = current_subtree.setdefault(char, {})
        current_subtree[""] = section
    return section_tree


# prune tree so that no single-entry subtree exists
def prune_subtree(tree: dict) -> dict:
    new_tree = {}
    for entry, subtree in tree.items():
        while isinstance(subtree, dict) and len(subtree) == 1:
            k, subtree = subtree.popitem()
            entry += k
        if isinstance(subtree, dict):
            subtree = prune_subtree(subtree)
        new_tree[entry] = subtree
    return new_tree


def total_subtree_size(tree: dict) -> int:
    total = 0
    for subtree in tree.values():
        if isinstance(subtree, dict):
            total += total_subtree_size(subtree)
        else:
            total += subtree.total_size()
    return total


def subtree_sizes(tree: dict, name_prefix: str = "") -> str:
    lines = []
    for name, subtree in tree.items():
        if isinstance(subtree, dict):
            subtree_name = name_prefix + name
            size = total_subtree_size(subtree)
            lines.append(f"{subtree_name}: {size}")
            if size > 0:
                subsizes = subtree_sizes(subtree, subtree_name)
                lines.append(textwrap.indent(subsizes, "    "))
        else:
            lines.append(f"*{subtree.name}: {subtree.total_size()}")
    return "\n".join(lines)


@click.command()
@click.argument("input", type=click.File("r"))
@click.option("-s", "--section", "sections", multiple=True, default=DEFAULT_SECTIONS)
def main(input, sections):
    lines = input.read().splitlines()
    parsed_sections = find_sections(lines)
    interesting_sections = sum((parsed_sections[section] for section in sections), [])

    tree = build_tree(interesting_sections)
    tree = prune_subtree(tree)
    print(subtree_sizes(tree))


if __name__ == "__main__":
    main()
