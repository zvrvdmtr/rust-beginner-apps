## Todo list

### About
Simple application which allows you to manipulate with tasks list. 
You can, get, list and delete tasks. Tasks store in to csv file, so there is no concurrency.
Because it is educational project, so I also have avoided to use additional crates for config, errors and cli.

### Covered Rust topics
1. Enums and pattern matching - https://doc.rust-lang.org/book/ch06-00-enums.html
2. Error handling - https://doc.rust-lang.org/book/ch09-00-error-handling.html
3. Generics - https://doc.rust-lang.org/book/ch10-01-syntax.html
4. Traits - https://doc.rust-lang.org/book/ch10-02-traits.html

### How to use
#### Add task
```
cargo run -p todo-list add task="YOUR TASK BODY"
```
#### Get task by id
```
cargo run -p todo-list get id=%YOUR TASK ID%
```
#### List tasks
```
cargo run -p todo-list list
```
#### Delete task
```
cargo run -p todo-list delete id=%YOUR TASK ID%
```