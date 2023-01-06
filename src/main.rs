extern crate sdl2;

use sdl2::mouse::MouseButton;
use sdl2::render::{Canvas, Texture};
use sdl2::{pixels::Color};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::{Rect, Point};
use sdl2::image::{LoadTexture};

const TITLE:&str             = "Paciência";
const WIDTH:u32              = 1024;
const HEIGHT:u32             = 768;
const BACKGROUND_COLOR:Color = Color::RGB(53, 101, 77);
const TEXTURE_FILE:&str      = "assets/images/cartas.png";
const CARD_WIDTH:u32         = 225 / 2;
const CARD_HEIGHT:u32        = 315 / 2;

static mut SELECTED_CARD:i32 = -1;

pub enum Naipe {
    COPAS   = 0,
    ESPADAS = 1,
    OUROS   = 2,
    PAUS    = 3
}

pub struct Card {
    idx: usize,
    valor: u32,
    naipe: u32,
    posicao: Point,
    offset: Point
}

pub struct GameContext<'a> {
    canvas:  &'a mut Canvas<sdl2::video::Window>,
    texture: &'a Texture<'a>,
    cards:   Vec<Card>
}

fn draw_frame(context : &mut GameContext) {
    context.canvas.set_draw_color(BACKGROUND_COLOR);
    context.canvas.clear();

    for carta in context.cards.iter_mut() {
        {
            let card = &carta;
            let tex_width  = context.texture.query().width;
            let tex_height = context.texture.query().height;

            let sprite_width  = tex_width/13;
            let sprite_height = tex_height/4;

            let sprite = Rect::new(i32::try_from(card.valor * sprite_width).unwrap(), i32::try_from(card.naipe * sprite_height).unwrap(), sprite_width, sprite_height);
            let dest   = Rect::new(card.posicao.x, card.posicao.y, CARD_WIDTH, CARD_HEIGHT);

            context.canvas.copy(context.texture, sprite, dest).unwrap();
        };
    }

    context.canvas.present();
}

fn intersects(x : i32, y : i32, card: &Card) -> bool {
    if (x >= card.posicao.x && x <= card.posicao.x + CARD_WIDTH as i32) &&
       (y >= card.posicao.y && y <= card.posicao.y + CARD_HEIGHT as i32) {
        return true;
    }
    return false;
}

fn mouse_btn_down(context : &mut GameContext, x : i32, y : i32) {
    for carta in context.cards.iter().rev() {
        if intersects(x, y, carta) {
            unsafe {
                SELECTED_CARD = carta.idx as i32;
                let carta = context.cards.get_mut(SELECTED_CARD as usize).unwrap();
                carta.offset = Point::new(x - carta.posicao.x, y - carta.posicao.y);
            }
            return;
        }
    }

    unsafe {
        SELECTED_CARD = -1;
    }
}

fn init_cards(cards : &mut Vec<Card>) {
    let mut idx = 0;
    for naipe in 0..4 {
        for valor in 0..13 {
            let pos = Point::new((50 * valor) as i32, (10 + naipe * CARD_HEIGHT) as i32);
            cards.push(Card { idx: idx, valor: valor, naipe: naipe, posicao: pos, offset: Point::new(0, 0)});
            idx = idx + 1;
        }
    }
}

fn create_cards() -> Vec<Card> {
    let mut cards = Vec::new();

    init_cards(&mut cards);

    cards
}

fn move_card(context : &mut GameContext, x : i32, y : i32) {
    unsafe {
        let carta = context.cards.get_mut(SELECTED_CARD as usize).unwrap();
        carta.posicao = Point::new(x - carta.offset.x, y - carta.offset.y);
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(TITLE, WIDTH, HEIGHT)
        .position_centered()
        .build()
        .expect("Não foi possível inicializar o subsistema de vídeo");

    let mut canvas      = window.into_canvas().build().expect("Não foi possível obter o canvas");
    let mut event_pump  = sdl_context.event_pump()?;
    let texture_creator = canvas.texture_creator();

    let texture = texture_creator.load_texture(TEXTURE_FILE).expect("Não foi possível carregar as texturas");

    let mut game_context = GameContext { canvas: &mut canvas, texture: &texture, cards: create_cards()};

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown {
                    x,
                    y,
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    mouse_btn_down(&mut game_context, x, y);
                },
                Event::MouseMotion { mousestate, x, y, .. } => {
                    if mousestate.left() {
                        unsafe {
                            if SELECTED_CARD >= 0 {
                                move_card(&mut game_context, x, y);
                            }
                        }
                    }
                },
                _ => {}
            }
        }

        draw_frame(&mut game_context);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}