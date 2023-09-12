pub struct Canvas{
    pub title: String,
    pub xlabel: String,
    pub ylabel: String,
    xmin: f32,
    xmax: f32,
    ymin: f32,
    ymax: f32,
    xres: usize,
    yres: usize,
    xticks: i32,
    yticks: i32,
    fit: bool
}

impl Canvas {
    pub fn new(xr:i32, yr:i32, xt:i32, yt:i32) -> Canvas {
        Canvas { 
            title:  String::from(""),
            xlabel: String::from("x"),
            ylabel: String::from("y"),
            xmin: 0.0,
            xmax: 10.0,
            ymin: 0.0,
            ymax: 10.0,
            xres: xr as usize,
            yres: yr as usize,
            xticks: xt,
            yticks: yt,
            fit: false,
        }
    }

    pub fn draw(&self, data_x: &Vec<f32>, data_y: &Vec<f32>){
        println!("{:^1$}",self.title,self.xres+8);

        let y_step = (self.ymax - self.ymin)/self.yres as f32;
        let x_step = (self.xmax -self.xmin)/self.xres as f32;
        let mut grid_raw = vec![0; self.xres*self.yres];
        let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(self.xres).collect();
        let grid = grid_base.as_mut_slice();


        for j in 0..data_x.len(){
            let x_index: usize = ((data_x[j]-self.xmin)/x_step) as usize;
            let y_index: usize = ((data_y[j]-self.ymin)/y_step) as usize;
            grid[self.yres - y_index - 1][x_index] = 1;
            
        }

        // drawing figure and Y-axis
        for i in 0..self.yres{
            let mut line = String::new();
            for j in 0..self.xres{
                if grid[i][j] == 1{
                    line.push_str("o");
                }else{
                    line.push_str(" ");
                }
            }
            if i == 0 || (i%(self.yres/self.yticks as usize) == 0){
                println!("{:>7} ─{}",((self.yres - i) as f32)*y_step,line)
            }else if i == self.yres - 1 {
                println!("{:>7} ─{}",self.ymin,line) 
            }
            else {
                println!("{:>7} │{}","",line)
            }   
        }

        // X-axis definition
        let mut xaxis = String::new();
        xaxis.push_str(&format!("{:>9}","└").to_string());
        for i in 0..self.xres{
            if i == self.xres - 1 || (i%(self.xres/self.xticks as usize) == 0){
                xaxis.push_str("│")
            }else{
                xaxis.push_str("─")
            }
        }
        println!("{}",xaxis);


        // X-axis labels
        let mut xticks = String::new();
        for i in 0..self.xres{
            if i == self.xres -1 {
                xticks.push_str(&format!("{:>6}{}","",self.xmax).to_string())
            } else if i == self.xres - 1 || (i%(self.xres/self.xticks as usize) == 0){
                xticks.push_str(&format!("{:>7} {} {:>width$}","",i as f32 *x_step,"",width=(self.xres/self.xticks as usize) - 10 as usize).to_string())
            }
        }
        println!("{}",xticks);

    }


}

