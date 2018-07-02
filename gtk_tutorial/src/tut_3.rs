use gtk::*;
use std::sync::atomic::{Ordering, AtomicUsize};
use std::sync::Arc;

const MESSAGES : [&str; 3] = ["Ouch! You hit me!", "...", "Thanks!"];

pub struct App {
    pub(in tut_3) window: Window,
    pub(in tut_3) header: Header,
    pub(in tut_3) content: Content,
    pub(in tut_3) health: Arc<HealthComponent>
}

pub struct Header {
    pub (in tut_3) container: HeaderBar,
    pub (in tut_3) hit: Button,
    pub (in tut_3)  heal: Button
}

pub struct Content {
    pub (in tut_3) container: Box,
    pub (in tut_3) health: Label,
    pub (in tut_3) message: Label
}

pub struct HealthComponent(AtomicUsize);

#[repr(u8)]
enum Message { Hit, Dead, Heal }

impl HealthComponent {
    fn new(initial : usize) -> HealthComponent {
        HealthComponent(AtomicUsize::new(initial))
    }

    fn get_health(&self) -> usize { self.0.load(Ordering::SeqCst)}

    fn subtract(&self, value: usize) -> usize {
        let current = self.0.load(Ordering::SeqCst);
        let new = if current < value {0} else {current - value};
        self.0.store(new, Ordering::SeqCst);
        new
    }

    fn heal(&self, value: usize) -> usize {
        let original = self.0.fetch_add(value, Ordering::SeqCst);
        original + value
    }
}


impl App {
    pub fn new() -> App {
        let health = HealthComponent::new(10);
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new(&health);

        let health = Arc::new(health);

        window.set_titlebar(&header.container);
        window.set_title("Button Boxer");
        window.set_wmclass("app-name", "Button Boxer");
        Window::set_default_icon_name("iconname");
        window.add(&content.container);


        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });



        // setup the hit button to subtract health
        {
            let health = health.clone();
            let message = content.message.clone();
            let info = content.health.clone();

            header.hit.clone().connect_clicked(move |_| {
                let new_health = health.subtract(1);
                let action = if new_health == 0 { Message::Dead } else { Message::Hit };
                message.set_label(MESSAGES[action as usize]);
                info.set_label(new_health.to_string().as_str());
            });
        }


        {
            let health = health.clone();
            let message = content.message.clone();
            let info = content.health.clone();
            header.heal.clone().connect_clicked(move |_| {
                let new_health = health.heal(5);
                message.set_label(MESSAGES[Message::Heal as usize]);
                info.set_label(new_health.to_string().as_str());
            });
        }


        App { window, header, content, health}
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

       container.set_show_close_button(true);

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
    fn new(health: &HealthComponent) -> Content {
       let container = Box::new(Orientation::Vertical, 0);
        let health_info = Box::new(Orientation::Vertical, 0);
        let health_label = Label::new("Current Health:");
        let health = Label::new(health.get_health().to_string().as_str());


        health_info.set_halign(Align::Center);
        health_label.set_halign(Align::Start);
        health.set_halign(Align::Start);

        health_info.pack_start(&health_label, false, false, 5);
        health_info.pack_start(&health, true, true, 5);

        let message = Label::new("Hello");


        container.pack_start(&health_info, true, false, 0);
        container.pack_start(&Separator::new(Orientation::Horizontal), false, false, 0);
        container.pack_start(&message, true, false, 0);

        Content { container, health, message }
    }

}

