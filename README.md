# Stytch Rust

Unofficial library to use the Stych email flow in Rust

## Usage

```rust
use stytch::Stytch;

// store credentials
let stytch = Stytch::new(
    "project_id",
    "secret",
    "redirect_for_login",
    "redirect_for_signup"
);

// create new user
let user = stytch.login_or_create("root@ogriffiths.com").await?;

// authenticate
let authenticated = stytch.auth(user.token).await?;
if authenticated.is_ok() {
    println!("This user is good!");
} else {
    println!("Nope!");
}
```

## Project Status

Feel free contribute or message me to take over this project as it's in maintenance mode as it stands; I'm up for hire!

## Licensing

This project is dual-licensed under both the [MIT](https://github.com/Owez/argi/blob/master/LICENSE-MIT) and [Apache](https://github.com/Owez/argi/blob/master/LICENSE-APACHE), so feel free to use either at your discretion.
