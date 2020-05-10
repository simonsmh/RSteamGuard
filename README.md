# RSteamGuard
Get Steam guard code without the Mobile Authenticator app, now with rust.

## Usage

You need to get your **shared secret**. Refer to [this article](https://github.com/SteamTimeIdler/stidler/wiki/Getting-your-%27shared_secret%27-code-for-use-with-Auto-Restarter-on-Mobile-Authentication).

Then add it to the first line of [`main.rs`](src/main.rs) or just compile this project directly with cargo.

```
cargo build --release
```

Install it to some place you like and run with

```
./steam_guard [secret]
```
or
```
SECRET=<secret> ./steam_guard
```

## Original Project
- [PySteamGuard](https://github.com/JeziL/PySteamGuard)
