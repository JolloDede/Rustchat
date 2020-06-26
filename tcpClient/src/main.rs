extern crate iui;
use iui::prelude::*;
use iui::controls::*;

use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use std::str;

const LOCAL: &str = "127.0.0.1:10000";
const MSG_SIZE: usize = 32;

fn main() {
    gui();
    two();
}

fn gui(){
    let ui = UI::init().expect("cant create UI");

    let mut win = Window::new(&ui, "Rustchat", 400, 400, WindowType::NoMenubar);

    let mut vbox = VerticalBox::new(&ui);
    let mut footer = HorizontalBox::new(&ui);

    let mut text_vbox = VerticalBox::new(&ui);
    text_vbox.set_padded(&ui, true);

    let text_msg = Label::new(&ui, "Message");
    let text_msgs = Label::new(&ui, "Message2");

    let message = Entry::new(&ui);

    let mut btn_send = Button::new(&ui, "Send");
    btn_send.on_clicked(&ui, {
        let text = message.value(&ui);
        println!("Send {}", text);
        move |_| {
            println!("Send {}", text);
        }
    });
    
    let btn_emoji = Button::new(&ui, "Emoji");

    text_vbox.append(&ui, text_msg, LayoutStrategy::Compact);
    text_vbox.append(&ui, text_msgs, LayoutStrategy::Compact);

    footer.append(&ui, btn_emoji, LayoutStrategy::Compact);
    footer.append(&ui, message, LayoutStrategy::Stretchy);
    footer.append(&ui, btn_send, LayoutStrategy::Compact);

    vbox.append(&ui, text_vbox, LayoutStrategy::Stretchy);
    vbox.append(&ui, footer, LayoutStrategy::Compact);

    win.set_child(&ui, vbox);
    win.show(&ui);
    ui.main();
}

// fn send(){
//     println!("Send")
// }

fn two (){
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();

                // println!("message recv {:?}", msg); Print the buffer

                let res = String::from_utf8(msg).expect("Found invalid UTF-8");
                println!("messag recv {}", res);
                
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection with server was severed");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("writing to socket failed");
                println!("message sent {:?}", msg);
            }, 
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }
        thread::sleep(Duration::from_millis(100));
    });

    println!("Write a Message:");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {break}
    }
    println!("bye bye!");
}