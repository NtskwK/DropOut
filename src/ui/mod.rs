use gtk::prelude::*;
use gtk::{Button, Window, WindowType};
use tokio::sync::mpsc::Sender;

pub enum UiEvent {
    StartGame,
}

pub fn init(tx: Sender<UiEvent>) {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Minecraft 启动器");
    window.set_default_size(350, 70);

    let button = Button::with_label("开始游戏");
    let tx_clone = tx.clone();
    button.connect_clicked(move |_| {
        println!("开始游戏按钮被点击");
        // Use blocking_send because we are in a synchronous callback
        if let Err(e) = tx_clone.blocking_send(UiEvent::StartGame) {
            eprintln!("Failed to send event: {}", e);
        }
    });

    window.add(&button);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();

    gtk::main();
}
