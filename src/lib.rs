#![no_std]

use embedded_hal::blocking::i2c::{Write, WriteRead};
use ht16k33::{DisplayData, DisplayDataAddress, LedLocation, HT16K33};

/// The default address for the display.
pub const DEFAULT_DISPLAY_ADDR: u8 = 0x70;
/// The default address for the display.
pub const DISPLAY_ADDR_1: u8 = 0x71;
/// The default address for the display.
pub const DISPLAY_ADDR_2: u8 = 0x72;
/// The default address for the display.
pub const DISPLAY_ADDR_3: u8 = 0x73;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Segment(u8);

const SEGMENT_A: Segment = Segment(0);
const SEGMENT_B: Segment = Segment(1);
const SEGMENT_C: Segment = Segment(2);
const SEGMENT_D: Segment = Segment(3);
const SEGMENT_E: Segment = Segment(4);
const SEGMENT_F: Segment = Segment(5);
const SEGMENT_G: Segment = Segment(6);
const SEGMENT_H: Segment = Segment(7);
const SEGMENT_I: Segment = Segment(8);
const SEGMENT_J: Segment = Segment(9);
const SEGMENT_K: Segment = Segment(10);
const SEGMENT_L: Segment = Segment(11);
const SEGMENT_M: Segment = Segment(12);
const SEGMENT_N: Segment = Segment(13);

fn illuminate_char<I2C, E>(display: &mut HT16K33<I2C>, segments_to_turn_on: u16, digit: u8)
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    for i in 0..14 {
        illuminate_segment(
            display,
            Segment(i),
            digit,
            ((segments_to_turn_on >> i) & 0b1) == 0b1,
        ); // Convert the segment number to a letter
    }
}

fn illuminate_segment<I2C, E>(display: &mut HT16K33<I2C>, segment: Segment, digit: u8, on: bool)
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    let mut com = segment.0; // Convert the segment letter back to a number

    if com > 6 {
        com -= 7;
    }
    if segment == SEGMENT_I {
        com = 0;
    }
    if segment == SEGMENT_H {
        com = 1;
    }

    // Convert digit (1 to 16) back to a relative position on a given digit on display
    let mut row = digit % 4;
    if segment > SEGMENT_G {
        row += 4;
    }

    let offset = digit / 4 * 16;
    let mut adr = com * 2 + offset;

    // Determine the address
    if row > 7 {
        adr += 1;
    }

    // Determine the data bit
    if row > 7 {
        row -= 8;
    }
    let dat = 1 << row;

    let row = DisplayDataAddress::from_bits_truncate(adr);
    let common = DisplayData::from_bits_truncate(dat);
    display.update_display_buffer(LedLocation { row, common }, on);
}

