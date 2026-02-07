mod button;
mod led;
mod mcu;
mod board_error;

pub use button::Button;
pub use led::Led;


pub struct Board {
    // Led and button resources will be stored here, so they can be taken by the user.
}

impl Board {
    
}

pub static mut BOARD: Board = Board {
    // Initialize the board with all resources available.
};
