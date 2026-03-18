import sys

with open(sys.argv[1]) as f:
    for line in f:
        # Skip empty lines, comments, and enum declaration lines
        line = line.strip()
        if (
            not line
            or line.startswith("//")
            or line.startswith("pub enum")
            or line == "{"
            or line == "}"
        ):
            continue
        # Remove trailing comma and print
        # "VariantName," -> "VariantName"
        print(line.rstrip(","))
