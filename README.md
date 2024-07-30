# kmap
A command line tool to process k-maps ***BLAZINGLY FAST*** ⚡

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

### Examples
```
$ kmap 4 0 1 2 3 4 5 6 7 8 9 10

F = A' + B'C' + B'D'

Total 81 groups, found solution out of 10^24.38 possible
Took 111.402µs

$ kmap 6 0 1 2 53

F = A'B'C'D'E' + A'B'C'D'F' + ABC'DE'F

Total 729 groups, found solution out of 10^219.45 possible
Took 325.738µs
```
![Time Analysis](https://github.com/adityagiri3600/kmap/blob/master/time%20analysis.png)