const UNKNOWN_CHAR: u16 = 95;
pub const FONT: [u16; 96] = [
    //NMLKJIHGFEDCBA
    0b00000000000000, // ' ' (space)
    0b00001000001000, // '!'
    0b00001000000010, // '"'
    0b01001101001110, // '#'
    0b01001101101101, // '$'
    0b10010000100100, // '%'
    0b00110011011001, // '&'
    0b00001000000000, // '''
    0b00000000111001, // '('
    0b00000000001111, // ')'
    0b11111010000000, // '*'
    0b01001101000000, // '+'
    0b10000000000000, // ','
    0b00000101000000, // '-'
    0b00000000000000, // '.'
    0b10010000000000, // '/'
    0b00000000111111, // '0'
    0b00010000000110, // '1'
    0b00000101011011, // '2'
    0b00000101001111, // '3'
    0b00000101100110, // '4'
    0b00000101101101, // '5'
    0b00000101111101, // '6'
    0b01010000000001, // '7'
    0b00000101111111, // '8'
    0b00000101100111, // '9'
    0b00000000000000, // ':'
    0b10001000000000, // ';'
    0b00110000000000, // '<'
    0b00000101001000, // '='
    0b01000010000000, // '>'
    0b01000100000011, // '?'
    0b00001100111011, // '@'
    0b00000101110111, // 'A'
    0b01001100001111, // 'B'
    0b00000000111001, // 'C'
    0b01001000001111, // 'D'
    0b00000101111001, // 'E'
    0b00000101110001, // 'F'
    0b00000100111101, // 'G'
    0b00000101110110, // 'H'
    0b01001000001001, // 'I'
    0b00000000011110, // 'J'
    0b00110001110000, // 'K'
    0b00000000111000, // 'L'
    0b00010010110110, // 'M'
    0b00100010110110, // 'N'
    0b00000000111111, // 'O'
    0b00000101110011, // 'P'
    0b00100000111111, // 'Q'
    0b00100101110011, // 'R'
    0b00000110001101, // 'S'
    0b01001000000001, // 'T'
    0b00000000111110, // 'U'
    0b10010000110000, // 'V'
    0b10100000110110, // 'W'
    0b10110010000000, // 'X'
    0b01010010000000, // 'Y'
    0b10010000001001, // 'Z'
    0b00000000111001, // '['
    0b00100010000000, // '\'
    0b00000000001111, // ']'
    0b10100000000000, // '^'
    0b00000000001000, // '_'
    0b00000010000000, // '`'
    0b00000101011111, // 'a'
    0b00100001111000, // 'b'
    0b00000101011000, // 'c'
    0b10000100001110, // 'd'
    0b00000001111001, // 'e'
    0b00000001110001, // 'f'
    0b00000110001111, // 'g'
    0b00000101110100, // 'h'
    0b01000000000000, // 'i'
    0b00000000001110, // 'j'
    0b01111000000000, // 'k'
    0b01001000000000, // 'l'
    0b01000101010100, // 'm'
    0b00100001010000, // 'n'
    0b00000101011100, // 'o'
    0b00010001110001, // 'p'
    0b00100101100011, // 'q'
    0b00000001010000, // 'r'
    0b00000110001101, // 's'
    0b00000001111000, // 't'
    0b00000000011100, // 'u'
    0b10000000010000, // 'v'
    0b10100000010100, // 'w'
    0b10110010000000, // 'x'
    0b00001100001110, // 'y'
    0b10010000001001, // 'z'
    0b10000011001001, // '{'
    0b01001000000000, // '|'
    0b00110100001001, // '}'
    0b00000101010010, // '~'
    0b11111111111111, // Unknown
];

/// Manage multiple HT16K33 objects over a convenient interface.
pub struct AlphaNum4<'a, I2C, const CONTENT_LEN: usize = 4, const NUM_DISPLAYS: usize = 1> {
    displays: [HT16K33<I2C>; NUM_DISPLAYS],
    display_content: [char; CONTENT_LEN],
    font: &'a [u16],
}

impl<I2C, const CONTENT_LEN: usize, const NUM_DISPLAYS: usize>
    AlphaNum4<'static, I2C, CONTENT_LEN, NUM_DISPLAYS>
{
    /// Construct a new `AlphaNum4` from `NUM_DISPLAYS` `HT16K33` instances.
    /// Uses a default latin ascii font.
    pub fn new(displays: [HT16K33<I2C>; NUM_DISPLAYS]) -> Self {
        Self {
            displays,
            display_content: [' '; CONTENT_LEN],
            font: &FONT[..],
        }
    }
}

impl<'a, I2C, const CONTENT_LEN: usize, const NUM_DISPLAYS: usize>
    AlphaNum4<'a, I2C, CONTENT_LEN, NUM_DISPLAYS>
{
    /// Construct a new `AlphaNum4` from `NUM_DISPLAYS` `HT16K33` instances and a custom font.
    pub fn new_with_font(displays: [HT16K33<I2C>; NUM_DISPLAYS], font: &'a [u16]) -> Self {
        Self {
            displays,
            display_content: [' '; CONTENT_LEN],
            font,
        }
    }

    /// Get a shared reference to the `idx`th display.
    pub fn display_at(&self, idx: usize) -> &HT16K33<I2C> {
        &self.displays[idx]
    }

    /// Get a unique reference to the `idx`th display.
    pub fn display_at_mut(&mut self, idx: usize) -> &mut HT16K33<I2C> {
        &mut self.displays[idx]
    }

    /// Get a shared slice to all of the displays.
    pub fn displays(&self) -> &[HT16K33<I2C>] {
        &self.displays
    }

    /// Get a unique slice to all of the displays.
    pub fn displays_mut(&mut self) -> &mut [HT16K33<I2C>] {
        &mut self.displays
    }
}

