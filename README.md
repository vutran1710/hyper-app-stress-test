## Stress-testing a *Hyper* app using bombardier

1. Build the executable file
``` shell
cargo build --release
```

2. Run app

3. Stress-test
```shell
> bombardier -c 125 -n 10000000 http://localhost:3000
```

4. Enjoy the result

## Result
Like shit! Hyper is way too much slower than Actix (only 1/3rd of Actix' perf). 
Probably due to the naive implementation.


