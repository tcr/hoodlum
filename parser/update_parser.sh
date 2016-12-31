#!/bin/bash

set -e

cd $(dirname $0)
cd src

rm hdl_parser.rs 2>/dev/null || true
rm hdl_parser.rs.gz 2>/dev/null || true

lalrpop hdl_parser.lalrpop
chmod 0644 hdl_parser.rs

gzip hdl_parser.rs
