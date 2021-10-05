use egui_backend::{
    egui::{self, Sense},
    fltk::{enums::*, prelude::*, *},
    gl, DpiScaling,
};
use fe_image_widget::ImageWidget;
use fltk_egui as egui_backend;
use std::{cell::RefCell, time::Instant};
use std::{rc::Rc, time::Duration};
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

    let mut jpeg: Option<ImageWidget> = None;
    let start_time = Instant::now();
    let mut quit = false;
    let mut http: Option<Futurized<(), ResponseData>> = None;
    let mut seed = 1;
    let mut btn_label = "Fetch random image";

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

        if let Some(ref _http) = http {
            let mut free = false;
            _http.try_resolve(|p, done| {
                match p {
                    Progress::Current(_) => {
                        app::awake();
                        btn_label = "fetching...";
                    }
                    Progress::Completed(data) => {
                        if let Some(_jpeg) = jpeg.as_mut() {
                            _jpeg.update_image(&mut painter, &data.content).unwrap();
                        } else {
                            jpeg = Some(ImageWidget::new(&mut painter, &data.content).unwrap())
                        }
                    }
                    Progress::Error(e) => {
                        println!("{}", e)
                    }
                    _ => (),
                }
                if done {
                    app::awake();
                    btn_label = "fetch next image";
                    free = true;
                }
            });

            if free {
                http = None;
            }
        }

        egui::CentralPanel::default().show(&egui_ctx, |ui| {
            ui.heading("My egui Application");
            egui::ScrollArea::auto_sized().show(ui, |ui| {
                if let Some(ref jpeg) = jpeg {
                    if ui
                        .add(jpeg.widget())
                        .interact(Sense::click())
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        println!("random image clicked!");
                    }
                }

                if ui
                    .button(btn_label)
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                {
                    if http.is_none() {
                        seed += 1;
                        let agent = AgentBuilder::new().timeout(Duration::from_secs(20)).build();
                        let url = format!("https://picsum.photos/seed/{}/{}/{}", seed, 350, 350);
                        http = Some(fetch(agent, url));
                    }
                }
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

use asynchron::{Futurize, Futurized, InnerTaskHandle, Progress};
use std::io::Read;
use ureq::AgentBuilder;

#[derive(Clone, Debug)]
pub struct ResponseData {
    pub url: String,
    pub content_type: String,
    pub content: Vec<u8>,
}

fn fetch(agent: ureq::Agent, url: String) -> Futurized<(), ResponseData> {
    let req = Futurize::task(
        0,
        move |task: InnerTaskHandle| -> Progress<(), ResponseData> {
            let res = if let Ok(res) = agent.get(&url).call() {
                res
            } else {
                return Progress::Error(
                    format!("Network problem, unable to request url: {}", &url).into(),
                );
            };

            // check if progress is canceled
            if task.should_cancel() {
                return Progress::Canceled;
            }

            if res.status() == 200 {
                let url = res.get_url().to_string();
                let content_type = res.content_type().to_string();
                let mut content = Vec::new();
                if let Err(_) = res.into_reader().read_to_end(&mut content) {
                    return Progress::Error("Unable read content data".to_string().into());
                };

                // and check here also.
                if task.should_cancel() {
                    Progress::Canceled
                } else {
                    let data = ResponseData {
                        url,
                        content_type,
                        content,
                    };

                    Progress::Completed(data)
                }
            } else {
                Progress::Error(format!("Network error, status: {}", res.status()).into())
            }
        },
    );
    req.try_do();
    req
}
