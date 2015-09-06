extern crate sdl;
extern crate rand;

use rand::distributions::{IndependentSample, Range};
use sdl::video::{SurfaceFlag, VideoFlag, Color};
use sdl::event::{Event, Key, Mouse};

#[derive(Copy, Clone)]
struct Spot {
    hidden: bool,
    mine: bool,
    flag: bool,
    n: u8,
}

struct Field {
    width: usize,
    height: usize,
    field: Vec<Vec<Spot>>,
}

impl Field {
    fn new(r: f32, w: usize, h: usize) -> Field {
        let bt = Range::new(0.,1.);
        let mut rng = rand::thread_rng();
        let mut field = Vec::with_capacity(h);
        for i in 0..h {
            field.push(Vec::with_capacity(w));
            for _ in 0..w {
                field[i].push(
                    Spot{hidden: true,
                         mine: bt.ind_sample(&mut rng) < r,
                         flag: false,
                         n: 0}
                );
            }
        }
        let mut f = Field {width: w, height: h, field: field};
        f.count_field();
        f
    }

    fn swap_mine(&mut self, x: usize, y: usize) {
        let f = &mut self.field[y][x];
        f.mine = !f.mine;
    }

    fn show_spot(&mut self, x: usize, y: usize) {
        if x >= self.width {return;}
        if y >= self.height {return;}
        {
            let f = &mut self.field[y][x];
            if f.flag {return;}
            if f.hidden {
                f.hidden = false;
                return;
            }
        }
        if self.field[y][x].n != self.count_flags(x, y) {return;}
        for i in 0..3 {
            if x+i == 0 || x+i > self.width {continue;}
            for j in 0..3 {
                if y+j == 0 || y+j > self.height {continue;}
                if i ==1 && j == 1 {continue;}
                let f = self.field[y+j-1][x+i-1];
                if f.hidden {
                    self.show_spot(x+i-1, y+j-1);
                }
            }
        }
    }

    fn flag_spot(&mut self, x: usize, y: usize) {
        let f = &mut self.field[y][x];
        if f.hidden {
            f.flag = !f.flag;
        }
    }

    fn count_field(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                self.field[j][i].n = self.count_neighbors(i, j);
            }
        }
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut n = 0;
        for i in 0..3 {
            if x+i == 0 || x+i > self.width {continue;}
            for j in 0..3 {
                if y+j == 0 || y+j > self.height {continue;}
                if i == 1 && j == 1 {continue;}
                if self.field[y+j-1][x+i-1].mine {n += 1}
            }
        }
        n
    }

    fn count_flags(&self, x: usize, y: usize) -> u8 {
        let mut n = 0;
        for i in 0..3 {
            if x+i == 0 || x+i > self.height {continue;}
            for j in 0..3 {
                if y+j == 0 || y+j > self.height {continue;}
                if i == 1 && j == 1 {continue;}
                if self.field[y+j-1][x+i-1].flag {n += 1}
            }
        }
        n
    }
}

struct Screen {
    width: isize,
    height: isize,
    spot_length: u16,
    surface: sdl::video::Surface,
}

impl Screen {
    fn new(w: isize, h: isize, l: u16) -> Screen {
        sdl::init(&[sdl::InitFlag::Video]);
        sdl::wm::set_caption("mines", "mines");

        let s = match sdl::video::set_video_mode(w, h, 32,
                                                 &[SurfaceFlag::HWSurface],
                                                 &[VideoFlag::DoubleBuf]) {
            Ok(s) => s,
            Err(err) => panic!("failed to set video mode: {}", err)
        };
        Screen {width: w, height: h, spot_length: l, surface: s}
    }

    fn draw_square(&self, x: u16, y: u16, w: u16, (r,g,b): (u8, u8, u8)) {
        self.surface.fill_rect(
            Some(sdl::Rect {x: x as i16, y: y as i16, w: w, h: w}),
            Color::RGB(r, g, b)
        );
    }

    fn draw_num(&self, n: u8, x: u16, y: u16) {
        self.draw_square(x, y, self.spot_length-1,
            (255, 255, 255)
        );
        let mut n = n;
        let sub = (self.spot_length-1)/3;
        let pack = (self.spot_length - sub*3)/2;
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
                self.draw_square(sub+x+pack, sub+y+pack, sub-1, color);
                break;
            }else if n == 2{
                self.draw_square(x+pack, y+pack, sub-1,  color);
                self.draw_square(sub*2+x+pack, sub*2+y+pack, sub-1, color);
                break;
            }else if n == 3{
                self.draw_square(sub+x+pack, sub+y+pack, sub-1, color);
                n = 2;
            }else if n == 4{
                self.draw_square(x+pack, sub*2+y+pack, sub-1, color);
                self.draw_square(sub*2+x+pack, y+pack, sub-1, color);
                n = 2;
            }else if n == 5{
                self.draw_square(sub+x+pack, sub+y+pack, sub-1, color);
                n = 4;
            }else if n == 6{
                self.draw_square(x+pack, sub+y+pack, sub-1, color);
                self.draw_square(sub*2+x+pack, sub+y+pack, sub-1, color);
                n = 4;
            }else if n == 7{
                self.draw_square(sub+x+pack, sub+y+pack, sub-1, color);
                n = 6;
            }else if n == 8{
                self.draw_square(sub+x+pack, y+pack, sub-1, color);
                self.draw_square(sub+x+pack, sub*2+y+pack, sub-1, color);
                n = 6;
            }
        }
    }

    fn draw_field(&self, ref field: &Field) {
        let length = self.spot_length;
        let mut n = 0;
        for ref i in field.field.iter() {
            let mut m = 0;
            for ref sq in i.iter() {
                if sq.hidden {
                    self.draw_square(m*length+1,
                        n*length+1,
                        length-1,
                        (180, 180, 180)
                    );
                    if sq.flag {
                        self.draw_square(m*length+4,
                            n*length+4,
                            length-7,
                            (255, 0, 0)
                        );
                    }
                }else{
                    if sq.mine {
                        self.draw_square(m*length+1,
                            n*length+1,
                            length-1,
                            (0, 0, 0)
                        );
                    }else{
                        self.draw_num(
                            sq.n,
                            m*length+1,
                            n*length+1
                        );
                    }
                }
                m += 1;
            }
            n += 1;
        }
    }
}


fn main() {
    const WIDTH: usize = 30;
    const HEIGHT: usize = 20;
    const SIZE: usize = 35;
    const R: f32 = 0.1;

    let mut field = Field::new(R, WIDTH, HEIGHT);

    let screen = Screen::new((SIZE*WIDTH) as isize + 1,
                             (SIZE*HEIGHT) as isize + 1, SIZE as u16);

    loop {
        match sdl::event::poll_event() {
            Event::Quit => break,
            Event::MouseButton(b, down, mx, my) => {
                if down {
                    if b == Mouse::Left {
                        field.show_spot(mx as usize/SIZE, my as usize/SIZE);
                    }
                    else if b == Mouse::Right {
                        field.flag_spot(mx as usize/SIZE, my as usize/SIZE);
                    }
                }
            },
            Event::Key(k, down, _, _) => {
                if down {
                    if k == Key::Escape {
                        break;
                    }
                    if k == Key::R {
                        field = Field::new(R, WIDTH, HEIGHT);
                    }
                }
            },
            _ => {}
        }
        screen.draw_field(&field);
        screen.surface.flip();
    }

    sdl::quit();
}
