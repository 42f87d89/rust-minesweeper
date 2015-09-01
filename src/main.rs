extern crate sdl;
extern crate rand;

use rand::distributions::{IndependentSample, Range};
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

fn draw_square(x: i16, y: i16, w: u16, (r,g,b): (u8, u8, u8), screen: &sdl::video::Surface) {
    screen.fill_rect(Some(sdl::Rect {x: x, y: y, w: w, h: w}), Color::RGB(r, g, b));
}

fn draw_num(n: u8, x: i16, y: i16, size: u16, screen: &sdl::video::Surface) {
    draw_square(x,
        y,
        size,
        (255, 255, 255),
        screen
    );
    let mut n = n;
    let sub = (size-1) as i16/3;
    let pack = (size as i16-sub*3)as i16/2;
    let color = if n == 1 {
        (100,100,255)
    }else if n == 2 {
        (0,255,0)
    }else if n == 3 {
        (255,0,0)
    }else if n == 4 {
        (0,0,255)
    }else if n == 5 {
        (200,50,0)
    }else if n == 6 {
        (0,255,255)
    }else if n == 7 {
        (0,0,0)
    }else {
        (150,150,150)
    };
    
    loop {
        if n == 0 {break;}
        else if n == 1{
            draw_square(sub+x+pack, sub+y+pack, sub as u16-1, color, screen);
            break;
        }else if n == 2{
            draw_square(x+pack, y+pack, sub as u16-1,  color, screen);
            draw_square(sub*2+x+pack, sub*2+y+pack, sub as u16-1, color, screen);
            break;
        }else if n == 3{
            draw_square(sub+x+pack, sub+y+pack, sub as u16-1, color, screen);
            n = 2;
        }else if n == 4{
            draw_square(x+pack, sub*2+y+pack, sub as u16-1, color, screen);
            draw_square(sub*2+x+pack, y+pack, sub as u16-1, color, screen);
            n = 2;
        }else if n == 5{
            draw_square(sub+x+pack, sub+y+pack, sub as u16-1, color, screen);
            n = 4;
        }else if n == 6{
            draw_square(x+pack, sub+y+pack, sub as u16-1, color, screen);
            draw_square(sub*2+x+pack, sub+y+pack, sub as u16-1, color, screen);
            n = 4;
        }else if n == 7{
            draw_square(sub+x+pack, sub+y+pack, sub as u16-1, color, screen);
            n = 6;
        }else if n == 8{
            draw_square(sub+x+pack, y+pack, sub as u16-1, color, screen);
            draw_square(sub+x+pack, sub*2+y+pack, sub as u16-1, color, screen);
            n = 6;
        }
    }
}

fn draw_field(ref field: &Vec<Vec<(bool,bool,bool)>>,
              length: i16,
              screen: &sdl::video::Surface) {
    let mut n = 0;
    for ref i in field.iter() {
        let mut m = 0;
        for ref sq in i.iter() {
            let hidden = sq.0;
            let mine = sq.1;
            let flag = sq.2;

            if hidden {
                draw_square(m*length+1,
                    n*length+1,
                    (length-1) as u16,
                    (180, 180, 180),
                    screen
                );
                if flag {
                    draw_square(m*length+4,
                        n*length+4,
                        (length-7) as u16,
                        (255, 0, 0),
                        screen
                    );
                }
            }else{
                if mine {
                    draw_square(m*length+1,
                        n*length+1,
                        (length-1) as u16,
                        (0, 0, 0),
                        screen
                    );
                }else{
                    draw_num(
                        count_field(m as usize, n as usize, field),
                        m as i16*length as i16+1,
                        n as i16*length as i16+1,
                        (length-1) as u16,
                        screen
                    );
                }
            }
            m += 1;
        }
        n += 1;
    }
}

fn make_mine(ref mut field: &mut Vec<Vec<(bool,bool,bool)>>, x: i16, y: i16, size: i16) {
    let i = (x/size) as usize;
    let j = (y/size) as usize;
    let (h, mine, f) = field[i][j];
    field[i][j] = (h, !mine, f);
}

fn random_field(r: f32, field: &mut Vec<Vec<(bool,bool,bool)>>) {
    let bt = Range::new(0.,1.);
    let mut rng = rand::thread_rng();
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            if bt.ind_sample(&mut rng) < r {
                field[i][j] = (true, true, false);
            }else{
                field[i][j] = (true, false, false);
            }
        }
    }
}

fn show_spot(ref mut field: &mut Vec<Vec<(bool,bool,bool)>>, x: i16, y: i16, size: i16) {
    let i = (x/size) as usize;
    let j = (y/size) as usize;
    let (_, m, flag) = field[i][j];
    if !flag {
        field[i][j] = (false, m, flag);
    }
}

fn flag_spot(ref mut field: &mut Vec<Vec<(bool,bool,bool)>>, x: i16, y: i16, size: i16) {
    let i = (x/size) as usize;
    let j = (y/size) as usize;
    let (hidden, m, flag) = field[i][j];
    if hidden {
        field[i][j] = (hidden, m, !flag);
    }
}

fn count_field(x: usize, y: usize, field: &Vec<Vec<(bool,bool,bool)>>) -> u8 {
    let w = field[0].len();
    let h = field.len();
    let mut n = 0;
    for i in 0..3 {
        if y+i == 0 || y+i == h+1 {continue;}
        for j in 0..3 {
            if i == 1 && j == 1 {continue;}
            if x+j == 0 || x+j == w+1 {continue;}
            if field[y+i-1][x+j-1].1 {n += 1}
        }
    }
    n
}

fn main() {
    const WIDTH: usize = 30;
    const HEIGHT: usize = 20;
    const SIZE: usize = 35;

    let field = &mut vec![vec![(true, false, false); WIDTH]; HEIGHT];

    let screen = init((SIZE*WIDTH) as isize + 1, (SIZE*HEIGHT) as isize + 1);
    loop {
        match sdl::event::poll_event() {
            Event::Quit => break,
            Event::MouseButton(b, down, mx, my) => {
                if down {
                    if b == Mouse::Left {
                        show_spot(field, my as i16, mx as i16, SIZE as i16);
                    }
                    else if b == Mouse::Right {
                        flag_spot(field, my as i16, mx as i16, SIZE as i16);
                    }
                }
            },
            Event::Key(k, down, _, _) => {
                if down {
                    if k == Key::Escape {
                        break;
                    }
                    if k == Key::R {
                        random_field(0.1, field);
                    }
                }
            }
            _ => {}
        }
        draw_field(&field, SIZE as i16, &screen);
        screen.flip();
    }

    sdl::quit();
}
