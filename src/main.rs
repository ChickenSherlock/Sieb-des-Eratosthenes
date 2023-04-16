use std::borrow::Borrow;
use std::thread;
use std::time::Duration;
use gtk::gdk::Screen;
use gtk::{Application, ApplicationWindow, Entry, Label, Grid, Button, Justification, StyleContext, CssProvider, glib, false_, Scrollable, ScrolledWindow};
use gtk::glib::MainContext;
use gtk::pango::WrapMode;
use gtk::prelude::*;

fn build_ui(app: &Application) {
    let go_button = Button::builder()
        .label("Start")
        .name("start")
        .build();

    let go_button_clone = go_button.clone();
    let status_label = Label::builder()
        .label("Status: paused")
        .build();

    let array_label = Label::builder()
        .name("label")
        .xalign(0.0)
        .yalign(0.0)
        .wrap(true)
        .max_width_chars(30)
        .wrap_mode(WrapMode::Word)
        .halign(gtk::Align::Start)
        .label("Liste:")
        .build();

    let array_grid = Grid::builder()
        .hexpand(true)
        .vexpand(true)
        .margin_start(7)
        .margin_top(7)
        .margin_end(7)
        .row_spacing(7)
        .column_spacing(7)
        .build();

    let scroll_view = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .child(&array_grid)
        .build();

    array_label.set_justify(Justification::Left);
    let length_entry = Entry::builder()
        .placeholder_text("Länge der Liste")
        .build();

    let delay_input = Entry::builder()
        .placeholder_text("Verzögerung (ms)")
        .build();

    let main_box = Grid::builder()
        .margin_bottom(7)
        .build();
    let array_grid_clone = array_grid.clone();
    let status_label_clone = status_label.clone();

    main_box.attach(&array_label,0,0,2,1);
    main_box.attach(&status_label,1,0,2,1);
    main_box.attach(&scroll_view,0,1,3,1);
    main_box.attach(&length_entry,0,2,1,1);
    main_box.attach(&delay_input,1,2,1,1);
    main_box.attach(&go_button,2,2,1,1);

    let (position_sender, position_receiver) = MainContext::channel::<(i32,i32)>(gtk::glib::Priority::DEFAULT);

    go_button.connect_button_press_event(move|_,_|{
        let mut exit = false;
        if &go_button_clone.label().unwrap() as &str == "Start"{
            exit = false;

            let length = match length_entry.buffer().text().parse::<i32>() {
                Ok(n) => n,
                Err(_) => {
                    0
                }
            };

            let delay = match delay_input.buffer().text().parse::<i32>() {
                Ok(n) => n,
                Err(_) => {
                    -1
                }
            };

            if length > 0 && delay>=0{
                go_button_clone.set_label("Stopp");
                go_button_clone.set_widget_name("stopp");
                status_label_clone.set_label("Status: Running");
                let mut row = 0;
                let mut col = 0;
                let iter = array_grid_clone.children();
                for widget in iter {
                    array_grid_clone.remove(&widget)
                }

                for i in 1..length+1 {
                    let number_label = Label::builder()
                        .label(i.to_string())
                        .name("norm")
                        .build();
                    array_grid_clone.attach(&number_label,col,row,1,1);

                    if i%10 == 0 {
                        row +=1;
                        col = 0
                    }else { col += 1 };
                }array_grid_clone.show_all();




                let ps = position_sender.clone();

                thread::spawn(move||{
                    let position_sender_clone = ps.clone();
                    let mut bool_array = vec![true;length as usize];

                    bool_array[0] = false;

                    for number in 1..length {
                        if number==1{continue}
                        println!("{:?}", number);
                        if bool_array[number as usize]{
                            for i in (number * number..length +1).step_by(number as usize) {
                                thread::sleep(Duration::from_millis(delay as u64));
                                bool_array[number as usize] = false;
                                position_sender_clone.send((i-1,0));

                            }
                        }

                    };
                    position_sender_clone.send((-1,1));
                    glib::Continue(true)
                });
            };
        }else {
            go_button_clone.set_label("Start");
            exit = true;
            go_button_clone.set_widget_name("start")
        }
        Inhibit(true)
    });

    position_receiver.attach(None,move|position|{
        if position.0 != -1{
            array_grid.children()[((array_grid.children().len() as i32 -1)-position.0) as usize].set_widget_name("red");
        }else { status_label.set_label("Status: Done") }
        glib::Continue(true)
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .resizable(false)
        .default_height(400)
        .default_width(400)
        .child(&main_box)
        .title("Sieb des Eratosthenes")
        .build();
    let provider = CssProvider::new();

    provider.load_from_data(include_str!("style.css").as_ref());


    StyleContext::add_provider_for_screen(
        &Screen::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    window.show_all();
}


fn main() {
    let app = Application::builder().application_id("me.ChickenSherlock.visualisierung").build();
    app.connect_activate(build_ui);
    app.run();
}
