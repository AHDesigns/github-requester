# simple pr counter

Uses Github's graphql api to get all PR's, open and closed, and builds up a hash of prs per author

## new to rust?
See the [docs](https://www.rust-lang.org/tools/install) - it's super easy to install and extremely well loved by those that use it (though it does have a bit of a learning curve!).

## setup
.env:
```sh
GH_SKY_ACCESS_TOKEN: <some token with read access to sky-uk repos>
REPO_NAME: <name of repo you wish to study>
```

## running
```sh
cargo run
```
