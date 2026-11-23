# SPDX-License-Identifier: Apache-2.0
"""Generate Rust parsers using the antlr4rust fork of ANTLR."""

import hashlib
from pathlib import Path
import subprocess
from urllib.request import urlretrieve


GENERATOR_URL = "https://github.com/antlr4rust/antlr4/releases/download/v0.6.0/antlr4-4.13.3-SNAPSHOT-complete.jar"
GENERATOR_SHA256 = "391e77311794f7c8fed2bc6d291f54b04725e37456d79e1981edfd3fc919b677"


def main():
    grammar_directory = Path(__file__).resolve().parent
    output_directory = grammar_directory.parent / "gen/antlr/rust"
    output_directory.mkdir(parents=True, exist_ok=True)
    generator = output_directory.parent / "antlr4-rust-0.6.0.jar"

    if not generator.exists():
        urlretrieve(GENERATOR_URL, generator)
    if hashlib.sha256(generator.read_bytes()).hexdigest() != GENERATOR_SHA256:
        raise SystemExit(
            f"ANTLR Rust generator checksum mismatch. Delete {generator} and retry."
        )

    for source in output_directory.glob("*.rs"):
        source.unlink()

    subprocess.run(
        [
            "java",
            "-jar",
            str(generator),
            "-Dlanguage=Rust",
            "-o",
            str(output_directory),
            "SubstraitLexer.g4",
            "SubstraitType.g4",
            "FuncTestCaseLexer.g4",
            "FuncTestCaseParser.g4",
        ],
        cwd=grammar_directory,
        check=True,
    )

    # The Rust fork doubles "Parser" in this context trait's name.
    parser = output_directory / "functestcaseparser.rs"
    parser.write_text(
        parser.read_text().replace(
            "FuncTestCaseParserParserContext", "FuncTestCaseParserContext"
        )
    )

    modules = "".join(
        f"pub mod {source.stem};\n" for source in sorted(output_directory.glob("*.rs"))
    )
    (output_directory / "lib.rs").write_text("#![allow(unused_parens)]\n" + modules)


if __name__ == "__main__":
    main()
