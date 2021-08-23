use num::Complex;
use macroquad::prelude::*;


#[macroquad::main("baseIminus1")]
async fn main() {
    let mut pts : Vec<Complex<f32>> =Vec::with_capacity(2_000_000);
    pts.push(Complex::new(0.0, 0.0));
    let mut current_power = Complex::new(1.0,0.0);
    let base = Complex::new(-1.0,1.0);
    let mut pow = 1;
    //let mut colors : Vec<Color> = vec![rand_color()];
 
    let mut cam = Camera2D::from_display_rect(Rect{x:-50.0*aspect_ratio(),y:-50.0,w:100.0*aspect_ratio(),h:100.0});
    set_camera(&cam);
    loop {
        clear_background(BLACK);
        draw_line(-1000.0, 0.0, 1000.0, 0.0, 0.05, WHITE);
        draw_line(0.0, -1000.0, 0.0, 1000.0, 0.05, WHITE);
        for (i,z) in (&pts).into_iter().enumerate() {
            let color_idx = (i as f64).log(2.0) as usize;
            draw_rectangle(z.re - 0.5, -1.0*z.im - 0.5, 1.0, 1.0, golden_color(color_idx))
        }

        if is_mouse_button_released(MouseButton::Left) {
            for j in 0..pow {
                pts.push(current_power + pts[j]);
            }
            pow *= 2;
            current_power *= base;
            println!("{}", pts.len());
        }
        let (_,wheel) = mouse_wheel();
        if wheel > 0.0 {
            cam.zoom *= Vec2::splat(1.1);
            set_camera(&cam);
        }
        else if wheel < 0.0 {
            cam.zoom *= Vec2::splat(1.0/1.1);
            set_camera(&cam);
        }

        next_frame().await
    }
}


pub fn aspect_ratio() -> f32 {
    screen_width()/screen_height()
}

pub fn rand_color() -> Color {
    hsv_to_rgb((rand::rand() as f32)/(std::u32::MAX as f32) ,0.8,1.0)
}

pub fn hsv_to_rgb(h:f32,s:f32,v:f32) -> Color {
    let c = v*s;
    let x = c*(1.0 - ((h*6.0)%2.0 - 1.0).abs());
    let m = v - c;
    let (rp,gp,bp) = if h < 1.0/6.0 {
        (c,x,0.0)
    }
    else if h < 2.0/6.0 {
        (x,c,0.0)
    }
    else if h < 3.0/6.0 {
        (0.0,c,x)
    }
    else if h < 4.0/6.0 {
        (0.0,x,c)
    }
    else if h < 5.0/6.0 {
        (x,0.0,c)
    }
    else {
        (c,0.0,x)
    };
    let (r,g,b) = ((rp + m)*255.0, (gp + m)*255.0, (bp + m)*255.0);
    Color::from_rgba(r as u8, g as u8, b as u8, 255)
} 

pub fn golden_color(level : usize) -> Color {
    let phi_inverse = 2.0/(1.0 + f32::sqrt(5.0));
    let h = ((level as f32)*phi_inverse)%1.0;
    hsv_to_rgb(h, 0.8, 1.0)
}