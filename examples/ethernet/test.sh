#!/bin/bash

hoodlum out.hdl test.v 2>&1 | grep "Missing FSM state"

exit $?
