use sdl2::rect::Rect;


pub trait Collision {
    fn doesCollide(&self, hitbox: Rect) -> bool;
}
