use godot::prelude::*;
use godot::classes::{Button, INode};

#[derive(GodotClass)]
#[class(base=Node)]
struct RustHelloWorld {
    #[export]
    submit_button: Option<Gd<Button>>,

    #[export]
    text: GString,

    #[export]
    number_of_displays: i32,

    #[export]
    frame_count: i32,

    base: Base<Node>,
}

#[godot_api]
impl RustHelloWorld {
    #[signal]
    fn on_button_pressed(hello_world_text: GString);  

    #[signal]
    fn on_count_changed_as_text(count_as_text: GString);  

    #[func]
    fn display_text_in_inspector(&mut self) {
        for _ in 0..self.number_of_displays {
            godot_print!("{}", self.text);
        }

        let text_variant = self.text.to_variant();
        self.base_mut().emit_signal("on_button_pressed", &[text_variant]);
    }
}

#[godot_api]
impl INode for RustHelloWorld {
    fn init(base: Base<Node>) -> Self {
        Self {
            submit_button: None,
            text: GString::from("Hello, world!"),
            number_of_displays: 5,
            frame_count: 0,
            base,
        }
    }

    fn ready(&mut self) {
        let callable = self.base().callable("display_text_in_inspector");

        let Some(button) = self.submit_button.as_mut() else {
            godot_warn!("submit_button is not assigned in the editor!");
            return;
        };

       
        let err = button.connect("pressed", &callable);

        if err != godot::global::Error::OK {
            godot_error!("Failed to connect 'pressed' signal: {err:?}");
        } else {
            godot_print!("Successfully connected button.pressed → display_text_in_inspector");
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        self.frame_count += 1;

        let count_as_text = format!("Frame count: {}", self.frame_count);
        let count_as_text_variant = count_as_text.to_variant();
        self.base_mut().emit_signal("on_count_changed_as_text", &[count_as_text_variant]);

    }
}