
use draw::{Canvas, Drawing, Shape, Style, Fill, Stroke, RGB, Position};
use draw::render::{self, svg::SvgRenderer};
use draw::shape::LineBuilder;
use crate::definitions::*;


// colors
const RECT_COLOR: RGB = RGB::new(65,105,225);
const BACKGROUND_COLOR: RGB = RGB::new(128, 128, 128);
const STROKE_COLOR: RGB = RGB::new(0, 0, 0);
const LINE_COLOR: RGB = RGB::new(220,20,60);
const LINE_WIDTH: u32 = 1;

fn create_rectangle(x: f32, y: f32, width: u32, height: u32, fill: RGB) -> Drawing {
    // create a rectangle
    let mut rect = Drawing::new(Shape::Rectangle {
        width :  width,
        height: height,
    });

    // move it around
    rect.position.x = x;
    rect.position.y = y;

    // give it a cool style
    rect.style = Style {
        fill: Some(Fill{color: fill}),
        stroke: Some(Stroke{color: STROKE_COLOR, width: 1}),
    };
    rect   
}

fn create_line_from_net(plan: &Floorplan, net: &Net, canvas_height: u32) -> Drawing {
    let get_center = |i: usize| -> (f32, f32) {
        let (pos_x, pos_y, rect) = plan[net.pins[i]];
        let (center_x, center_y) = rect.center(pos_x, pos_y);
        (center_x as f32, canvas_height as f32 - center_y as f32)
    };

    let (x, y) = get_center(0);
    let mut line = LineBuilder::new(Position::new(x, y));
    for i in 1..net.pins.len() {
        // manhatten line
        // let (x1, y1) = get_center(i - 1);
        // let (x2, y2) = get_center(i);
        // line.line_to(Position{x: x2, y: y1});
        // line.line_to(Position{x: x2, y: y2});

        // euclidean line
        let (x, y) = get_center(i);
        line.line_to(Position{x, y});
    }
    
    // Consume the builder, turn the line into a shape for use with the display list
    let shape: Shape = line.into();
    let mut line_drawing = Drawing::new(shape);
    line_drawing.style =  Style {
        fill: None,
        stroke: Some(Stroke{color: LINE_COLOR, width: LINE_WIDTH}),
    };
    line_drawing
}

fn compute_canvas_size(plan: &Floorplan) -> (u32, u32) {
    let (max_x, max_y) : (usize, usize) = plan
    .iter()
    .fold((0,0),
     |(acc_x, acc_y), (x, y, rect) | 
     (acc_x.max(x + rect.width),  acc_y.max(y + rect.height))
    );
    (max_x as u32, max_y as u32)
}

pub fn draw_floorplan(plan: &Floorplan, file: &str, net_list: &Vec<Net>, draw_nets: bool) {
    let (canvas_width, canvas_height) = compute_canvas_size(&plan);
    let mut canvas = Canvas::new(canvas_width, canvas_height);
    
    // add background
    let background = create_rectangle(0.0, 0.0, canvas_width, canvas_height, BACKGROUND_COLOR);
    canvas.display_list.add(background);
    
    // add rectangles
    for (x, y, module_rect) in plan {
        let mut rect = create_rectangle(*x as f32 , *y as f32, module_rect.width as u32, module_rect.height as u32, RECT_COLOR);      
        // shift origin from upper left to lower left
        rect.position.y = canvas_height as f32 - rect.position.y - module_rect.height as f32;
        canvas.display_list.add(rect);
    }

    // add nets
    if draw_nets {
        for net in net_list.iter() {
            let line = create_line_from_net(&plan, net, canvas_height);
            canvas.display_list.add(line);
        }
    }

    // save the canvas as an svg
    eprintln!("--> drawing floorplan to {}", file);
    render::save(
        &canvas,
        file,
        SvgRenderer::new(),
    )
    .expect("Failed to save");
}
