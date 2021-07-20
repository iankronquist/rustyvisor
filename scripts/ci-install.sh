#!/bin/bash

set -x
set -e

source ~/.cargo/env

sudo apt-get install linux-headers-$(uname -r)
