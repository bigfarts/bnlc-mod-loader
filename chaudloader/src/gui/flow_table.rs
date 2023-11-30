use fltk::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
struct FlowTableData {
    padding_top: i32,
    padding_right: i32,
    padding_bottom: i32,
    padding_left: i32,
    spacing_horizontal: i32,
    spacing_vertical: i32,
    min_column_width: i32,
}
#[derive(Debug, Clone)]
pub struct FlowTable {
    inner: fltk::group::Group,
    data: std::rc::Rc<std::cell::RefCell<FlowTableData>>,
}

impl Default for FlowTable {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, None)
    }
}
impl FlowTable {
    pub fn new<T: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: T) -> Self {
        let mut inner = fltk::group::Group::new(x, y, w, h, label);
        let data = FlowTableData::default();
        let data = std::rc::Rc::from(std::cell::RefCell::from(data));
        let data_ref = data.clone();

        // Dummy frame which will be positioned in the background
        // This keeps the scrollbars stable when using padding
        let mut dummy = fltk::frame::Frame::default();

        // Exclude any built-in children of the inner widget
        // Also want to exclude the dummy, so we do this after creating it
        let mut ignore_children = Vec::new();
        for i in 0..inner.children() {
            ignore_children.push(inner.child(i).unwrap());
        }

        inner.resize_callback(move |inner, x, y, w, h| {
            let data = *data_ref.borrow();
            let mut cx = 0;
            let mut cy = 0;
            let mut ch_max = 0;
            let x = x + data.padding_left;
            let y = y + data.padding_top;
            let w = w - data.padding_left - data.padding_right;
            let h = h - data.padding_top - data.padding_bottom;

            // Determine number of columns
            // Avoid divide by zero
            let col_width = if data.min_column_width > 0
                && data.min_column_width + data.spacing_horizontal != 0
            {
                // At least 1 column
                let num_cols = std::cmp::max(
                    1,
                    1 + (w - data.min_column_width)
                        / (data.min_column_width + data.spacing_horizontal),
                );
                // Determine width per column
                Some((w - (num_cols - 1) * data.spacing_horizontal) / num_cols)
            } else {
                None
            };

            for i in 0..inner.children() {
                let mut c = inner.child(i).unwrap();
                if ignore_children.contains(&c) {
                    // Ignore built-in children
                    continue;
                }
                let ch = c.height();

                // Adjust child width
                if let Some(cw) = col_width {
                    c.set_size(cw, ch);
                }

                // Wrap to next row
                // Skip if there are no sized elements on this row yet
                if ch_max != 0 && cx + c.w() > w {
                    cx = 0;
                    cy += ch_max + data.spacing_vertical;
                    ch_max = 0;
                }

                // Update max height for this row
                ch_max = std::cmp::max(ch_max, c.h());

                // Set position for child
                c.set_pos(x + cx, y + cy);

                // Go to next column
                cx += c.w() + data.spacing_horizontal;
            }

            dummy.set_frame(fltk::enums::FrameType::FlatBox);
            dummy.set_color(fltk::enums::Color::from_hex(0x7FFF7F));
            dummy.resize(
                x - data.padding_left,
                y - data.padding_top,
                w + data.padding_left + data.padding_right,
                cy + ch_max + data.padding_top + data.padding_bottom,
            );

            let cur_w = inner.w();
            let cur_h = inner.h();
            let dum_w = dummy.w();
            let dum_h = dummy.h();
            if (cur_w, cur_h) != (dum_w, dum_h) {
                // Wow I can't believe this works
                inner.set_size(dum_w, dum_h);
            }
        });
        Self { inner, data }
    }

    pub fn with_padding(self, top: i32, right: i32, bottom: i32, left: i32) -> Self {
        self.data.borrow_mut().padding_top = top;
        self.data.borrow_mut().padding_right = right;
        self.data.borrow_mut().padding_bottom = bottom;
        self.data.borrow_mut().padding_left = left;
        self
    }

    pub fn with_spacing(self, horizontal: i32, vertical: i32) -> Self {
        self.data.borrow_mut().spacing_horizontal = horizontal;
        self.data.borrow_mut().spacing_vertical = vertical;
        self
    }

    pub fn with_min_column_width(self, width: i32) -> Self {
        self.data.borrow_mut().min_column_width = width;
        self
    }

    pub fn end(&mut self) {
        self.inner.end();
        // Force a resize to ensure callback is called at least once before initial draw
        let x = self.x();
        let y = self.y();
        let w = self.w();
        let h = self.h();
        self.resize(x, y, w, h);
    }
}

fltk::widget_extends!(FlowTable, fltk::group::Group, inner);