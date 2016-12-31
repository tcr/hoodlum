# hoodlum-parser

Contains the LALRPOP parser code to cache its build between hoodlum changes.

`lalrpop` is not installed or required by the build step. Instead, it is
generated manually the update_parser.sh script when the `src/hdl_parser.lalrpop`
file is changed. This creates a `hdl_parser.rs.gz` file which is extracted
by the `build.rs` file. This keeps the archive and git repository size small,
doesn't require the end user to build LALRPOP or use the generation step, and
overall speeds up development.
