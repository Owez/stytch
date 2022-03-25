# Stych Rust

Unofficial glue wrapper to use the basic email flow of Stych inside of Rust

## Usage

```rust
use stych::Stych;

// store credentials
let stych = Stych::new(
    "project_id",
    "secret",
    "redirect_for_login",
    "redirect_for_signup"
);

// create new user
let user = stych.login_or_create("root@ogriffiths.com").await?;

// authenticate
let authenticated = stych.auth(user.token).await?;
if authenticated.is_ok() {
    println!("This user is good!");
} else {
    println!("Nope!");
}
```

## Why?

I needed to use Stych as a requirement inside of an internal project, but there wheren't any Rust adapters available. Feel free contribute or message me to take over this project as it's in maintenance mode as it stands!
