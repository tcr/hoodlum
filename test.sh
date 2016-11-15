#!/bin/bash

cat src/header.rs src/lib2.rs src/footer.rs > src/lib.rs
exec cargo test
