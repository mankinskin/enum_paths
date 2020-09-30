extern crate enum_paths;
use enum_paths::*;
use std::str::FromStr;

#[derive(Debug, PartialEq, AsPath)]
enum Route {
    Users(UserRoute),
    Tasks(TaskRoute),
    #[name = ""]
    Empty,
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
    assert_eq!(
        Route::Users(UserRoute::Profile(1)).as_path(),
        "/users/1"
    );
    assert_eq!(
        Route::Users(UserRoute::List).as_path(),
        "/users/list"
    );
    assert_eq!(
        Route::Tasks(TaskRoute::Task(TaskInfo { id: 1 })).as_path(),
        "/tasks/task/1"
    );
    assert_eq!(
        Route::Tasks(TaskRoute::List).as_path(),
        "/tasks"
    );
    assert_eq!(
        Route::Empty.as_path(),
        ""
    );
}
#[test]
fn parse_path() {
    assert_eq!(
        Route::parse_path("/users/1").unwrap(),
        Route::Users(UserRoute::Profile(1)),
    );
    assert_eq!(
        Route::parse_path("users/1").unwrap(),
        Route::Users(UserRoute::Profile(1)),
    );
    assert_eq!(
        Route::parse_path("/users/list").unwrap(),
        Route::Users(UserRoute::List),
    );
    assert_eq!(
        Route::parse_path("/tasks/task/1").unwrap(),
        Route::Tasks(TaskRoute::Task(TaskInfo { id: 1 })),
    );
    assert_eq!(
        Route::parse_path("/tasks///task///2").unwrap(),
        Route::Tasks(TaskRoute::Task(TaskInfo { id: 2 })),
    );
    assert_eq!(
        Route::parse_path("/tasks").unwrap(),
        Route::Tasks(TaskRoute::List),
    );
    assert_eq!(
        Route::parse_path("/").unwrap(),
        Route::Empty,
    );
    assert_eq!(
        Route::parse_path("///").unwrap(),
        Route::Empty,
    );
    assert_eq!(
        Route::parse_path("").unwrap(),
        Route::Empty,
    );
}
