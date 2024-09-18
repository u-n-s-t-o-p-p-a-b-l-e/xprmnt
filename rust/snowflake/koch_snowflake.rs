use std::f64::consts::PI;

fn draw_line(x1: f64, y1: f64, x2: f64, y2: f64, depth:  usize) {
    if depth == 0 {
        println!("Line from ({:.2}, {:.2} to {:.2}, {:.2})", x1, y1, x2, y2);
    } else {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let x3 = x1 + dx / 3.0;
        ley y3 = y1 + dy / 3.0;
        let x5 = x1 + 2.0 * dx / 3.0;
        let y5 = y1 + 2.0 * dy / 3.0;

        let angle = PI / 3.0;
        let x4 = (x3 + x5) / 2.0 - (y5 - y3) * angle.sin();
        let y4 = (y3 + y5) / 2.0 + (x5 - x3) * angle.sin();
    }
}
