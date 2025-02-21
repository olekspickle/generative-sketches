use nannou::prelude::*;
// use rand;

const SCALE: f32 = 0.35;
const SAMPLES: i32 = 500;
const ROTATION: f32 = 1.;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    // Clear the background to blue.
    draw.background().color(CORNFLOWERBLUE);

    // Get window as Rect.
    let win = app.window_rect();
    // Get time as float.
    let t = app.time;

    // // Draw a quad that follows the inverse of the ellipse.
    // draw.quad()
    //     // .x_y(app.mouse.x * t.cos() * PI, app.mouse.y * t.sin())
    //     .x_y(app.mouse.x, app.mouse.y)
    //     .w_h(QUAD_W, QUAD_W)
    //     // .rotate(t)
    //     .color(CORNFLOWERBLUE)
    //     .stroke_weight(5.)
    //     .stroke_color(MEDIUMSPRINGGREEN);

    for i in -SAMPLES..SAMPLES {
        // TODO: normalize for any sample quantity to be uniform
        // let norm = i.normalize(-win.w() as i32 / 2, win.w() as i32 / 2) as f32;
        let norm = i as f32;
        draw.line().weight(2.).color(MEDIUMSPRINGGREEN).points(
            Point2::new(app.mouse.x, app.mouse.y),
            Point2::new(
                app.mouse.x + norm * SCALE * rand::random::<f32>() + ROTATION * t.sin(),
                app.mouse.y + norm * SCALE * rand::random::<f32>() + ROTATION * t.cos(),
            ),
        );
        // I didnt come up with some clever formula to make it one draw call per cycle run
        // But two draw call dont seem to be an issue yet.
        draw.line().weight(2.).color(MEDIUMSPRINGGREEN).points(
            Point2::new(app.mouse.x, app.mouse.y),
            Point2::new(
                app.mouse.x + norm * SCALE * rand::random::<f32>() + ROTATION * t.sin(),
                app.mouse.y - norm * SCALE * rand::random::<f32>() + ROTATION * t.cos(),
            ),
        );
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

pub trait Normalize<N> {
    fn normalize(&self, n1: N, n2: N) -> N;
}

impl Normalize<i32> for i32 {
    fn normalize(&self, n1: i32, n2: i32) -> i32 {
        if *self == 0 {
            *self
        } else {
            let n = n1 + (*self as f32 * (*self as f32 / (n2 as f32 * SCALE))) as i32;
            let n = n * self / self; //return the sign
            println!("{:?} n:{}", self, n);
            n
        }
    }
}
