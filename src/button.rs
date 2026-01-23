#[derive(PartialEq)]
pub enum ButtonStatus {
    Unknown,
    Pressed,
    Released
}

pub enum Button {
    User,
    Reset
}

impl Button {
    pub fn as_pin(&self) -> i32 {
        match *self {
            Button::Reset => 11,
            Button::User => 12,
        }
    }
}

pub fn init(btn: Button) {

}

pub fn read_status(btn: Button) -> ButtonStatus {
    ButtonStatus::Unknown
}