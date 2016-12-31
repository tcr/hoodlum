#!/bin/bash

cd $(dirname $0)
cd src
lalrpop hdl_parser.lalrpop
gzip hdl_parser.rs -f
