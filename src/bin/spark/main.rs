pub mod commands;
fn main(){
    commands::install::install_spark();
    commands::remove::remove_spark_installation();
    commands::help::show_help();
}
