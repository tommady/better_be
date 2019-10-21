# better_be
for formating a better string 


### usage
```
better-be 0.1.2
format the words you want

USAGE:
    better_be [FLAGS] [OPTIONS] <input-words>

FLAGS:
    -h, --help         Prints help information
    -l, --lowercase
    -V, --version      Prints version information

OPTIONS:
    -f, --from <words-from>     [default:  ]
    -t, --to <words-to>         [default: \ ]

ARGS:
    <input-words>
```

### example1
```
./better_be -l -t "-" "As a subscriber, I want to process comments and insert into ES"
as-a-subscriber--i-want-to-process-comments-and-insert-into-es
```

### example2
```
./better_be "As a subscriber, I want to process comments and insert into ES"
As\ a\ subscriber,\ I\ want\ to\ process\ comments\ and\ insert\ into\ ES
```

### example3
```
./better_be -s "As a subscriber, I want to process comments and insert into ES"
As\ a\ subscriber\ I\ want\ to\ process\ comments\ and\ insert\ into\ ES
```

