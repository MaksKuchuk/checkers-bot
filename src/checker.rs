#[derive(Clone)]
pub struct Checker {
    color: Order,
    is_king: bool,
}

impl Checker {
    pub fn new(color: Order) -> Checker {
        Checker {
            color,
            is_king: false,
        }
    }

    pub fn get_color(&self) -> Order {
        self.color.clone()
    }

    pub fn is_king(&self) -> bool {
        self.is_king
    }
}

#[derive(Clone, PartialEq)]
pub enum Order {
    WHITE,
    BLACK,
}
