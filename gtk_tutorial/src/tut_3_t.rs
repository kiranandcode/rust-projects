use gtk::*;
use std::sync::atomic::{Ordering, AtomicUsize};
use std::sync::Arc;


pub struct App {
    window: Window,
    header: Header,
    content: Content,
    health: Arc<HealthContainer>
}

pub struct HealthContainer(AtomicUsize);

const Messages : [&str;3] = ["Ouch! You hit me!", "...", "Thanks! I feel better now" ];

struct Header {
    pub(in tut_3_t) container: HeaderBar,
    pub(in tut_3_t) hit: Button,
    pub(in tut_3_t) heal: Button,
}

struct Content {
    pub(in tut_3_t) container: Box,
    pub(in tut_3_t) health: Label,
    pub(in tut_3_t) message: Label
}

#[repr(u8)]
enum Message {
    Hit = 0, Dead = 1, Heal = 2,
}

impl HealthContainer {
    pub fn heal(&self, amt: usize) -> usize {
        let old_health = self.0.fetch_add(amt, Ordering::SeqCst);
        old_health + amt
    }


    pub fn hit(&self, amt: usize) -> usize {
        let old_health = self.0.load(Ordering::SeqCst);
        let new_health = if old_health < amt { 0 } else { old_health - amt };
        self.0.store(amt, Ordering::SeqCst);
        new_health
    }

    pub fn get_health(&self) -> usize { self.0.load(Ordering::SeqCst) }
}

impl App {
    pub fn new() -> App {
        let health = HealthContainer(AtomicUsize::new(10));
        let header = Header::new();
        let content = Content::new(&health);
        let window = Window::new(WindowType::Toplevel);

        let health = Arc::new(health);
        window.set_titlebar(&header.container);
        window.set_title("Button Boxer");
        window.set_wmclass("button-boxer", "Button Boxer");
        Window::set_default_icon_name("iconname");
        window.add(&content.container);

        window.connect_delete_event(move |_, _| {
            main_quit(); 
            Inhibit(false)
        });



        {
            let health = health.clone();
            let message = content.message.clone();
            let info = content.health.clone();

            header.hit.clone().connect_clicked(move |_| {
                let new_health = health.hit(1);
                let action = if new_health == 0 { Message::Dead } else { Message::Hit };
                message.set_label(Messages[action as usize]);
                info.set_label(new_health.to_string().as_str());
            });
        }


        {
            let health = health.clone();
            let message = content.message.clone();
            let info = content.health.clone();
            header.heal.clone().connect_clicked(move |_| {
                let new_health = health.heal(5);
                message.set_label(Messages[Message::Heal as usize]);
                info.set_label(new_health.to_string().as_str());
            });
        }

        App {
            window,
            header,
            content,
            health 
        }
    }

    pub fn run(&self) {
        self.window.show_all();
        main();
    }
}

impl Header {

    fn new() -> Header {
        let container = HeaderBar::new();
        container.set_title("Button Boxer");

        container.set_show_close_button(false);

        let hit = Button::new_with_label("Hit");
        let heal = Button::new_with_label("Heal");

        hit.get_style_context().map(|c| c.add_class("destructive-action"));
        heal.get_style_context().map(|c| c.add_class("suggested-action"));

        container.pack_start(&hit);
        container.pack_end(&heal);


        Header { container, hit, heal }
    }
}

impl Content {
    fn new(health: &HealthContainer) -> Content {
       let container = Box::new(Orientation::Vertical, 0); 
       let health_info = Box::new(Orientation::Horizontal, 0);

       let message = Label::new("Hello");
       let health_label = Label::new("Current Health:");
       let health = Label::new(health.get_health().to_string().as_str());


       health_info.set_halign(Align::Center);
       health_label.set_halign(Align::Start);
       health.set_halign(Align::Start);


       health_info.pack_start(&health_label, false, false, 5);
       health_info.pack_start(&health, true, true, 5);


       container.pack_start(&health_info, true, false, 0);
       container.pack_start(&Separator::new(Orientation::Horizontal), false, false, 0);
       container.pack_start(&message, true, false, 0);

       Content { container, health, message }

    }
}
