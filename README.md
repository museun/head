# head

`head(1)` but works on windows

## usage
```
usage:
    head [flags] [FILE]..

flags:
    -c, --bytes NUM  number of bytes to read
    -n, --lines NUM  number of lines to read
    -q, --quiet      if multiple files are provided disable the header
    -h, --help       print this message

    if no flags were provided, 10 lines will be read    
    if 'FILE' is - then stdin will be read

```

## installation
```
cargo install https://github.com/museun/head
````

*or*

```
git clone https://github.com/museun/head 
cd head 
cargo install -f --path .
```

---

License: 0BSD
