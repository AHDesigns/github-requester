# simple pr counter

Uses Github's graphql api to get all PR's, open and closed, and builds up a hash of prs per author.

Has to get paginated results, so let it run for a short while.

Each pr's info is logged out, and then at the end, there will be a tidy hash of "users: number of prs".

## new to rust?
See the [docs](https://www.rust-lang.org/tools/install) - it's super easy to install and extremely well loved by those that use it (though it does have a bit of a learning curve!).

## setup
.env:
```sh
ACCESS_TOKEN: <github access token with read access to org>
REPO_NAME: <name of repo you wish to study>
```

## running
```sh
cargo run
```
