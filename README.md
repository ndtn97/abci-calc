# abci-calc
simple abci point calculator for normal priority spot/on-demand jobs


## Build
```
cargo build --release
```

## Make alias in .bashrc/.zshrc
```
alias ap={abci-calc Directory}/target/release/abci-calc
```

## How to use
```
ap {instance-name} {days}d {hours}h {minutes}m {seconds}s x{num-jobs}
```

- Glarge 3hours/job x6 jobs
    ```
    ap glarge 3h x2x3
    > Glarge Single: 0d3h0m0s 2.70pts x6 Total: 0d18h0m0s 16.20pts
    ```

- Full 1h20m/job x24 jobs
    ```
    ap full 1h x2x3 x4 20m
    > Full Single: 0d1h20m0s 1.33pts x24 Total: 1d8h0m0s 32.00pts
    ```

- Asmall 1h30m10s x2 jobs
    ```
    ap asmall 1h30m10s x2
    > AGsmall Single: 0d1h30m10s 0.75pts x2 Total: 0d3h0m20s 1.50pts
    ```

- Fuzzy instance name matching
    ```
    ap gl 1h x2x3 x4 20m
    > Glarge Single: 0d1h20m0s 1.20pts x24 Total: 1d8h0m0s 28.80pts
    ap as 1h x2x3
    > AGsmall Single: 0d1h0m0s 0.50pts x1 Total: 0d1h0m0s 0.50pts
    ap glr 3h
    > Glarge Single: 0d3h0m0s 2.70pts x1 Total: 0d3h0m0s 2.70pts
    ```