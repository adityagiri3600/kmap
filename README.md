# kmap
A command line tool to process k-maps 🧩

## Usage
```
kmap <NUMBER_OF_VARIABLES> <MINTERMS>...
```

### Arguments
```
  <NUMBER_OF_VARIABLES>  Number of variables in the k-map
  <MINTERMS>...          List of minterms
```

### Options
```
  -h, --help  Print help
```

### Example
```
$ kmap 4 0 1 2 3 4 5 6 7 8 9 10
F = A' + B'C' + B'D'
```
