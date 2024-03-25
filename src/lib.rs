// &'static is a "lifetime specifier", something you'll learn more about late

pub fn hello() -> &'static str {
    "Hello, World!"
}

