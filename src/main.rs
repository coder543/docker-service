#[macro_use]
extern crate rouille;

fn main() {
    println!("Now listening on localhost:8000");

    rouille::start_server("0.0.0.0:8000", move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response::redirect_302("/hello/world")
            },

            (GET) (/hello/world) => {
                println!("hello world");
                rouille::Response::text("hello world")
            },

            _ => rouille::Response::empty_404()
        )
    });
}
