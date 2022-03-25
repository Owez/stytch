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
let user = stych.login_or_create("root@ogriffiths.com").await.unwrap();

// authenticate
let authenticated = stych.auth(user.token).await.is_ok();
if authenticated {
    println!("This user is good!");
} else {
    println!("Nope!");
}
```
