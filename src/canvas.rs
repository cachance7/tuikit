///! A canvas is a trait defining the draw actions
use crate::attr::Attr;
use crate::cell::Cell;
use std::error::Error;
use unicode_width::UnicodeWidthChar;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait Canvas {
    /// Get the canvas size (width, height)
    fn size(&self) -> Result<(usize, usize)>;

    /// clear the canvas
    fn clear(&mut self) -> Result<()>;

    /// change a cell of position `(row, col)` to `cell`
    /// return the width of the character/cell
    fn put_cell(&mut self, row: usize, col: usize, cell: Cell) -> Result<usize>;

    /// just like put_cell, except it accept (char & attr)
    /// return the width of the character/cell
    fn put_ch_with_attr(&mut self, row: usize, col: usize, ch: char, attr: Attr) -> Result<usize> {
        self.put_cell(row, col, Cell {ch, attr})
    }

    /// print `content` starting with position `(row, col)` with `attr`
    /// - canvas should NOT wrap to y+1 if the content is too long
    /// - canvas should handle wide characters
    /// return the printed width of the content
    fn print_with_attr(
        &mut self,
        row: usize,
        col: usize,
        content: &str,
        attr: Attr,
    ) -> Result<usize> {
        let mut cell = Cell {
            attr,
            ..Cell::default()
        };

        let mut width = 0;
        for ch in content.chars() {
            cell.ch = ch;
            self.put_cell(row, col + width, cell)?;
            width += ch.width().unwrap_or(2);
        }
        Ok(width)
    }

    /// print `content` starting with position `(row, col)` with default attribute
    fn print(&mut self, row: usize, col: usize, content: &str) -> Result<usize> {
        self.print_with_attr(row, col, content, Attr::default())
    }

    /// move cursor position (row, col) and show cursor
    fn set_cursor(&mut self, row: usize, col: usize) -> Result<()>;

    /// show/hide cursor, set `show` to `false` to hide the cursor
    fn show_cursor(&mut self, show: bool) -> Result<()>;
}

/// A sub-area of a canvas.
/// It will handle the adjustments of cursor movement, so that you could write
/// to for example (0, 0) and BoundedCanvas will adjust it to real position.
pub struct BoundedCanvas<'a> {
    canvas: &'a mut Canvas,
    top: usize,
    left: usize,
    width: usize,
    height: usize,
}

impl<'a> BoundedCanvas<'a> {
    pub fn new(
        top: usize,
        left: usize,
        width: usize,
        height: usize,
        canvas: &'a mut Canvas,
    ) -> Self {
        Self {
            canvas,
            top,
            left,
            width,
            height,
        }
    }
}

impl<'a> Canvas for BoundedCanvas<'a> {
    fn size(&self) -> Result<(usize, usize)> {
        Ok((self.width, self.height))
    }

    fn clear(&mut self) -> Result<()> {
        for row in self.top..(self.top + self.height) {
            for col in self.left..(self.left + self.width) {
                let _ = self.put_cell(row, col, Cell::empty());
            }
        }

        Ok(())
    }

    fn put_cell(&mut self, row: usize, col: usize, cell: Cell) -> Result<usize> {
        if row >= self.height || col >= self.width {
            return Err(format!("({}, {}) out of box", row, col).into());
        }

        self.canvas.put_cell(row + self.top, col + self.left, cell)
    }

    fn set_cursor(&mut self, row: usize, col: usize) -> Result<()> {
        if row >= self.height || col >= self.width {
            return Err(format!("({}, {}) out of box", row, col).into());
        }

        self.canvas.set_cursor(row + self.top, col + self.left)
    }

    fn show_cursor(&mut self, show: bool) -> Result<()> {
        self.canvas.show_cursor(show)
    }
}
