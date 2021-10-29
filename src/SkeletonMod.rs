use sdl2::mixer::Channel;
use sdl2::mixer::Chunk;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::video::{WindowContext, Window};

use crate::PlayerMod;
use crate::PlayerMod::Player;
use crate::SpriteLoader::Animations;
use crate::SpriteLoader::Animation;
use crate::SpriteLoader::Sprites;

pub struct Skeleton<'a> {
    sprites: Sprites<'a>,
    position: Rect,
    hitbox: Rect,
    timer: usize,
    playerIsTrapped: bool,
    gateHitBox: Rect,
    gateSound: Chunk,
    skeletonDie: Chunk
}

impl<'a> Skeleton<'a> {
    pub fn new(creator: &TextureCreator<WindowContext>, x: i32, y: i32) -> Skeleton {
        let sprites = Sprites::new(creator, SKELETON_SPRITES).unwrap();
        let position = Rect::new(x, y, 50, 50);
        let hitbox = Rect::new(x, y, 50, 100);
        let gateHitBox = Rect::new(PlayerMod::THRESHOLD - 50 , 150, 50, 50);
        let gateSound = Chunk::from_file(&"Resources/Music/Gate Sound.wav").unwrap();
        let skeletonDie = Chunk::from_file(&"Resources/Music/Skeleton Die.wav").unwrap();
        Skeleton{sprites, hitbox, position, timer: 0, playerIsTrapped: false, gateHitBox, gateSound, skeletonDie}
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        if self.playerIsTrapped {
            let state = (self.timer as f32 / 15f32 + 1f32).floor() as usize;
            self.sprites.getSprite(0).draw(canvas, self.position, false, false);
            self.sprites.getSprite(state).draw(canvas, Rect::new(
                self.position.x(),
                self.position.y() + 50,
                self.position.width(),
                self.position.height(),
            ), false, false);
            self.sprites.getSprite(3).draw(canvas, self.gateHitBox, false, false)
        }
        else {
            self.sprites.getSprite(0).draw(canvas, self.position, false, false);
            self.sprites.getSprite(1).draw(canvas, Rect::new(
                self.position.x(),
                self.position.y() + 50,
                self.position.width(),
                self.position.height(),
            ), false, false);
        }
    }

    pub fn trapPlayer(&mut self, channel: Channel) -> Channel {
        self.playerIsTrapped = true;
        channel.play(&self.gateSound, 0).unwrap()
    }

    pub fn doesCollide(&self, hitbox: Rect) -> bool {
        if self.playerIsTrapped {
            return self.gateHitBox.has_intersection(hitbox)
        }
        false
    }

    pub fn update(&mut self, player: &Player, mut channel: Channel) -> Channel {
        
        if self.playerIsTrapped && player.attackCollision(self.hitbox) {
            self.playerIsTrapped = false;
            channel = channel.play(&self.skeletonDie, 0).unwrap();
        }
        
        self.timer += 1;

        if self.timer > 29 {
            self.timer = 0;
        }

        channel
    }
}







const SKELETON_SPRITES: &[&str] = &[
    "Resources/Images/Skeleton_top__half.png",
    "Resources/Images/Skeleton_bottom__half.png",
    "Resources/Images/Skeleton_bottom_walk__half.png",
    "Resources/Images/Gate.png",
];
