extern crate enum_paths;
use enum_paths::*;
use std::str::FromStr;

#[derive(Debug, PartialEq, AsPath, Named)]
enum Route {
    Users(UserRoute),
    #[segment_as = "stuff"]
    Tasks(TaskRoute),
    #[segment_as = ""]
    Empty,
}

#[derive(Debug, PartialEq, AsPath, Named)]
enum UserRoute {
    #[segment_as = ""]
    Profile(u32),
    List,
}

#[derive(Debug, PartialEq, AsPath, Named)]
enum TaskRoute {
    Task(TaskInfo),
    #[segment_as = ""]
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
        FromStr::from_str(s).map(|id| Self { id })
    }
}
#[test]
fn as_path() {
    assert_eq!(Route::Users(UserRoute::Profile(1)).as_path(), "/users/1");
    assert_eq!(Route::Users(UserRoute::List).as_path(), "/users/list");
    assert_eq!(
        Route::Tasks(TaskRoute::Task(TaskInfo { id: 1 })).as_path(),
        "/stuff/task/1"
    );
    assert_eq!(Route::Tasks(TaskRoute::List).as_path(), "/stuff");
    assert_eq!(Route::Empty.as_path(), "");
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
        Route::parse_path("/stuff/task/1").unwrap(),
        Route::Tasks(TaskRoute::Task(TaskInfo { id: 1 })),
    );
    assert_eq!(
        Route::parse_path("/stuff///task///2").unwrap(),
        Route::Tasks(TaskRoute::Task(TaskInfo { id: 2 })),
    );
    assert_eq!(
        Route::parse_path("/stuff").unwrap(),
        Route::Tasks(TaskRoute::List),
    );
    assert_eq!(Route::parse_path("/").unwrap(), Route::Empty,);
    assert_eq!(Route::parse_path("///").unwrap(), Route::Empty,);
    assert_eq!(Route::parse_path("").unwrap(), Route::Empty,);
}


#[test]
fn name() {
    assert_eq!(Route::Empty.get_name(),"Empty");
    assert_eq!(Route::Tasks(TaskRoute::Task(TaskInfo { id: 1 })).get_name(),"Tasks");
    assert_eq!(TaskRoute::Task(TaskInfo { id: 1 }).get_name(),"Task");
}

