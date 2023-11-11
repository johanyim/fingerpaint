pub mod palette;
pub mod color;
pub mod ui;
pub mod cl_args;
pub mod config {
    #[derive(serde::Deserialize)]
    pub struct Config {
        pub path: String,
        pub selected: String,
        pub auto_type: bool,
        pub close_on_select: bool,
        pub copy_to_clipboard: bool,
    }
}



