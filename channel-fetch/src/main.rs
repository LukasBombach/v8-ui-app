use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
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
    let (tx1, rx1): (Sender<V>, Receiver<V>) = mpsc::channel();
    let (tx2, rx2): (Sender<R>, Receiver<R>) = mpsc::channel();

    let thread_tx1 = tx1.clone();
    let thread_tx2 = tx2.clone();

    let request = |v: V| -> R {
        tx1.send(v).unwrap();
        rx2.recv().unwrap()
    };

    let respond = |request_handler: &mut dyn FnMut(V) -> R| -> () {
        let v = rx1.recv().unwrap();
        let r = request_handler(v);
        tx2.send(r).unwrap();
    };

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
