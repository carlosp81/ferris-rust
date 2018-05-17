
extern crate ggez;
extern crate rand;
use ggez::{Context, GameResult};
use ggez::GameError;
use ggez::{audio, graphics};
use self::rand::Rng;
use game::MainState;
use game::entity::Lifetime;
use game::entity::Movement;
use game::entity::Entity;
use game::entity::EntityType;
use game::DEFAULT_FONT;


pub struct PowerupSpawner {
    pub max_cooldown: i64,
    pub current_cooldown: i64,

}

impl PowerupSpawner {
    pub fn new(cooldown: i64) -> PowerupSpawner {
        let p = PowerupSpawner {
            max_cooldown: cooldown,
            current_cooldown: cooldown,
        };
        p
    }   

    pub fn update(&mut self, delta_ms: u64, ctx: &mut Context) -> Option<Entity> {
        self.current_cooldown -= delta_ms as i64;
        if (self.current_cooldown <= 0)
        {
            let font = graphics::Font::new(ctx, DEFAULT_FONT, 48);
		    let text = graphics::Text::new(ctx, "Score: ", &font.unwrap()).unwrap();
            
            let e = Entity {
                text,
                entity_type: EntityType::Powerup,
                x: 0.0,
                y: 0.0,
                hp: 1,
                dam: 0,
                vel: 0.0,
                bounds: graphics::Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 32.0,
                    h: 32.0,
                },
		        movement: Movement::Linear(0.0, 65000.0),
                lifetime: Lifetime::Forever,
                seed: 0.0,
                timer: 0,
                bullet_cooldown: 0,
                angle: 0.0,
            };

            // Reset cooldown.
            self.current_cooldown = self.max_cooldown;

            // Return powerup entity option type.
            return Some(e);
        }
        None
    }
}