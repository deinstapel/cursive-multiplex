# Creating a new GIF

## Recording

Inside a `80x24` terminal record it using

```
$ cargo build --example basic
$ cd assets
$ terminalizer record --config ./config.yml demo
```

## Rendering

```
$ terminalizer render demo.yml -o demo.gif
```

## Optimizing

```
$ gifsicle --colors 24 -O3 demo.gif -o demo.gif
```