impl<'a, I2C, const CONTENT_LEN: usize> AlphaNum4<'a, I2C, CONTENT_LEN, 1> {
    /// Get a shared reference to the inner `HT16K33` instance.
    pub fn display(&self) -> &HT16K33<I2C> {
        &self.displays[0]
    }

    /// Get a unique reference to the inner `HT16K33` instance.
    pub fn display_mut(&mut self) -> &mut HT16K33<I2C> {
        &mut self.displays[0]
    }
}

fn print_char_at<I2C, E, const NUM_DISPLAYS: usize>(
    displays: &mut [HT16K33<I2C>; NUM_DISPLAYS],
    ch: char,
    index: u8,
    font: &[u16],
) where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    // moved alphanumeric_segs array to PROGMEM
    let mut char_pos: Option<u16> = None;

    // space
    if ch == ' ' {
        char_pos = Some(0);
    }
    // Printable Symbols -- Between first character ! and last character ~
    else if ('!'..='~').contains(&ch) {
        char_pos = Some((ch as u8 - b'!' + 1) as u16);
    }

    let display_index = (index / 4) as usize;

    // Take care of special characters by turning correct segment on
    // TODO check how to handle index counts for special segments?
    let char_pos = match char_pos {
        Some(14) => {
            // '.'
            show_dot(&mut displays[display_index], true);
            return;
        }
        Some(26) => {
            // ':'
            show_colon(&mut displays[display_index], true);
            return;
        }
        Some(c) => c,
        None => UNKNOWN_CHAR,
    };

    let segments_to_turn_on = font[char_pos as usize];

    illuminate_char(&mut displays[display_index], segments_to_turn_on, index % 4);
}

/// Illuminate the colon LED on `display` if `enable` is true, otherwise disable it.
pub fn show_colon<I2C, E>(display: &mut HT16K33<I2C>, enable: bool)
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    let adr = 0x01;
    let dat = 1;
    let location = LedLocation {
        row: DisplayDataAddress::from_bits_truncate(adr),
        common: DisplayData::from_bits_truncate(dat),
    };
    display.update_display_buffer(location, enable);
}

/// Illuminate the dot LED on `display` if `enable` is true, otherwise disable it.
pub fn show_dot<I2C, E>(display: &mut HT16K33<I2C>, enable: bool)
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    let adr = 0x01;
    let dat = 1;
    let location = LedLocation {
        row: DisplayDataAddress::from_bits_truncate(adr),
        common: DisplayData::from_bits_truncate(dat),
    };
    display.update_display_buffer(location, enable);
}

