use gtk::prelude::*;
mod infix;
pub struct Button{
    val:String,
    screen: gtk::Entry,
    b:gtk::Button
}
impl Button {
    fn new(val:String,screen: gtk::Entry ) -> Button {
        let b =gtk::Button::with_label(&val);  
        let v=val.clone();
        let screen_ =screen.clone();
        b.connect_clicked( move |_|{
            println!("{}",v);
            // let a = v.clone()
            let mut que = screen.get_text().to_string();
            que.push_str(v.as_str());
            screen.set_text(que.as_str());
            // clicked(self.val.clone());

        });
        Button{val,screen:screen_,b}
    }
    pub fn clicked(&mut self ,val:String){
        let mut old = self.screen.get_text().to_string();
        // let new = self.val.chars().collect();
        old.push_str(&val);
        self.screen.set_text(&old);
    }

}
pub fn init()-> gtk::Box{
    // let val2 = self.val.clone();
    
    let main_box=gtk::BoxBuilder::new().homogeneous(true).spacing(10).expand(true).orientation(gtk::Orientation::Vertical);    
    let main_box=main_box.build();
    let box2 =gtk::BoxBuilder::new().homogeneous(true).spacing(10).expand(true).orientation(gtk::Orientation::Horizontal).build();
    let box1 =gtk::BoxBuilder::new().homogeneous(true).spacing(10).expand(true).orientation(gtk::Orientation::Horizontal).build();
    let box3 =gtk::BoxBuilder::new().homogeneous(true).spacing(10).expand(true).orientation(gtk::Orientation::Horizontal).build();
    let box4 =gtk::BoxBuilder::new().homogeneous(true).spacing(10).expand(true).orientation(gtk::Orientation::Horizontal).build();
    let screen = gtk::Entry::with_buffer(&gtk::EntryBuffer::new(None));
    // let screen=screen.clone()_;
    let a = screen.clone();
    let b1=Button::new("1".to_string(),a);
    // b1.connect_clicked(|_|{
    //     // println!("{:?}",s);
    //     println!("{}",val);
    // });
    // let b2=gtk::Button::with_label("2");
    let a= screen.clone();
    let b2=Button::new("2".to_string(),a);
    let b3=Button::new("3".to_string(),screen.clone());
    let b4=Button::new("4".to_string(),screen.clone());
    let b5=Button::new("5".to_string(),screen.clone());
    let b6=Button::new("6".to_string(),screen.clone());
    let b7=Button::new("7".to_string(),screen.clone());
    let b8=Button::new("8".to_string(),screen.clone());
    let b9=Button::new("9".to_string(),screen.clone());
    let be=Button::new("=".to_string(),screen.clone());
    let bp=Button::new("+".to_string(),screen.clone());
    let bm=Button::new("-".to_string(),screen.clone());
    let bmu=Button::new("*".to_string(),screen.clone());
    let bd=Button::new("/".to_string(),screen.clone());
    let bsqrt=Button::new("sqrt".to_string(),screen.clone());
    let bchange=Button::new("+/-".to_string(),screen.clone());
    let bac=Button::new("AC".to_string(),screen.clone());
    let b0=Button::new("0".to_string(),screen.clone());
    let b00=Button::new("00".to_string(),screen.clone());
    let bper = Button::new(".".to_string(),screen.clone());
    box1.add(&bsqrt.b);
    box1.add(&b7.b);
    box1.add(&b8.b);
    box1.add(&b9.b);
    box1.add(&bd.b);
    box2.add(&bchange.b);
    box2.add(&b4.b);
    box2.add(&b5.b);
    box2.add(&b6.b);
    box2.add(&bmu.b);
    box3.add(&bac.b);
    box3.add(&b1.b);
    box3.add(&b2.b);
    box3.add(&b3.b);
    box3.add(&bm.b);
    box4.add(&b0.b);
    box4.add(&b00.b);
    box4.add(&bper.b);
    box4.add(&be.b);
    box4.add(&bp.b);
    let a = screen.clone();
    main_box.add(&a);
    main_box.add(&box1);
    main_box.add(&box2);
    main_box.add(&box3);
    main_box.add(&box4);
    main_box
    

}
// }