pub enum Direction {
    Left,
    Right,
    Straight,
}

pub struct Position(pub i32, pub i32);

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

pub struct Car {
    position: Position,
    direction: Direction
}

impl Car {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            direction: Direction::Straight
        }
    }

    pub fn move_car(&mut self) {
        match self.direction {
            Direction::Left => self.position.0 -= 1,
            Direction::Right => self.position.0 += 1,
            Direction::Straight => {}
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, texture: &sdl2::render::Texture) {
        canvas.copy(texture, None, Some(sdl2::rect::Rect::new(self.position.0, self.position.1, 100, 100))).unwrap();
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}