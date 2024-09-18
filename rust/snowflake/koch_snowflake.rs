use std::f64::consts::PI;

fn draw_line(x1: f64, y1: f64, x2: f64, y2: f64, depth:  usize) {
    if depth == 0 {
        println!("Line from ({:.2}, {:.2} to {:.2}, {:.2})", x1, y1, x2, y2);
    } else {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let x3 = x1 + dx / 3.0;
        let y3 = y1 + dy / 3.0;
        let x5 = x1 + 2.0 * dx / 3.0;
        let y5 = y1 + 2.0 * dy / 3.0;

        let angle = PI / 3.0;
        let x4 = (x3 + x5) / 2.0 - (y5 - y3) * angle.sin();
        let y4 = (y3 + y5) / 2.0 + (x5 - x3) * angle.sin();

        draw_line(x1, y1, x3, y3, depth -1);
        draw_line(x3, y3, x4, y4, depth -1);
        draw_line(x4, y4, x5, y5, depth -1);
        draw_line(x5, y5, x2, y2, depth -1);
    }
}

fn koch_snowflake(depth: usize) {
    let size = 200.0;
    let x1 = 100.0;
    let y1 = 100.0;
    let x2 = x1 + size * (PI / 3.0).cos();
    let y2 = y1 + size * (PI / 3.0).sin();
    let x3 = x1 + size * (2.0 * PI / 3.0).cos();
    let y3 = y1 + size * (2.0 * PI / 3.0).sin();

    draw_line(x1, y1, x2, y2, depth);
    draw_line(x2, y2, x3, y3, depth);
    draw_line(x3, y3, x1, y1, depth);
}

fn main() {
    let depth = 3;
    koch_snowflake(depth);
}
