use clap::Parser;

#[derive(Parser)]
pub struct Arguments{
    /// input data file
    pub filename: String,

    /// Define figure title
    #[arg(long,short,default_value_t=String::from(""))]
    pub title: String,

    /// x-axis resolution
    #[arg(long,short,default_value_t=80)]
    pub xres: i32,

    /// y-axis resolution
    #[arg(long,short,default_value_t=30)]
    pub yres: i32,   
    
    /// number of ticks on x-axis
    #[arg(long,default_value_t=2)]
    pub xticks: i32,

    /// number of ticks on y-axis
    #[arg(long,default_value_t=2)]
    pub yticks: i32,   
}