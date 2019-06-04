pub mod libinfo {
    use cfg_if::cfg_if;
    // enable logging only during debug builds 
    cfg_if! {
        if #[cfg(feature = "wee_alloc")] {
            pub fn hello_text() -> &'static str {
                "Yes, feature is enabled (in my_lib)!"
            }
        } else {
            pub fn hello_text() -> &'static str {
                "Nope, feature is disabled (in my_lib)!"
            }
        }
    }
}
