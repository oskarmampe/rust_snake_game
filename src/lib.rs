use std::collections::VecDeque;

pub type Position = (usize, usize);

#[derive(Debug)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left
}

#[derive(Debug)]
pub struct SnakeGame {
    width: usize,
    height: usize,
    // Head is the first item, tail is the last item
    // Since it's ordered.
    snake: VecDeque<Position>,
    direction: Direction,
    food: Position,
    lost: bool,
}

impl SnakeGame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            snake: [((width - 2).max(0), (height / 2))].into_iter().collect(),
            direction: Direction::Left,
            food: {(2.min(width - 1), height / 2)},
            lost: false,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match (&self.direction, direction) {
            (Direction::Top, Direction::Top) |
            (Direction::Top, Direction::Bottom) |
            (Direction::Right, Direction::Right) |
            (Direction::Right, Direction::Left) |
            (Direction::Bottom, Direction::Top) |
            (Direction::Bottom, Direction::Bottom) |
            (Direction::Left, Direction::Right) |
            (Direction::Left, Direction::Left) => {}
            (_, direction) => {
                self.direction = direction
            }
        }
    }

    pub fn is_valid(&mut self, (x, y): Position) -> bool {
        x < self.width && y < self.height
    }
    
    pub fn tick(&mut self) {
        if self.lost {
            return;
        }
        // Move the snake by removing the last item, and adding to the first item.
        // This functions a lot like a double ended queue.
        let head = self.snake.get(0);
        
        let new_head = head.map(|&(x,y)| match self.direction {
            Direction::Top => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Bottom => (x, y + 1),
            Direction::Left => (x - 1, y),
        });

        
        if let Some(new_head) = new_head {
            if !self.is_valid(new_head) || self.snake.contains(&new_head) {
                self.lost = true;
            } else {
                // Remove the tail
                self.snake.pop_back();
                // Add to the head
                self.snake.push_front(new_head);
            }

        }
    }
}

#[cfg(test)]
mod tests {
     use crate::SnakeGame;
     
     #[test]
     fn test() {
         println!("{:?}", SnakeGame::new(10, 10));
     }
}