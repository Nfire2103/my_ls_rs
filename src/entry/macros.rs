macro_rules! major {
    ($val:expr) => {
        $val >> 8
    };
}

macro_rules! minor {
    ($val:expr) => {
        $val & 0xff
    };
}

macro_rules! printlnif {
    ($val:expr) => {
        if $val {
            println!();
        }
    };
}
