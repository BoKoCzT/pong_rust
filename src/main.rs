use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler, KeyCode};
use ggez::graphics::{self, Color, DrawParam};
use ggez::input::keyboard;
use ggez::nalgebra as na;

const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_SPEED: f32 = 6.0;   // Rýchlosť pohybu hráčovej pálky
const BALL_SIZE: f32 = 20.0;
const BALL_SPEED_X: f32 = 4.0;   // Horizontálna rýchlosť loptičky
const BALL_SPEED_Y: f32 = 4.0;   // Vertikálna rýchlosť loptičky
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

struct Paddle {
    pos: na::Point2<f32>,
}

impl Paddle {
    // Vytvorenie novej pálky na zadaných súradniciach (x, y)
    fn new(x: f32, y: f32) -> Self {
        Self {
            pos: na::Point2::new(x, y),
        }
    }

    // Pohyb pálky hore, ak nie je na vrchnej hrane obrazovky
    fn move_up(&mut self) {
        if self.pos.y > 0.0 {
            self.pos.y -= PADDLE_SPEED;
        }
    }

    // Pohyb pálky dole, ak nie je na spodnej hrane obrazovky
    fn move_down(&mut self) {
        if self.pos.y < SCREEN_HEIGHT - PADDLE_HEIGHT {
            self.pos.y += PADDLE_SPEED;
        }
    }
}

struct Ball {
    pos: na::Point2<f32>,     // Aktuálna pozícia loptičky
    vel: na::Vector2<f32>,    // Vektor rýchlosti loptičky (smer a rýchlosť)
}

impl Ball {
    // Vytvorenie novej loptičky na strede obrazovky
    fn new() -> Self {
        Self {
            pos: na::Point2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            vel: na::Vector2::new(BALL_SPEED_X, BALL_SPEED_Y),
        }
    }

    // Aktualizácia pozície loptičky
    fn update(&mut self) {
        self.pos += self.vel;

        // Odrazenie od horného a dolného okraja obrazovky
        if self.pos.y <= 0.0 || self.pos.y >= SCREEN_HEIGHT - BALL_SIZE {
            self.vel.y = -self.vel.y;
        }
    }

    // Kontrola, či loptička narazila do pálky a prípadná zmena smeru loptičky
    fn check_collision(&mut self, paddle: &Paddle) {
        if self.pos.x <= paddle.pos.x + PADDLE_WIDTH &&
           self.pos.y >= paddle.pos.y &&
           self.pos.y <= paddle.pos.y + PADDLE_HEIGHT {
            self.vel.x = -self.vel.x;   // Zmena horizontálneho smeru loptičky
        }
    }

    // Reset loptičky na stred obrazovky
    fn reset(&mut self) {
        self.pos = na::Point2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
        self.vel = na::Vector2::new(BALL_SPEED_X, BALL_SPEED_Y);
    }
}

struct MainState {
    left_paddle: Paddle,   // Hráčova pálka
    right_paddle: Paddle,  // Pálka ovládaná počítačom
    ball: Ball,            // Loptička
    left_score: i32,       // Skóre hráča
    right_score: i32,      // Skóre počítača
}

impl MainState {
    // Inicializácia hry so základnými hodnotami
    fn new() -> GameResult<MainState> {
        let s = MainState {
            left_paddle: Paddle::new(30.0, SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0),
            right_paddle: Paddle::new(SCREEN_WIDTH - 50.0, SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0),
            ball: Ball::new(),
            left_score: 0,
            right_score: 0,
        };
        Ok(s)
    }

    // Počítač automaticky ovláda pálku
    fn move_computer_paddle(&mut self) {
        // Ak je loptička nad pálkou, pálka sa pohybuje hore
        if self.ball.pos.y < self.right_paddle.pos.y {
            self.right_paddle.move_up();
        }
        // Ak je loptička pod pálkou, pálka sa pohybuje dole
        if self.ball.pos.y > self.right_paddle.pos.y + PADDLE_HEIGHT {
            self.right_paddle.move_down();
        }
    }
}

impl EventHandler for MainState {
    // Aktualizácia stavu hry (pohyb pálok a loptičky)
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Pohyb ľavej pálky podľa vstupu od hráča
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.left_paddle.move_up();
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.left_paddle.move_down();
        }

        // Pohyb počítačovej pálky
        self.move_computer_paddle();

        // Pohyb loptičky
        self.ball.update();

        // Kontrola kolízie loptičky s pálkami
        self.ball.check_collision(&self.left_paddle);
        self.ball.check_collision(&self.right_paddle);

        // Kontrola skórovania
        if self.ball.pos.x <= 0.0 {
            self.right_score += 1;   // Počítač získava bod
            self.ball.reset();       // Reset loptičky
        }
        if self.ball.pos.x >= SCREEN_WIDTH {
            self.left_score += 1;    // Hráč získava bod
            self.ball.reset();       // Reset loptičky
        }

        Ok(())
    }

    // Vykreslenie hry (pálky, loptička a skóre)
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);  // Vyčistenie obrazovky

        // Nakreslenie hráčovej pálky
        let paddle_rect = graphics::Rect::new(self.left_paddle.pos.x, self.left_paddle.pos.y, PADDLE_WIDTH, PADDLE_HEIGHT);
        let paddle_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), paddle_rect, Color::WHITE)?;
        graphics::draw(ctx, &paddle_mesh, DrawParam::default())?;

        // Nakreslenie počítačovej pálky
        let right_paddle_rect = graphics::Rect::new(self.right_paddle.pos.x, self.right_paddle.pos.y, PADDLE_WIDTH, PADDLE_HEIGHT);
        let right_paddle_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), right_paddle_rect, Color::WHITE)?;
        graphics::draw(ctx, &right_paddle_mesh, DrawParam::default())?;

        // Nakreslenie loptičky
        let ball_rect = graphics::Rect::new(self.ball.pos.x, self.ball.pos.y, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), ball_rect, Color::WHITE)?;
        graphics::draw(ctx, &ball_mesh, DrawParam::default())?;

        graphics::present(ctx)?;  // Aktualizovanie obrazovky
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("pong", "ggez").window_setup(ggez::conf::WindowSetup::default().title("Pong"));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
