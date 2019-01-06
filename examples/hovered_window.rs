//! Whenever the user hovers over a window, write its name to stdout.

extern crate i3ipc;

use i3ipc::event::Event;
use i3ipc::I3EventListener;
use i3ipc::Subscription;

fn main() {
    let mut listener = I3EventListener::connect().expect("failed to connect");
    listener
        .subscribe(&[Subscription::Window])
        .expect("failed to subscribe");
    for event in listener.listen() {
        match event {
            Ok(Event::WindowEvent(w)) => {
                println!("{}", w.container.name.unwrap_or("unnamed".to_owned()))
            }
            Err(e) => println!("Error: {}", e),
            _ => unreachable!(),
        }
    }
}
