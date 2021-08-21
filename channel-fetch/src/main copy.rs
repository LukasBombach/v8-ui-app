use futures::executor::block_on;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

async fn hello_world() {
    println!("hello, world!");
}

struct Client {}

impl Client {
    fn request<V, R>(&self, value: V) -> R {}
}

struct Server<T> {}
impl<T> Server<T> {}

fn fetch<A, B>() -> (Client, Server) {
    let (tx1, rx1): (Sender<T>, Receiver<T>) = mpsc::channel();
    let (tx2, rx2): (Sender<T>, Receiver<T>) = mpsc::channel();
}

fn main() {
    let (request, respond) = fetch();

    // Sending thread
    let hello_thread = thread::spawn(move || {
        let response = request("hello?");
    });

    // Responding thread
    let world_thread = thread::spawn(move || {
        respond(|data| match data {
            "hello?" => "hello world!",
            _ => "",
        });
    });

    /* let (tx1, rx1): (Sender<&str>, Receiver<&str>) = mpsc::channel();
    let (tx2, rx2): (Sender<&str>, Receiver<&str>) = mpsc::channel();

    let thread_tx1 = tx1.clone();
    let thread_tx2 = tx2.clone();

    let hello_thread = thread::spawn(move || {
        thread_tx1.send("hello").unwrap();

        let message = rx2.recv().unwrap();
        println!("{}", message);
    });

    let world_thread = thread::spawn(move || {
        let message = rx1.recv().unwrap();
        println!("{}", message);

        thread_tx2.send("world").unwrap();
    });

    hello_thread.join().unwrap();
    world_thread.join().unwrap();

    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed */
}
