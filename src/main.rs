use pad::{PadStr, Alignment};
fn main() {
    const CANVAS_X: usize = 10;
    const CANVAS_Y: usize = 10;
    let mut canvas: [[usize; CANVAS_Y]; CANVAS_X] = [[0; CANVAS_Y]; CANVAS_X];
    let mut title: String = String::new();
    let mut i: usize = 0;

    let mut y_max:f32 = 10.0;
    let mut y_min:f32 = 0.0;

    let mut x_max:f32 = 10.0;
    let mut x_min:f32 = 0.0;


    title += "this is my title";

    canvas[2][2] = 1;
    canvas[2][3] = 1;
    canvas[2][4] = 1;
    canvas[5][5] = 1;

    println!("{}",title.pad_to_width_with_alignment(16, Alignment::Middle));
    println!("{:.1}",y_max);
    while i < (CANVAS_Y) {
        let mut raster_line: String = String::new();
        if (i == 0) || (i%5 == 0){
            raster_line.push_str(&format!("{:>5}","-").to_string());
        } else {
            raster_line.push_str(&format!("{:>5}","|").to_string());
        }

        for c in canvas[i].iter(){
            if c == &0 {
                raster_line.push(' ')
            } else {
                raster_line.push('o')
            }
        };
        println!("{}",raster_line);
        i += 1;
    };
    let mut raster_line: String = String::new();
    raster_line.push_str(&format!("{:>4}",y_min).to_string());
    for i in 0..CANVAS_X {
        if canvas[CANVAS_Y - 1][i] == (1 as usize) {
            raster_line.push('o')
        }else if (i == 0) || (i == CANVAS_X) {
            raster_line.push('|');
        }else {
            raster_line.push('-');
        }
    };
    println!("{}",raster_line);
    println!("{:>5}        {:>5}",x_min,x_max);



}
