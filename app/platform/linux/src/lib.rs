use domain::pal::Console;

pub struct LinuxConsole;

impl Console for LinuxConsole {
    fn putc(c: u8) {
        print!("{}", c as char);
    }
}
