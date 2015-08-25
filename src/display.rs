use layout::{ Rect, BoxType, LayoutBox };
use css::{ Color, Value };

type DisplayList = Vec<DisplayCommand>;

enum DisplayCommand {
    SolidColor(Color, Rect)
}

struct Canvas {
    pixels: Vec<Color>,
    width: usize,
    height: usize
}

fn build_display_list (layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    list
}

fn render_layout_box (list: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(list, layout_box);
    render_borders(list, layout_box);
    // TODO: Render text

    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}

fn render_background (list: &mut DisplayList, layout_box: &LayoutBox) {
    get_color(layout_box, "background")
        .map(|color| list.push(DisplayCommand::SolidColor(color, layout_box.dimensions.border_box())));
}

fn get_color (layout_box: &LayoutBox, name: &str) -> Option<Color> {
    match layout_box.box_type {
        BoxType::BlockNode(style) | BoxType::InlineNode(style) => {
            match style.value(name) {
                Some(Value::ColorValue(color)) => Some(color),
                _ => None
            }
        },
        BoxType::AnonymousBlock => None
    }
}

fn render_borders (list: &mut DisplayList, layout_box: &LayoutBox) {
    let color = match get_color(layout_box, "border-color") {
        Some(color) => color,
        _ => return // if there is no color it is transparent => dont draw
    };

    let d = &layout_box.dimensions;
    let border_box = d.border_box();

    // left
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y,
        width: d.border.left,
        height: border_box.height
    }));

    // right
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x + border_box.width - d.border.right,
        y: border_box.y,
        width: d.border.right,
        height: border_box.height
    }));

    // top
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y,
        width: border_box.width,
        height: d.border.top
    }));

    //bottom
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y + border_box.height - d.border.bottom,
        width: border_box.width,
        height: d.border.bottom
    }));
}

fn paint (layout_root: &LayoutBox, bounds: Rect) -> Canvas {
    let display_list = build_display_list(layout_root);
    let mut canvas = Canvas::new(bounds.width as usize, bounds.height as usize);

    for item in display_list {
        canvas.paint_item(&item);
    }

    canvas
}

impl Canvas {
    fn new (width: usize, height: usize) -> Canvas {
        let white = Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255
        };

        Canvas {
            pixels: vec![white; width * height],
            width: width,
            height: height
        }
    }

    fn paint_item (&mut self, item: &DisplayCommand) {
        match item {
            &DisplayCommand::SolidColor(color, rect) => {
                // clamp to canvas size
                let x0 = rect.x.clamp(0.0, self.width as f32) as usize;
                let y0 = rect.y.clamp(0.0, self.height as f32) as usize;
                let x1 = (rect.x + rect.width).clamp(0.0, self.width as f32) as usize;
                let y1 = (rect.y + rect.height).clamp(0.0, self.height as f32) as usize;

                for y in (y0 .. y1) {
                    for x in (x0 .. x1) {
                        // TODO alpha compositing with existing pixel
                        self.pixels[x + y * self.width] = color;
                    }
                }
            }
        }
    }
}

trait Clamp {
    fn clamp(self, lower: Self, upper: Self) -> Self;
}

impl Clamp for f32 {
    fn clamp(self, lower: f32, upper: f32) -> f32 {
        self.max(lower).min(upper)
    }
}
