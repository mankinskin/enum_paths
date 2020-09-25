#![feature(bool_to_option)]
extern crate enum_paths;
use enum_paths::*;
use std::str::FromStr;

#[derive(Debug, PartialEq, AsPath)]
enum Route {
    Users(UserRoute),
    Tasks(TaskRoute),
}

#[derive(Debug, PartialEq, AsPath)]
enum UserRoute {
    #[name = ""]
    Profile(u32),
    List,
}

#[derive(Debug, PartialEq, AsPath)]
enum TaskRoute {
    Task(TaskInfo),
    #[name = ""]
    List,
}
#[derive(Debug, PartialEq)]
struct TaskInfo {
    id: u32,
}
impl ToString for TaskInfo {
    fn to_string(&self) -> String {
        self.id.to_string()
    }
}
impl FromStr for TaskInfo {
    type Err = <u32 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FromStr::from_str(s)
            .map(|id| Self { id })
    }
}
#[test]
fn as_path() {
    let path = Route::Users(UserRoute::Profile(1));
    let url = "/users/1";
    let serialized = path.as_path();
    assert_eq!(serialized, url);

    let path = Route::Users(UserRoute::List);
    let url = "/users/list";
    let serialized = path.as_path();
    assert_eq!(serialized, url);

    let path = Route::Tasks(TaskRoute::Task(TaskInfo { id: 1 }));
    let url = "/tasks/task/1";
    let serialized = path.as_path();
    assert_eq!(serialized, url);

    let path = Route::Tasks(TaskRoute::List);
    let url = "/tasks";
    let serialized = path.as_path();
    assert_eq!(serialized, url);
}
#[test]
fn parse_path() {
    let url = "/tasks/task/1";
    let path = Route::Tasks(TaskRoute::Task(TaskInfo { id: 1 }));
    let parsed = ParsePath::parse_path(url).unwrap();
    assert_eq!(path, parsed);

    let url = "/tasks";
    let path = Route::Tasks(TaskRoute::List);
    let parsed = ParsePath::parse_path(url).unwrap();
    assert_eq!(path, parsed);

    let url = "/users/1";
    let path = Route::Users(UserRoute::Profile(1));
    let parsed = ParsePath::parse_path(url).unwrap();
    assert_eq!(path, parsed);

    let url = "/users/list";
    let path = Route::Users(UserRoute::List);
    let parsed = ParsePath::parse_path(url).unwrap();
    assert_eq!(path, parsed);
}
