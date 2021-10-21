extern crate sdl2;

use sdl2::EventPump;
use sdl2::render::Canvas;
//use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::{WindowContext, Window};
use sdl2::rect::Rect;
use sdl2::keyboard::{KeyboardState, Scancode};

use crate::CollisionMod::Collision;
use crate::MapMod::Map;
use crate::SpriteLoader::FlipAnimation;

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
    timer: u32,
    position: Rect,
    hitbox: Rect,
    velocity: Vector,
    direction: Direction,
    attackTimer: u32,
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
        Player{animations, timer: 0, position, hitbox, velocity, direction: Direction::Down, attackTimer: 0,}
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        self.animations.drawNextFrame(canvas, self.position);
    }

    pub fn update(&mut self, state: Option<KeyboardState>, events: &EventPump, map: &Map) {
        
        if let Some(state) = state {
            self.checkKeyboardInput(&state);
        }

        self.position.reposition((self.position.x + self.velocity.0, self.position.y + self.velocity.1));
        self.hitbox.reposition((self.hitbox.x + self.velocity.0, self.hitbox.y + self.velocity.1));

        if map.doesCollide(self.hitbox) {
            self.position.reposition((self.position.x - self.velocity.0, self.position.y - self.velocity.1));
            self.hitbox.reposition((self.hitbox.x - self.velocity.0, self.hitbox.y - self.velocity.1));
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
}







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
