use std::thread;

async fn hello_world() {
    println!("hello, world!");
}

/* struct Client<V, R> {}

impl<V, R> Client<V, R> {
    fn request(&self, value: V) -> R {}
}

struct Server<V, R> {}
impl<V, R> Server<V, R> {
    fn respond<F>(&self, request_handler: F) -> ()
    where
        F: Fn(V) -> R,
    {
    }
} */

/* -> (Client<V, R>, Server<V, R>) */

fn fetch<V, R>() -> (impl FnMut(V) -> R, impl FnMut(&mut dyn FnMut(V) -> R) -> ()) {
    let request = |value: V| -> R {};

    let respond = |request_handler: &mut dyn FnMut(V) -> R| -> () {};

    (request, respond)
}

fn main() {
    let (request, respond) = fetch();

    // Sending thread
    let hello_thread = thread::spawn(move || {
        let response = request("hello?");
    });

    // Responding thread
    let world_thread = thread::spawn(move || {
        respond(&mut |data| match data {
            "hello?" => "hello world!",
            _ => "",
        });
    });
}