impl<'a, I2C, E, const CONTENT_LEN: usize, const NUM_DISPLAYS: usize>
    AlphaNum4<'a, I2C, CONTENT_LEN, NUM_DISPLAYS>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Store the first `CONTENT_LEN` chars in the string. Then display the
    /// first `4 * NUM_DISPLAYS` characters to the display(s).
    pub fn print_str(&mut self, s: &str) {
        for (n, o) in self.display_content.iter_mut().zip(s.chars()) {
            *n = o;
        }
        // Clear the rest of the content
        for n in self.display_content.iter_mut().skip(s.len()) {
            *n = ' ';
        }
        self.print_content()
    }

    /// Print the char `ch` at position `index`.
    pub fn print_char(&mut self, ch: char, index: impl Into<u8>) {
        let index = index.into();
        self.display_content[index as usize] = ch;
        print_char_at(&mut self.displays, ch, index, self.font);
    }

    fn print_content(&mut self) {
        for (i, char) in self
            .display_content
            .iter()
            .take(4 * NUM_DISPLAYS)
            .enumerate()
        {
            print_char_at(&mut self.displays, *char, i as u8, self.font);
        }
    }

    /// Rotate the stored display content `shift_amount` characters to the left.
    /// The first `shift_amount` characters in the display content will be
    /// appended to the back of the display content. Then display the first
    /// `4 * NUM_DISPLAYS` characters to the display(s).
    ///
    /// ## Example
    /// ```no_run
    /// let display = AlphaNum4::new([disp1, disp2]);
    /// display.print_str("Hi Mom!!");
    /// // Display contents: |Hi M| |om!!|
    /// display.rotate_left(3);
    /// // Display contents: |Mom!| |!Hi |
    /// ```
    pub fn rotate_left(&mut self, shift_amount: usize) {
        let mut new_display_content = [' '; CONTENT_LEN];
        for (n, o) in new_display_content
            .iter_mut()
            .zip(self.display_content.iter().skip(shift_amount))
        {
            *n = *o;
        }

        for (n, o) in new_display_content
            .iter_mut()
            .skip(CONTENT_LEN - shift_amount)
            .zip(self.display_content.iter())
        {
            *n = *o;
        }

        self.display_content = new_display_content;
        self.print_content();
    }

    /// Rotate the stored display content `shift_amount` characters to the
    /// right. The last `shift_amount` characters in the display content will be
    /// prepended to the front of the display content. Then display the first
    /// `4 * NUM_DISPLAYS` characters to the display(s).
    ///
    /// ## Example
    /// ```no_run
    /// let display = AlphaNum4::new([disp1, disp2]);
    /// display.print_str("Hi Mom!!");
    /// // Display contents: |Hi M| |om!!|
    /// display.rotate_right(3);
    /// // Display contents: |m!!H| |i Mo|
    /// ```
    pub fn rotate_right(&mut self, shift_amount: usize) {
        let mut new_display_content = [' '; CONTENT_LEN];
        for (n, o) in new_display_content
            .iter_mut()
            .skip(shift_amount)
            .zip(self.display_content.iter())
        {
            *n = *o;
        }

        for (n, o) in new_display_content
            .iter_mut()
            .take(shift_amount)
            .zip(self.display_content.iter().skip(CONTENT_LEN - shift_amount))
        {
            *n = *o;
        }

        self.display_content = new_display_content;
        self.print_content();
    }

    /// Shift the stored display content `shift_amount` characters to the left.
    /// Then display the first `4 * NUM_DISPLAYS` characters to the display(s).
    ///
    /// ## Example
    /// ```no_run
    /// let display = AlphaNum4::new([disp1, disp2]);
    /// display.print_str("Hi Mom!!");
    /// // Display contents: |Hi M| |om!!|
    /// display.shift_left(3);
    /// // Display contents: |Mom!| |!   |
    /// ```
    pub fn shift_left(&mut self, shift_amount: usize) {
        let mut new_display_content = [' '; CONTENT_LEN];
        for (n, o) in new_display_content
            .iter_mut()
            .zip(self.display_content.iter().skip(shift_amount))
        {
            *n = *o;
        }
        for n in new_display_content
            .iter_mut()
            .skip(CONTENT_LEN - shift_amount)
        {
            *n = ' ';
        }
        self.display_content = new_display_content;
        self.print_content();
    }

    /// Shift the stored display content `shift_amount` characters to the left.
    /// Then display the first `4 * NUM_DISPLAYS` characters to the display(s).
    ///
    /// ## Example
    /// ```no_run
    /// let display = AlphaNum4::new([disp1, disp2]);
    /// display.print_str("Hi Mom!!");
    /// // Display contents: |Hi M| |om!!|
    /// display.shift_right(3);
    /// // Display contents: |   H| |i Mo|
    /// ```
    pub fn shift_right(&mut self, shift_amount: usize) {
        let mut new_display_content = [' '; CONTENT_LEN];
        for (n, o) in new_display_content
            .iter_mut()
            .skip(shift_amount)
            .zip(self.display_content.iter())
        {
            *n = *o;
        }
        for n in new_display_content.iter_mut().take(shift_amount) {
            *n = ' ';
        }
        self.display_content = new_display_content;
        self.print_content();
    }
}
