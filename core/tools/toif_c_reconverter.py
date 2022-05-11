#!/usr/bin/env python3

import click


def process_line(infile, outfile):
    line = infile.readline()
    data = [x.strip().lower() for x in line.split(',')]
    for c in data:
        if len(c) == 4:
            outfile.write(bytes((int(c, 16),)))

@click.command()
@click.argument("infile", type=click.File("rt"))
@click.argument("outfile", type=click.File("wb"))
def toif_convert(infile, outfile):
    """Convert c header to toif file

    \b
    Examples:
      toif_convert.py outfile.h somefile.toif
    """

    if outfile.name.endswith(".toif") or infile.name == "-":

        outfile.write(bytes((0x54,)))
        outfile.write(bytes((0x4f,)))
        outfile.write(bytes((0x49,)))
        outfile.write(bytes((0x67,)))

        infile.readline()
        infile.readline()
        infile.readline()
        infile.readline()
        infile.readline()
        process_line(infile, outfile)
        infile.readline()
        process_line(infile, outfile)
        infile.readline()
        process_line(infile, outfile)
        infile.readline()

    else:
        raise click.ClickException("At least one of the arguments must end with .toif")


if __name__ == "__main__":
    toif_convert()
