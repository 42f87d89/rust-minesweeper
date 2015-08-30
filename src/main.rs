
extern crate sdl;

use sdl::video::{SurfaceFlag, VideoFlag, Color};
use sdl::event::{Event, Key, Mouse};

fn init(screen_width: isize, screen_height: isize) -> sdl::video::Surface {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("mines", "mines");

    let screen = match sdl::video::set_video_mode(screen_width, screen_height, 32,
                                                  &[SurfaceFlag::HWSurface],
                                                  &[VideoFlag::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };
    screen
}

fn draw_field(ref field: &Vec<Vec<bool>>, length: i16, screen: &sdl::video::Surface) {
    let mut n = 0;
    for ref i in field.iter() {
        let mut m = 0;
        for ref j in i.iter() {
            screen.fill_rect(Some(
                sdl::Rect {x: m as i16*length as i16, y: n as i16*length as i16, w: length as u16, h: length as u16}
            ), if **j {Color::RGB(0, 0, 0)} else {Color::RGB(255, 255, 255)});
            m += 1;
        }
        n += 1;
    }
}

fn flip_field(ref mut field: &mut Vec<Vec<bool>>, x: i16, y: i16, size: i16) {
    let i = (x/size) as usize;
    let j = (y/size) as usize;
    field[i][j] = !(field[i][j]);
}

fn main() {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 30;
    const SIZE: usize = 25;

    let field = &mut vec![vec![false; WIDTH]; HEIGHT];

    let screen = init((SIZE*WIDTH) as isize, (SIZE*HEIGHT) as isize);
    loop {
        match sdl::event::poll_event() {
            Event::Quit => break,
            Event::MouseButton(_, down, mx, my) => {
                if down {
                    flip_field(field, my as i16, mx as i16, SIZE as i16);
                    for ref i in field.iter(){
                        println!("{:?}", i);
                    }
                    println!("");
                }
            },
            Event::Key(k, _, _, _)
                if k == Key::Escape
                    => break,
            _ => {}
        }
        draw_field(&field, SIZE as i16, &screen);
        screen.flip();
    }

    sdl::quit();
}
