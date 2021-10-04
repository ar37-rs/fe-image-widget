use egui_backend::{
    egui,
    fltk::{enums::*, prelude::*, *},
    gl, DpiScaling,
};
use fe_image_widget::{Options, SVGWidget};
use fltk_egui as egui_backend;
use std::rc::Rc;
use std::{cell::RefCell, time::Instant};
const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::get_system_colors();
    app::set_font_size(20);
    let mut glut_win = window::GlutWindow::new(200, 200, SCREEN_WIDTH, SCREEN_HEIGHT, None);
    glut_win.set_mode(Mode::Opengl3);
    glut_win.end();
    glut_win.make_resizable(true);
    glut_win.show();
    glut_win.make_current();

    let (painter, egui_input_state) =
        egui_backend::with_fltk(&mut glut_win, DpiScaling::Custom(1.));
    let mut egui_ctx = egui::CtxRef::default();

    let state = Rc::from(RefCell::from(egui_input_state));
    let painter = Rc::from(RefCell::from(painter));
    glut_win.handle({
        let state = state.clone();
        let painter = painter.clone();
        let mut w = glut_win.clone();
        move |_, ev| match ev {
            enums::Event::Push
            | enums::Event::Released
            | enums::Event::KeyDown
            | enums::Event::KeyUp
            | enums::Event::MouseWheel
            | enums::Event::Resize
            | enums::Event::Move
            | enums::Event::Drag => {
                let mut state = state.borrow_mut();
                state.fuse_input(&mut w, ev, &mut painter.borrow_mut());
                true
            }
            _ => false,
        }
    });
    let mut opt = Options::default();
    opt.fontdb.load_system_fonts();
    let mut svg = SVGWidget::new(
        &mut painter.borrow_mut(),
        include_bytes!("./tiger.svg"),
        opt.to_ref(),
    )
    .unwrap();
    let size = svg.size();
    svg.resize(size.x / 2.0, size.y / 2.0);
    let start_time = Instant::now();
    let mut quit = false;

    while a.wait() {
        let mut state = state.borrow_mut();
        let mut painter = painter.borrow_mut();
        state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(state.input.take());

        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.6, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        egui::CentralPanel::default().show(&egui_ctx, |ui| {
            ui.heading("My egui Application");
            egui::ScrollArea::auto_sized().show(ui, |ui| {
                ui.add(svg.widget()).on_hover_text("grrr...!");
                if ui
                    .button("Quit?")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                {
                    quit = true;
                }
            });
        });

        let (egui_output, paint_cmds) = egui_ctx.end_frame();
        state.fuse_output(&mut glut_win, &egui_output);

        let paint_jobs = egui_ctx.tessellate(paint_cmds);

        //Draw egui texture
        painter.paint_jobs(None, paint_jobs, &egui_ctx.texture());
        glut_win.swap_buffers();
        glut_win.flush();

        if egui_output.needs_repaint {
            app::awake()
        } else if quit {
            break;
        }
    }
}
