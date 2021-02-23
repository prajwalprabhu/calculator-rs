// To import all needed traits.
use gio::prelude::*;
use gtk::prelude::*;
use std::env;
// mod layout;
mod infix;

fn main() {
    let uiapp = gtk::Application::new(
        Some("org.gtkrsnotes.demo"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");
    uiapp.connect_activate(|app| {
        let win = gtk::ApplicationWindow::new(app);
        // let layout = include!("layout.rs");
        // let layout =layout::init();
        let layout = run();
        win.add(&layout);
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
    println!("Done");
}

fn run()->gtk::Box{
    let main_box = gtk::BoxBuilder::new()
        .homogeneous(true)
        .spacing(10)
        .expand(true)
        .orientation(gtk::Orientation::Vertical);
    let main_box = main_box.build();
    
    let screen = gtk::Entry::with_buffer(&gtk::EntryBuffer::new(None));
    let mut boxes:Vec<gtk::Box>=Vec::new(); 
    for _ in 0..4{
        boxes.push(gtk::BoxBuilder::new()
        .homogeneous(true)
        .spacing(10)
        .expand(true)
        .orientation(gtk::Orientation::Horizontal)
        .build())
    }
    
let value = [["sqrt","7","8","9","/"],
["+/-","4","5","6","*"],
["AC","1","2","3","-"],
["0","00",".","=","+"]];
for (i,val) in boxes.iter().enumerate() {
    for &j in value[i].iter(){
        let b = gtk::Button::with_label(j);
        let screen_ = screen.clone();
        b.connect_clicked(move |_| {
            let mut old =screen_.get_text().to_string();
            if j== "=" {
                old = format!("{}", infix::eval(old.clone()));
            } else {
                old.push_str(j);
            }
            screen_.set_text(&old);
        });

        val.add(&b);

    }
}
main_box.add(&screen);
for i in boxes{
    main_box.add(&i);
}
main_box
}