use web_sys::HtmlInputElement;
use yew::{Event, TargetCast};

pub trait InputEventExt {
    fn input_value(&self) -> Option<String>;
}

impl InputEventExt for Event {
    fn input_value(&self) -> Option<String> {
        self.target_dyn_into::<HtmlInputElement>()
            .map(|x| x.value())
    }
}
