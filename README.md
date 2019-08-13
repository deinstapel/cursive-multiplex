<p align="center">
  <img src="assets/cursive-multiplex.svg" height="128">
</p>
<h1 align="center">Welcome to cursive-multiplex 👋</h1>
<p align="center">
  <a href="https://travis-ci.org/deinstapel/cursive-multiplex">
    <img src="https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fdeinstapel.github.io%2Fcursive-multiplex%2Fstable-build.json" alt="stable build">
  </a>
  <a href="https://travis-ci.org/deinstapel/cursive-multiplex">
    <img src="https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fdeinstapel.github.io%2Fcursive-multiplex%2Fnightly-build.json" alt="nightly build">
  </a>
  <a href="https://travis-ci.org/deinstapel/cursive-multiplex">
    <img src="https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fdeinstapel.github.io%2Fcursive-multiplex%2Fcargo-test.json" alt="cargo test">
  </a>
  <a href="https://github.com/fin-ger/shellshot">
    <img src="https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fdeinstapel.github.io%2Fcursive-multiplex%2Fshellshot.json" alt="shellshot">
  </a>
  <a href="https://crates.io/crates/cursive-multiplex">
    <img alt="crates.io" src="https://img.shields.io/crates/v/cursive-multiplex.svg">
  </a>
  <a href="https://docs.rs/cursive-multiplex">
    <img alt="Docs.rs" src="https://docs.rs/cursive-multiplex/badge.svg">
  </a>
  <a href="https://github.com/deinstapel/cursive-multiplex/blob/master/LICENSE">
    <img alt="GitHub" src="https://img.shields.io/github/license/deinstapel/cursive-multiplex.svg">
  </a>
  <a href="http://makeapullrequest.com">
    <img alt="PRs Welcome" src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" target="_blank" />
  </a>
  <br>
  <i>A tmux like multiplexer for gyscos/cursive views</i>
</p>

---

> This project is work-in-progress

## How does it look like?

<details>
  <summary>Expand to view</summary>
  <img src="assets/demo.gif" alt="Demo GIF">
</details>

## Usage

Simply add to your `Cargo.toml`
```Cargo
[dependencies]
cursive-multiplex = "^0.1.2
```

And then use the `MuxBuilder` to build a new Mux View
```rust
let builder = cursive_multiplex::MuxBuilder::new();
let (mut mux, root_node)_= builder.build(cursive::views::TextView::new("Hello World!".to_string()));
```

> With the MuxBuilder defaults are automatically set for controls of course you can still change them, have a look at the [docs](https://docs.rs/cursive-multiplex).

###  Adding views

You can add views by giving a path or an id to an existing node e.g.

```rust
let new_node = mux.add_horizontal_id(cursive::views::TextView::new("Foo"), node1).unwrap();
```

Its also possible to add views by their path.
```rust
let new_node = mux.add_horizontal_path(cursive::views::TextView::new("Foo", Path::LeftOrUp(Box::new(None))));
```

Returned will be a Result Ok contains the new id assigned to the view, or an error in case of failure.

### Removing Views

You can also remove views, by giving the id of the views.

```rust
mux.remove_id(new_node)?;
```

On success the id of the removed node is returned.

### Switch Views

If you want to reorder your views you can easily switch them by using

```rust
mux.switch_views(new_node, old_node)?;
```


## Add to your project

Add the crate to your `Cargo.toml` under `dependencies`

```Cargo
[dependencies]
cursive-multiplex = "0.1.0"
```

## Troubleshooting

If you find any bugs/unexpected behaviour or you have a proposition for future changes open an issue describing the current behaviour and what you expected.

## Development

> TBD

### Running the tests

#### Preparing integration tests

In order to run the integration tests, you first need to install a recent version of `npm`!

After `npm` is installed, install required dependencies:

```
$ ./scripts/prepare-end2end-tests.sh
```

This will use `npm` to install `jest` and `shellshot` in the `tests` folder.

#### Running all test suites

Just run

```
$ cargo test
```

to execute all available tests.

#### shields.io endpoints

[shields.io](https://shields.io) endpoints are generated inside the `./target/shields` folder. They are used in this README.

## Authors

**Fin Christensen**

> [:octocat: `@fin-ger`](https://github.com/fin-ger)  
> [:elephant: `@fin_ger@mastodon.social`](https://mastodon.social/web/accounts/787945)  
> [:bird: `@fin_ger_github`](https://twitter.com/fin_ger_github)  

<br>

**Johannes Wünsche**

> [:octocat: `@jwuensche`](https://github.com/jwuensche)  
> [:elephant: `@fredowald@mastodon.social`](https://mastodon.social/web/accounts/843376)  
> [:bird: `@Fredowald`](https://twitter.com/fredowald)  

## Show your support

Give a :star: if this project helped you!
