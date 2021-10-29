extern crate sdl2;

use sdl2::EventPump;
use sdl2::mixer::Channel;
use sdl2::render::Canvas;
//use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::{WindowContext, Window};
use sdl2::rect::Rect;
use sdl2::keyboard::{KeyboardState, Scancode};

use crate::CollisionMod::Collision;
use crate::MapMod::Map;
use crate::SpriteLoader::FlipAnimation;
use crate::SpriteLoader::Sprites;
use crate::Skeleton;

use super::SpriteLoader::{Animation, StandardAnimation, Animations};

struct Vector(i32, i32);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player<'a> {
    animations: Animations<'a>,
    sword: Sprites<'a>,
    timer: u32,
    position: Rect,
    hitbox: Rect,
    velocity: Vector,
    direction: Direction,
    attackTimer: u32,
    trapped: bool,
}

impl<'a> Player<'a> {
    pub fn new(creator: &'a TextureCreator<WindowContext>, x: i32, y: i32) -> Player<'a> {
        let mut animations = vec![];
        for animation in SPRITES {
            let idxs: Vec<usize> = 
                (0..animation.len())
                .chain((1..animation.len() - 1)
                    .rev()
                )
                .collect();
            
            let animation = Animation::Standard(StandardAnimation::fromFiles(creator, *animation, &idxs).unwrap());
            animations.push(animation);
        }
        let left = Animation::Flip(FlipAnimation::new(1));
        let leftAttack = Animation::Flip(FlipAnimation::new(4));
        animations.push(left);
        animations.push(leftAttack);
        let animations = Animations::new(animations);
        let position = Rect::new(x, y, 50, 50);
        let hitbox = Rect::new(x + 2, y + 2, 46, 46);
        let velocity = Vector(0, 0);
        let sword = Sprites::new(creator, &[&"Resources/Images/Sword.png"]).unwrap();
        Player{animations, sword, timer: 0, position, hitbox, velocity, direction: Direction::Down, attackTimer: 0, trapped: false,}
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        if self.attackTimer > 0 {
            match self.direction {
                Direction::Down => {
                    self.animations.drawNextFrame(canvas, self.position);
                    self.sword.getSprite(0).draw(canvas,
                    Rect::new(SWORD_DOWN.0 + self.position.x(),
                    SWORD_DOWN.1 + self.position.y(),
                    SWORD_DOWN.2,
                    SWORD_DOWN.3), false, true);
                },
                Direction::Left => {
                    self.sword.getSprite(0).draw(canvas,
                    Rect::new(SWORD_LEFT.0 + self.position.x(),
                    SWORD_LEFT.1 + self.position.y(),
                    SWORD_LEFT.2,
                    SWORD_LEFT.3), false, false);
                    self.animations.drawNextFrame(canvas, self.position);
                },
                Direction::Right => {
                    self.sword.getSprite(0).draw(canvas,
                    Rect::new(SWORD_RIGHT.0 + self.position.x(),
                    SWORD_RIGHT.1 + self.position.y(),
                    SWORD_RIGHT.2,
                    SWORD_RIGHT.3), false, false);
                    self.animations.drawNextFrame(canvas, self.position);
                },
                Direction::Up => {
                    self.sword.getSprite(0).draw(canvas,
                    Rect::new(SWORD_UP.0 + self.position.x(),
                    SWORD_UP.1 + self.position.y(),
                    SWORD_UP.2,
                    SWORD_UP.3), false, false);
                    self.animations.drawNextFrame(canvas, self.position);
                },
            }
        }
        else {
            self.animations.drawNextFrame(canvas, self.position);
    
        }
    }

    pub fn update(&mut self, state: Option<KeyboardState>, events: &EventPump, mut channel: Channel, map: &Map, skeleton: &mut Skeleton) -> Channel {
        if let Some(state) = state {
            self.checkKeyboardInput(&state);
        }

        self.position.reposition((self.position.x + self.velocity.0, self.position.y));
        self.hitbox.reposition((self.hitbox.x + self.velocity.0, self.hitbox.y));

        if map.doesCollide(self.hitbox) || skeleton.doesCollide(self.hitbox) {
            self.position.reposition((self.position.x - self.velocity.0, self.position.y));
            self.hitbox.reposition((self.hitbox.x - self.velocity.0, self.hitbox.y));
        }

        self.position.reposition((self.position.x, self.position.y + self.velocity.1));
        self.hitbox.reposition((self.hitbox.x, self.hitbox.y + self.velocity.1));

        if map.doesCollide(self.hitbox) || skeleton.doesCollide(self.hitbox) {
            self.position.reposition((self.position.x, self.position.y - self.velocity.1));
            self.hitbox.reposition((self.hitbox.x, self.hitbox.y - self.velocity.1));
        }

        if !self.trapped {
            if self.position.x() >= THRESHOLD {
                channel = skeleton.trapPlayer(channel);
                self.trapped = true;
            }
        }
        
        self.timer += 1;

        if self.timer > 20 {
            self.timer = 0;
            self.animations.update();
        }

        if self.attackTimer > 0 {
            self.attackTimer -= 1;
            if self.attackTimer == 0 {
                match &self.direction {
                    Direction::Up => self.animations.changeAnimation(2),
                    Direction::Down => self.animations.changeAnimation(0),
                    Direction::Left => self.animations.changeAnimation(6),
                    Direction::Right => self.animations.changeAnimation(1),
                }.unwrap();
                self.checkKeyboardInput(&events.keyboard_state());
            }
        }
        channel
    }

    pub fn checkKeyboardInput(&mut self, state: &KeyboardState) {
        if self.attackTimer > 0 {return ();}

        if state.is_scancode_pressed(Scancode::Down) {
            self.velocity.1 = 3;
            self.animations.changeAnimation(0).unwrap();
            self.direction = Direction::Down;
        }
        else if state.is_scancode_pressed(Scancode::Up) {
            self.velocity.1 = -3;
            self.animations.changeAnimation(2).unwrap();
            self.direction = Direction::Up;
        }
        else {
            self.velocity.1 = 0;
        }

        if state.is_scancode_pressed(Scancode::Left) {
            self.velocity.0 = -3;
            self.animations.changeAnimation(6).unwrap();
            self.direction = Direction::Left;
        }
        else if state.is_scancode_pressed(Scancode::Right) {
            self.velocity.0 = 3;
            self.animations.changeAnimation(1).unwrap();
            self.direction = Direction::Right; 
        }
        else {
            self.velocity.0 = 0;
        }
        if state.is_scancode_pressed(Scancode::Space) || state.is_scancode_pressed(Scancode::KpSpace) {
            self.velocity = Vector(0, 0);
            self.attackTimer = 21;
            match &self.direction {
                Direction::Up => self.animations.changeAnimation(5),
                Direction::Down => self.animations.changeAnimation(3),
                Direction::Left => self.animations.changeAnimation(7),
                Direction::Right => self.animations.changeAnimation(4),
            }.unwrap();    
        }
    }

    fn relTupleToRect(&self, coords: (i32, i32, u32, u32)) -> Rect {
        Rect::new(
            coords.0 + self.position.x(),
            coords.1 + self.position.y(),
            coords.2,
            coords.3,
        )
    }

    pub fn attackCollision(&self, hitbox: Rect) -> bool {
        if self.attackTimer == 0 {
            return false;
        }
        let coords = match self.direction {
            Direction::Down => SWORD_DOWN_COLLISION,
            Direction::Left => SWORD_LEFT_COLLISION,
            Direction::Right => SWORD_RIGHT_COLLISION,
            Direction::Up => SWORD_UP_COLLISION,
        };
        let attackBox = self.relTupleToRect(coords);
        attackBox.has_intersection(hitbox)
    }
}




pub const THRESHOLD: i32 = 450;

const SWORD_DOWN: (i32, i32, u32, u32) = (10, 43, 30, 30);
const SWORD_RIGHT: (i32, i32, u32, u32) = (30, 5, 30, 30);
const SWORD_LEFT: (i32, i32, u32, u32) = (-10, 5, 30, 30);
const SWORD_UP: (i32, i32, u32, u32) = (0, -10, 50, 50);

const SWORD_DOWN_COLLISION: (i32, i32, u32, u32) = (23, 43, 4, 16);
const SWORD_RIGHT_COLLISION: (i32, i32, u32, u32) = (43, 5, 4, 16);
const SWORD_LEFT_COLLISION: (i32, i32, u32, u32) = (3, 5, 4, 16);
const SWORD_UP_COLLISION: (i32, i32, u32, u32) = (27, -10, 6, 27);


const NINJA_FLOAT: &[&str] = &[    
    &"Resources/Images/Ninja_float_0__half.png",
    &"Resources/Images/Ninja_float_1__half.png",
    &"Resources/Images/Ninja_float_2__half.png",
];

const NINJA_RIGHT_FLOAT: &[&str] = &[
    &"Resources/Images/Ninja_right_float_0.png",
    &"Resources/Images/Ninja_right_float_1.png",
    &"Resources/Images/Ninja_right_float_2.png",
];

const NINJA_UP_FLOAT: &[&str] = &[
    &"Resources/Images/Ninja_up_float_0__half.png",
    &"Resources/Images/Ninja_up_float_1__half.png",
    &"Resources/Images/Ninja_up_float_2__half.png",
];

const NINJA_ATTACK: &[&str] = &[
    &"Resources/Images/Ninja_attack__half.png",
];

const NINJA_UP_ATTACK: &[&str] = &[
    &"Resources/Images/Ninja_up_attack__half.png",
];

const NINJA_RIGHT_ATTACK: &[&str] = &[
    &"Resources/Images/Ninja_right_attack.png",
];

const SPRITES: &[&[&str]] = &[
    NINJA_FLOAT,
    NINJA_RIGHT_FLOAT,
    NINJA_UP_FLOAT,
    NINJA_ATTACK,
    NINJA_RIGHT_ATTACK,
    NINJA_UP_ATTACK,
];
