# Egraph Mapping via Equality Saturation

Usage:
  python infix_to_prefix.py -f 7nm.genlib (Prepare the dynamic rules based the provided library)
  cargo run

Before running script:

1. Before runnig the project, several modification is made to egg project, specifically, language.rs to provides additional several struct methods. The language.rs file is located in "$CARGO_HOME/registry/src/index..../egg-0.9.5/src/language.rs". Replace the provided language.rs to the default language.rs

2. Install the coincbc MILP solver:
  wget https://raw.githubusercontent.com/coin-or/coinbrew/master/coinbrew
  chmod u+x coinbrew
  ./coinbrew fetch Cbc@2.10.5
  ./coinbrew build Cbc

3. Update the PATH:
  export PATH=$PATH:/export/jyin629/packages/coinbrew/dist/bin
  export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/export/jyin629/packages/coinbrew/dist/lib
  export RUSTFLAGS='-L /export/jyin629/packages/coinbrew/dist/lib'
