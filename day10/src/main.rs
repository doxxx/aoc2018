use lazy_static::lazy_static;
use piston_window::*;
use regex::Regex;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let points = read_input()?;

    show_points(&points);

    Ok(())
}

fn read_input() -> Result<Vec<Point>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input.lines().map(parse_point).collect())
}

fn parse_point(s: &str) -> Point {
    lazy_static! {
        static ref re: Regex =
            Regex::new(r"position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>").unwrap();
    }

    let caps = re.captures(s).unwrap();

    Point {
        position: Pair {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
        },
        velocity: Pair {
            x: caps[3].parse().unwrap(),
            y: caps[4].parse().unwrap(),
        },
    }
}

#[derive(Debug, Clone)]
struct Point {
    position: Pair,
    velocity: Pair,
}

#[derive(Debug, Clone)]
struct Pair {
    x: i32,
    y: i32,
}

fn show_points(points: &[Point]) {
    let mut points = Vec::from(points);
    let width = 400;
    let height = 400;
    let mut seconds = 0;

    while points_out_of_bounds(&points, width, height) {
        move_points(&mut points, true);
        seconds += 1;
    }

    println!("Points advanced to within a {}x{} block.", width, height);
    println!("Press Left/Right to move points backwards/forwards in time until the text resolves.");

    let mut window: PistonWindow = WindowSettings::new(
        format!("Advent of Code 2018 - Day 10: T={}", seconds),
        [width as u32, height as u32],
    )
    .resizable(false)
    .build()
    .unwrap();

    let mut canvas = img::ImageBuffer::new(width as u32, height as u32);

    let mut tex =
        Texture::from_image(&mut window.factory, &canvas, &TextureSettings::new()).unwrap();

    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::Space) | Button::Keyboard(Key::Right) => {
                    move_points(&mut points, true);
                    seconds += 1;
                }
                Button::Keyboard(Key::Backspace) | Button::Keyboard(Key::Left) => {
                    move_points(&mut points, false);
                    seconds -= 1;
                }
                _ => {}
            }
            window.set_title(format!("Advent of Code 2018 - Day 10: T={}", seconds));
        }
        if let Some(_) = e.render_args() {
            draw_points(&mut canvas, &points);
            tex.update(&mut window.encoder, &canvas).unwrap();
            window.draw_2d(&e, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                image(&tex, c.transform, g);
            });
        }
    }
}

fn points_out_of_bounds(points: &[Point], width: i32, height: i32) -> bool {
    points.iter().any(|p| {
        p.position.x < 0 || p.position.x >= width || p.position.y < 0 || p.position.y >= height
    })
}

fn draw_points(canvas: &mut img::ImageBuffer<img::Rgba<u8>, Vec<u8>>, points: &[Point]) {
    let black = img::Rgba([0, 0, 0, 255]);
    let white = img::Rgba([255, 255, 255, 255]);

    canvas.pixels_mut().for_each(|p| *p = black);

    for point in points {
        if point.position.x >= 0
            && (point.position.x as u32) < canvas.width()
            && point.position.y >= 0
            && (point.position.y as u32) < canvas.height()
        {
            canvas.put_pixel(point.position.x as u32, point.position.y as u32, white);
        }
    }
}

fn move_points(points: &mut [Point], forward: bool) {
    let sign = if forward { 1 } else { -1 };
    points.iter_mut().for_each(|p| {
        p.position.x += p.velocity.x * sign;
        p.position.y += p.velocity.y * sign;
    });
}
