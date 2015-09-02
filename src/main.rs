extern crate sdl;
extern crate rand;

use rand::distributions::{IndependentSample, Range};
use sdl::video::{SurfaceFlag, VideoFlag, Color};
use sdl::event::{Event, Key, Mouse};

#[derive(Copy, Clone)]
struct Spot {
    //x: usize,
    //y: usize,
    hidden: bool,
    mine: bool,
    flag: bool,
    //n: i8;
}

struct Field {
    width: usize,
    height: usize,
    field: Vec<Vec<Spot>>,
}

impl Field {
    fn new(r: f32, w: usize, h: usize) -> Field {
        //TODO: make it so the field isn't rebuilt
        let bt = Range::new(0.,1.);
        let mut rng = rand::thread_rng();
        let mut field = vec![vec![Spot{hidden:false,mine:false,flag:false}; w]; h];
        for i in 0..h {
            for j in 0..w {
                if bt.ind_sample(&mut rng) < r {
                    field[i][j] = Spot{hidden: true, mine: true, flag: false};
                }else{
                    field[i][j] = Spot{hidden: true, mine: false, flag: false};
                }
            }
        }
        Field {width: w, height: h, field: field}
    }

    fn swap_mine(&mut self, x: usize, y: usize) {
        let f = self.field[x][y];
        self.field[x][y].mine = !f.mine;
    }

    fn show_spot(&mut self, x: usize, y: usize) {
        let f = self.field[x][y];
        if !f.hidden {
            for i in 0..3{
                for j in 0..3{
                    if i == 0 && j == 0 {continue;}
                    if x+i == 0 || x+i>self.width {continue;}
                    if y+j == 0 || y+j>self.height {continue;}
                    println!("{} {}", x, y);
                    self.show_spot(x+i-1, y+j-1);
                }
            }
        }
        if !f.flag {
            self.field[x][y].hidden = false;
        }
    }

    fn flag_spot(&mut self, x: usize, y: usize) {
        let f = self.field[x][y];
        if f.hidden {
            self.field[x][y].flag = !f.flag;
        }
    }

    fn count_field(self, x: usize, y: usize) -> u8 {
        let mut n = 0;
        for i in 0..3 {
            if y+i == 0 || y+i == self.height+1 {continue;}
            for j in 0..3 {
                if i == 1 && j == 1 {continue;}
                if x+j == 0 || x+j == self.width+1 {continue;}
                if self.field[y+i-1][x+j-1].mine {n += 1}
            }
        }
        n
    }
}

struct Screen<'a> {
    width: isize,
    height: isize,
    spot_length: usize,
    surface: & 'a sdl::video::Surface,
}

impl <'a>Screen<'a> {
    fn new(w: isize, h: isize, l: usize) -> Screen<'a> {
        sdl::init(&[sdl::InitFlag::Video]);
        sdl::wm::set_caption("mines", "mines");

        let s = match sdl::video::set_video_mode(w, h, 32,
                                                 &[SurfaceFlag::HWSurface],
                                                 &[VideoFlag::DoubleBuf]) {
            Ok(s) => &s,
            Err(err) => panic!("failed to set video mode: {}", err)
        };
        Screen {width: w, height: h, spot_length: l, surface: s}
    }

    fn draw_square(self, x: i16, y: i16, w: u16, (r,g,b): (u8, u8, u8)) {
        self.surface.fill_rect(Some(sdl::Rect {x: x, y: y, w: w, h: w}), Color::RGB(r, g, b));
    }

    fn draw_num(self, n: u8, x: i16, y: i16, size: u16) {
        self.draw_square(x, y, size,
            (255, 255, 255)
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
                self.draw_square(sub+x+pack, sub+y+pack, sub as u16-1, color);
                break;
            }else if n == 2{
                self.draw_square(x+pack, y+pack, sub as u16-1,  color);
                self.draw_square(sub*2+x+pack, sub*2+y+pack, sub as u16-1, color);
                break;
            }else if n == 3{
                self.draw_square(sub+x+pack, sub+y+pack, sub as u16-1, color);
                n = 2;
            }else if n == 4{
                self.draw_square(x+pack, sub*2+y+pack, sub as u16-1, color);
                self.draw_square(sub*2+x+pack, y+pack, sub as u16-1, color);
                n = 2;
            }else if n == 5{
                self.draw_square(sub+x+pack, sub+y+pack, sub as u16-1, color);
                n = 4;
            }else if n == 6{
                self.draw_square(x+pack, sub+y+pack, sub as u16-1, color);
                self.draw_square(sub*2+x+pack, sub+y+pack, sub as u16-1, color);
                n = 4;
            }else if n == 7{
                self.draw_square(sub+x+pack, sub+y+pack, sub as u16-1, color);
                n = 6;
            }else if n == 8{
                self.draw_square(sub+x+pack, y+pack, sub as u16-1, color);
                self.draw_square(sub+x+pack, sub*2+y+pack, sub as u16-1, color);
                n = 6;
            }
        }
    }
    fn draw_field(self, length: i16, field: Field) {
        let mut n = 0;
        for ref i in field.field.iter() {
            let mut m = 0;
            for ref sq in i.iter() {
                if sq.hidden {
                    self.draw_square(m*length+1,
                        n*length+1,
                        (length-1) as u16,
                        (180, 180, 180)
                    );
                    if sq.flag {
                        self.draw_square(m*length+4,
                            n*length+4,
                            (length-7) as u16,
                            (255, 0, 0)
                        );
                    }
                }else{
                    if sq.mine {
                        self.draw_square(m*length+1,
                            n*length+1,
                            (length-1) as u16,
                            (0, 0, 0)
                        );
                    }else{
                        self.draw_num(
                            field.count_field(m as usize, n as usize),
                            m as i16*length as i16+1,
                            n as i16*length as i16+1,
                            (length-1) as u16
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

    let mut field = Field::new(0.1, WIDTH, HEIGHT);

    let screen = Screen::new((SIZE*WIDTH) as isize + 1, (SIZE*HEIGHT) as isize + 1, SIZE);

    loop {
        match sdl::event::poll_event() {
            Event::Quit => break,
            Event::MouseButton(b, down, mx, my) => {
                if down {
                    if b == Mouse::Left {
                        field.show_spot(my as usize/SIZE, mx as usize/SIZE);
                    }
                    else if b == Mouse::Right {
                        field.flag_spot(my as usize/SIZE, mx as usize/SIZE);
                    }
                }
            },
            Event::Key(k, down, _, _) => {
                if down {
                    if k == Key::Escape {
                        break;
                    }
                    if k == Key::R {
                        field = Field::new(0.1, WIDTH, HEIGHT);
                    }
                }
            },
            _ => {}
        }
        screen.draw_field(SIZE as i16, field);
        screen.surface.flip();
    }

    sdl::quit();
}
