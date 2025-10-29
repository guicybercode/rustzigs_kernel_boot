use core::fmt;

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const VGA_ADDRESS: usize = 0xB8000;

#[repr(transparent)]
struct VGAColor(u8);

impl VGAColor {
    const BLACK: VGAColor = VGAColor(0);
    const BLUE: VGAColor = VGAColor(1);
    const GREEN: VGAColor = VGAColor(2);
    const CYAN: VGAColor = VGAColor(3);
    const RED: VGAColor = VGAColor(4);
    const MAGENTA: VGAColor = VGAColor(5);
    const BROWN: VGAColor = VGAColor(6);
    const LIGHT_GRAY: VGAColor = VGAColor(7);
    const DARK_GRAY: VGAColor = VGAColor(8);
    const LIGHT_BLUE: VGAColor = VGAColor(9);
    const LIGHT_GREEN: VGAColor = VGAColor(10);
    const LIGHT_CYAN: VGAColor = VGAColor(11);
    const LIGHT_RED: VGAColor = VGAColor(12);
    const PINK: VGAColor = VGAColor(13);
    const YELLOW: VGAColor = VGAColor(14);
    const WHITE: VGAColor = VGAColor(15);
}

#[repr(C)]
struct VGAEntry {
    character: u8,
    color: u8,
}

impl VGAEntry {
    fn new(character: u8, color: VGAColor) -> VGAColor {
        VGAColor(character | ((color.0 as u8) << 4))
    }
}

struct VGAWriter {
    column_position: usize,
    color_code: VGAColor,
    buffer: &'static mut [VGAEntry; VGA_WIDTH * VGA_HEIGHT],
}

impl VGAWriter {
    fn new() -> VGAWriter {
        unsafe {
            VGAWriter {
                column_position: 0,
                color_code: VGAColor::LIGHT_GRAY,
                buffer: &mut *(VGA_ADDRESS as *mut [VGAEntry; VGA_WIDTH * VGA_HEIGHT]),
            }
        }
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= VGA_WIDTH {
                    self.new_line();
                }

                let row = VGA_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code.0;
                self.buffer[row * VGA_WIDTH + col] = VGAEntry {
                    character: byte,
                    color: color_code,
                };
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let character = self.buffer[row * VGA_WIDTH + col].character;
                let color = self.buffer[row * VGA_WIDTH + col].color;
                self.buffer[(row - 1) * VGA_WIDTH + col] = VGAEntry { character, color };
            }
        }
        self.clear_row(VGA_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = VGAEntry {
            character: b' ',
            color: self.color_code.0,
        };
        for col in 0..VGA_WIDTH {
            self.buffer[row * VGA_WIDTH + col] = blank;
        }
    }
}

impl fmt::Write for VGAWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

static mut WRITER: VGAWriter = VGAWriter {
    column_position: 0,
    color_code: VGAColor::LIGHT_GRAY,
    buffer: unsafe { &mut *(VGA_ADDRESS as *mut [VGAEntry; VGA_WIDTH * VGA_HEIGHT]) },
};

pub fn init() {
    unsafe {
        for i in 0..VGA_WIDTH * VGA_HEIGHT {
            WRITER.buffer[i] = VGAEntry {
                character: b' ',
                color: VGAColor::BLACK.0,
            };
        }
    }
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        WRITER.write_fmt(args).unwrap();
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
