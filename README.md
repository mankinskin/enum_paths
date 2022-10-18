# enum_paths

Define "/" separated paths with nested enums

```rust
#[derive(Debug, PartialEq, AsPath)]
enum Route {
    Users(UserRoute),
}

#[derive(Debug, PartialEq, AsPath)]
enum UserRoute {
    #[name = ""]
    Profile(u32),
    List,
}
```
And convert between them:
```rust
#[test]
fn as_path() {
    assert_eq!(
        Route::Users(UserRoute::Profile(1)).as_path(),
        "/users/1"
    );
    assert_eq!(
        Route::Users(UserRoute::List).as_path(),
        "/users/list"
    );
}
#[test]
fn parse_path() {
    assert_eq!(
        ParsePath::parse_path("/users/1").unwrap(),
        Route::Users(UserRoute::Profile(1))
    );
    assert_eq!(
        ParsePath::parse_path("/users/list").unwrap(),
        Route::Users(UserRoute::List)
    );
}
```
# How to use it
Derive [`AsPath`](src/lib.rs#L6) and [`ParsePath`](src/lib.rs#L15) using the derive macro.
Override names using attributes (the default is to convert variant names to snake_case):
```rust
#[derive(AsPath)]
enum Route {
    #[name = "new_name"]
    OldName, // would have been "old_name"
    
    #[name = "prefix"]
    Nested(u32), // turns out as "/prefix/{u32}" instead of "/nested/{u32}"
    
    #[name = ""]
    MustBeAtTheEnd, // a (single) unit variant with no name must be declared last
}
```
You're good to go!
```rust
Route::OldName.as_path(); // "/new_name"

match Route::parse_path("/") {
    Ok(route) => log!(route),   // Route::MustBeAtTheEnd
    Err(err) => error!(err),
}
```
