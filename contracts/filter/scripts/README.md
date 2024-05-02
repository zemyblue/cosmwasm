# contract optimize
`./optimizer.sh` is a script to compile optimized `filter` contract. This script should be run in `./contracts/filter` directory, not this `./contracts/filter/scripts` directory. 

```sh
source ./scripts/optizer.sh
```

If the compile is success, the wasm file is outputed in the `./artifacts` directory of root this project.

# filter test with node
If you want to test filter contract, please locate a terminal in this scripts directory, and then run below commands.

```sh
yarn install
node test.js
```
