#!/bin/bash

# hoodlum out.hdl test.v 2>&1 | grep "Cannot not compile Await statement"
make clean
make 2>&1 | grep "Net not found"
exit $?
