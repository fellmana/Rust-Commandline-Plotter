#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
mod canvas;
mod fileio;
mod commandline;
use clap::Parser;
fn main() {
    let args: commandline::Arguments = commandline::Arguments::parse();
    let canvas = canvas::Canvas::new(args.xres,args.yres,args.xticks,args.yticks);
    let data:(Vec<f32>, Vec<f32>) = fileio::read_data_from_file(&*args.filename," ");
    let vecx: Vec<f32> = data.0;
    let vecy: Vec<f32> = data.1;

    //let vecx: Vec<f32> = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    //let vecy: Vec<f32> = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    canvas.draw(&vecx, &vecy);
}
