#!/usr/bin/env python3

import click


@click.command()
@click.argument("infile", type=click.File("rb"))
@click.argument("outfile", type=click.File("wt"))
def toif_convert(infile, outfile):
    """Convert toif file to c header

    \b
    Examples:
      toif_convert.py somefile.toif outfile.h
    """

    if infile.name.endswith(".toif") or infile.name == "-":
        name = "icon"
        outfile.write("// clang-format off\n")
        outfile.write(f'static const uint8_t toi_{name}[] = {{\n',)
        outfile.write("    // magic\n",)
        outfile.write("    'T', 'O', 'I', 'g',\n",)
        infile.read(4)

        outfile.write("    // width (16-bit), height (16-bit)\n",)
        outfile.write("    ")
        for _ in range(4):
            hex_data = infile.read(1).hex()
            outfile.write(f'0x{hex_data}, ')
        outfile.write("\n")

        outfile.write("    // compressed data length (32-bit)\n",)
        outfile.write("    ")
        for _ in range(4):
            hex_data = infile.read(1).hex()
            outfile.write(f'0x{hex_data}, ')
        outfile.write("\n")

        outfile.write("    // compressed data\n",)
        outfile.write("    ")
        hex_data = infile.read(1).hex()
        while hex_data:
            outfile.write(f'0x{hex_data}, ')
            hex_data = infile.read(1).hex()
        outfile.write("\n};\n")

        byte = infile.read(1)

    else:
        raise click.ClickException("At least one of the arguments must end with .toif")


if __name__ == "__main__":
    toif_convert()
