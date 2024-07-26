# Egraph Mapping via Equality Saturation

## Project Overview

This project implements Egraph mapping via equality saturation, using dynamic rules and a MILP solver to optimize logic synthesis.

## Usage

### Prepare Dynamic Rules
```
python infix_to_prefix.py -f 7nm.genlib  # Prepare the dynamic rules based on the provided library
```

### Run the Project
```
cargo run
```

## Prerequisites
Before running the project, perform the following setup steps:

### Modify the egg Project
Several modifications are needed in the egg project, specifically in the language.rs file. This file provides additional struct methods and is located in:
```
$CARGO_HOME/registry/src/index.../egg-0.9.5/src/language.rs
```
Replace the default language.rs file with the provided one.

### Install the CoinCBC MILP Solver

To install the CoinCBC MILP solver, follow these steps:
```
wget https://raw.githubusercontent.com/coin-or/coinbrew/master/coinbrew
chmod u+x coinbrew
./coinbrew fetch Cbc@2.10.5
./coinbrew build Cbc
```
Note: It has to be version 2.10.5, as the egg project cannot read the file library with the latest master version.

### Update Environment Variables
Ensure the CoinCBC binaries and libraries are included in your system's PATH and library path:
```
export PATH=$PATH:/coinbrew_install_path/coinbrew/dist/bin
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/coinbrew_install_path/coinbrew/dist/lib
export RUSTFLAGS='-L /coinbrew_install_path/coinbrew/dist/lib
```

