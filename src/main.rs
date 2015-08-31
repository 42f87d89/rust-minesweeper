extern crate sdl;
extern crate rand;

//use rand::Rng;
use sdl::video::{SurfaceFlag, VideoFlag, Color};
use sdl::event::{Event, Key};

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

fn draw_field(ref field: &Vec<Vec<bool>>, length: i16, screen: &sdl::video::Surface) {
    let mut n = 0;
    for ref i in field.iter() {
        let mut m = 0;
        for ref j in i.iter() {
            draw_square(m*length+1,
                      n*length+1,
                      (length-1) as u16,
                      if **j {(0, 0, 0)} else {(255, 255, 255)},
                      screen
            );
            if !**j{
                draw_num(
                    count_field(m as usize, n as usize, field),
                    m as i16*length as i16+1,
                    n as i16*length as i16+1,
                    (length-1) as u16,
                    screen);
            }
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

//fn random_field(r: f32, field: &mut Vec<Vec<bool>>) {
    //field.iter().map(|x| {x.map(|y| {
        
        /*let rn = rand::random::<u32>();
        if rn%1 > r {
            true
        }else{
            false
        }*/
        
    //})});
    //let rng = rand::thread_rng();
//}

fn count_field(x: usize, y: usize, field: &Vec<Vec<bool>>) -> u8 {
    let w = field[0].len();
    let h = field.len();
    let mut n = 0;
    for i in 0..3 {
        if y+i == 0 || y+i == h+1 {continue;}
        for j in 0..3 {
            if i == 1 && j == 1 {continue;}
            if x+j == 0 || x+j == w+1 {continue;}
            if field[y+i-1][x+j-1] {n += 1}
        }
    }
    n
}

fn main() {
    const WIDTH: usize = 30;
    const HEIGHT: usize = 20;
    const SIZE: usize = 35;

    let field = &mut vec![vec![false; WIDTH]; HEIGHT];

    let screen = init((SIZE*WIDTH) as isize + 1, (SIZE*HEIGHT) as isize + 1);
    loop {
        match sdl::event::poll_event() {
            Event::Quit => break,
            Event::MouseButton(_, down, mx, my) => {
                if down {
                    flip_field(field, my as i16, mx as i16, SIZE as i16);
                    println!("{}", count_field((mx/SIZE as u16) as usize, (my/SIZE as u16) as usize, field));
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
