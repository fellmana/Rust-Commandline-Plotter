use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn read_data_from_file(filepath: &str,sep: &str)-> (Vec<f32>, Vec<f32>){
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let mut x = Vec::new();
    let mut y = Vec::new();

    let mut loop_index:i32 = 0;

    for line in reader.lines(){
        let s = line.unwrap();
        let parts = s.split(sep);
        let collection = parts.collect::<Vec<&str>>();

        // handle headerline in data 
        if loop_index == 0{
            if collection[0] == "#"{
                loop_index += 1;
                continue;
            }
        }
        x.push(collection[0].parse::<f32>().unwrap());
        y.push(collection[1].parse::<f32>().unwrap());
        loop_index += 1;
    }
    //println!("{:?}",x);
    //println!("{:?}",y);
    return (x,y);
}