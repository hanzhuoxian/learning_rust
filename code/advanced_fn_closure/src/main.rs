fn return_closure_impl() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn return_closure_implicit() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let my_closure = return_closure_impl();
    let result = my_closure(5);
    println!("The result is: {}", result);

    let my_closure_boxed = return_closure_implicit();
    let result_boxed = my_closure_boxed(5);
    println!("The result from boxed closure is: {}", result_boxed);
}
