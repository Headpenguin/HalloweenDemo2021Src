use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::SpriteLoader::Sprites;
use crate::CollisionMod::Collision;

#[derive(Default)]
struct Tile {
    idx: usize,
}

impl Tile {
    fn new(idx: usize) -> Tile {
        Tile {idx}
    }
}

struct TileRenderer<'a> {
    textures: Sprites<'a>,
}

impl<'a> TileRenderer<'a> {
    fn new(textures: Sprites) -> TileRenderer {
        TileRenderer{textures}
    }
    fn render(&self, tile: &Tile, quad: Rect, canvas: &mut Canvas<Window>) {
        self.textures.getSprite(tile.idx).draw(canvas, quad, false, false);
    }
}

enum CollisionType {
    Block,
    None,
}

impl Default for CollisionType {
    fn default() -> Self {
        CollisionType::None
    }
}
pub struct Map<'a> {
    tiles: [[Tile; 17]; 12],
    collisionMap: [[CollisionType; 17]; 12],
    renderer: TileRenderer<'a>,
}

impl<'a> Map <'a> {
    
    pub fn new(map: [[usize; 17]; 12], creator: &TextureCreator<WindowContext>) -> Map {
        let mut tiles: [[Tile; 17]; 12] = Default::default();
        let mut collisionMap: [[CollisionType; 17]; 12] = Default::default();
        
        for column in map.iter().enumerate() {
            
            for tile in column.1.iter().enumerate() {
                
                tiles[column.0][tile.0] = Tile::new(*tile.1);

                if *tile.1 == 1 {
                    collisionMap[column.0][tile.0] = CollisionType::Block;
                }

            }

        }

        let sprites = Sprites::new(creator, &[&"Resources/Images/Ground.png", &"Resources/Images/Wall.png"]).unwrap();
        let renderer = TileRenderer::new(sprites);
        Map {tiles, collisionMap, renderer}
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let mut quad = Rect::new(0, 0, 50, 50);
        for column in self.tiles.iter() {
            for tile in column {
                self.renderer.render(&tile, quad, canvas);
                quad.reposition((quad.x + 50, quad.y));
            }
            quad.reposition((0, quad.y + 50));
        }
    }

}

impl<'a> Collision for Map<'a> {
    fn doesCollide(&self, hitbox: Rect) -> bool {
        let leftBound = (hitbox.x as f32 / 50f32 ).floor() as usize;
        let rightBound = ((hitbox.x + hitbox.w) as f32 / 50f32 ).ceil() as usize;
        let topBound = (hitbox.y as f32 / 50f32 ).floor() as usize;
        let bottomBound = ((hitbox.y + hitbox.h) as f32 / 50f32 ).ceil() as usize;
        for y in topBound..bottomBound {
            for x in leftBound..rightBound {
                match self.collisionMap[y][x] {
                    CollisionType::Block => return true,
                    _ => (),
                }
            }
        }
        false
    }
}
