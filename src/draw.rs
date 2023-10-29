
use draw::{Canvas, Drawing, Shape, Style, Fill, Stroke, RGB};
use draw::render::{self, svg::SvgRenderer};
use crate::polish_expression::Floorplan;

// colors
const RECT_COLOR: RGB = RGB::new(65,105,225);
const BACKGROUND_COLOR: RGB = RGB::new(128, 128, 128);
const STROKE_COLOR: RGB = RGB::new(0, 0, 0);

fn create_rectangle(x: f32, y: f32, width: u32, heigth: u32, fill: RGB) -> Drawing {
    // create a rectangle
    let mut rect = Drawing::new(Shape::Rectangle {
        width :  width,
        height: heigth,
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

fn compute_canvas_size(plan: &Floorplan) -> (u32, u32) {
    let (max_x, max_y) : (usize, usize) = plan
    .iter()
    .fold((0,0),
     |(acc_x, acc_y), (x, y, rect, _) | 
     (acc_x.max(x + rect.width),  acc_y.max(y + rect.heigth))
    );
    (max_x as u32, max_y as u32)
}

pub fn draw_floorplan(plan: &Floorplan, file: &str) {
    let (canvas_width, canvas_height) = compute_canvas_size(&plan);
    let mut canvas = Canvas::new(canvas_width, canvas_height);
    
    // add background
    let background = create_rectangle(0.0, 0.0, canvas_width, canvas_height, BACKGROUND_COLOR);
    canvas.display_list.add(background);
    
    // add rectangles
    for (x, y, module_rect, _) in plan {
        let mut rect = create_rectangle(*x as f32 , *y as f32, module_rect.width as u32, module_rect.heigth as u32, RECT_COLOR);      
        // shift origin from upper left to lower left
        rect.position.y = canvas_height as f32 - rect.position.y - module_rect.heigth as f32;
        canvas.display_list.add(rect);
    }

    // save the canvas as an svg
    println!("--> drawing floorplan to {}", file);
    render::save(
        &canvas,
        file,
        SvgRenderer::new(),
    )
    .expect("Failed to save");
}
