use egaku2d::glutin;

use axgeom::*;
use fps_counter::FPSCounter;
use glutin::event::WindowEvent;

use glutin::event::Event;
use glutin::event::VirtualKeyCode;
use glutin::event_loop::ControlFlow;

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get_physical())
        .build_global()
        .unwrap();

    let events_loop = glutin::event_loop::EventLoop::new();

    let (mut botsys, area) = pathfind::game::Game::new();

    let a = vec2((1920. / 1.2f32).floor(), (1080. / 1.2f32).floor()).inner_as();
    let mut glsys =
        egaku2d::WindowedSystem::new([a.x,a.y], &events_loop, "pathfind demo");

    glsys.set_viewport_from_width(area.x);
    //glsys.set_viewport_from_width(300.0);

    //let mut glsys=very_simple_2d::FullScreenSystem::new(&events_loop);

    let texture = glsys
        .texture("tileset.png", [23, 9])
        .unwrap();

    let dino_tex = glsys.texture("dino.png", [24, 1]).unwrap();
    let wall_save = {
        let (grid, walls) = botsys.get_wall_grid();

        let mut sprites = glsys.canvas_mut().sprites();

        for x in 0..walls.dim().x {
            for y in 0..walls.dim().y {
                let curr = vec2(x, y);
                if walls.get(curr) {
                    const TOP_LEFT: [u8;2] = [1, 1];
                    const TOP: [u8;2] = [2, 1];
                    const TOP_RIGHT: [u8;2] = [3, 1];
                    const LEFT: [u8;2] = [1, 2];
                    const RIGHT: [u8;2] = [3, 2];

                    const BOTTOM_LEFT: [u8;2] = [1, 3];
                    const BOTTOM: [u8;2] = [2, 3];
                    const BOTTOM_RIGHT: [u8;2] = [3, 3];
                    const INNER: [u8;2] = [2, 2];

                    const T: bool = true;
                    const F: bool = false;

                    //RIGHT,BOTTOM,lEFT,TOP
                    let ans1 = [
                        walls.get_option(curr + vec2(1, 0)).unwrap_or(T),
                        walls.get_option(curr + vec2(0, 1)).unwrap_or(T),
                        walls.get_option(curr + vec2(-1, 0)).unwrap_or(T),
                        walls.get_option(curr + vec2(0, -1)).unwrap_or(T),
                    ];

                    //BOTTOM RIGHT,BOTTOM LEFT,TOP LEFT,TOP RIGHT
                    let ans2 = [
                        walls.get_option(curr + vec2(1, 1)).unwrap_or(T),
                        walls.get_option(curr + vec2(-1, 1)).unwrap_or(T),
                        walls.get_option(curr + vec2(-1, -1)).unwrap_or(T),
                        walls.get_option(curr + vec2(1, -1)).unwrap_or(T),
                    ];

                    let coord = match (ans1, ans2) {
                        ([T, T, F, F], _) => TOP_LEFT,
                        ([T, T, T, F], _) => TOP,
                        ([F, T, T, F], _) => TOP_RIGHT,
                        ([F, T, T, T], _) => RIGHT,
                        ([F, F, T, T], _) => BOTTOM_RIGHT,
                        ([T, F, T, T], _) => BOTTOM,
                        ([T, F, F, T], _) => BOTTOM_LEFT,
                        ([T, T, F, T], _) => LEFT,

                        ([T, T, T, T], [T, T, T, F]) => [1, 5],
                        ([T, T, T, T], [T, T, F, T]) => [2, 5],
                        ([T, T, T, T], [F, T, T, T]) => [1, 4],
                        ([T, T, T, T], [T, F, T, T]) => [2, 4],
                        ([T, T, T, T], [T, F, T, F]) => [3, 4],
                        ([T, T, T, T], [F, T, F, T]) => [3, 5],
                        _ => INNER,
                    };

                    let pos=grid.to_world_center(vec2(x, y));
                    sprites.add(
                        [pos.x,pos.y],
                        texture.coord_to_index(coord),
                        0.0,
                    );
                }
            }
        }
        sprites.save(glsys.canvas_mut())
    };

    let mut mousepos = vec2(0.0, 0.0);
    let mut mouse_active = false;

    let _fps = FPSCounter::new();

    let mut timer = egaku2d::RefreshTimer::new(16);

    let mut counter = 0;
    events_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(VirtualKeyCode::Escape) => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                },
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(_logical_size) => {}
                WindowEvent::CursorMoved {
                    device_id: _,
                    position,
                    ..
                } => {
                    //let glutin::dpi::LogicalPosition { x, y } = logical_position;
                    mousepos = vec2(position.x as f32, position.y as f32);
                }
                WindowEvent::MouseInput {
                    device_id: _,
                    state,
                    button,
                    ..
                } => {
                    if button == glutin::event::MouseButton::Left {
                        match state {
                            glutin::event::ElementState::Pressed => {
                                mouse_active = true;
                            }
                            glutin::event::ElementState::Released => {
                                mouse_active = false;
                            }
                        }
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                if timer.is_ready() {
                    if mouse_active {
                        /*
                        let v=vec2(border.width as f32*(mousepos.x/window_border.x),
                                   border.vec().y as f32*(mousepos.y/window_border.y));

                        va.push(v);
                        */
                    }
                    let canvas = glsys.canvas_mut();

                    canvas.clear_color([0.2, 0.2, 0.2]);

                    botsys.step(canvas);
                    let (grid, _) = botsys.get_wall_grid();
                    let (bot_prop, bots) = botsys.get_bots();

                    {
                    
                        //square_save.draw(canvas,[1.0,1.0,1.0,0.5]);

                        /*
                        {
                            let mut lines = canvas.lines(1.0);
                            for b in bots.iter(){

                                if let pathfind::game::GridBotState::Moving(a,_b)=b.state{
                                    let curr=a.pos();
                                    let curr_pos=grid.to_world_center(curr);
                                    lines.add(b.bot.pos.into(),curr_pos.into());
                                }
                            }
                            lines.send_and_uniforms(canvas).with_color([1.0,0.0,0.0,0.3]).draw();
                        }
                        {
                            let mut lines = canvas.lines(1.0);
                            for b in bots.iter(){

                                if let pathfind::game::GridBotState::Moving(a,_b)=b.state{

                                    if let Some((_carddir,next))=a.peek(){
                                        let next_pos=grid.to_world_center(next);
                                        lines.add(b.bot.pos.into(),next_pos.into());
                                    }
                                }
                            }
                            lines.send_and_uniforms(canvas).with_color([0.0,0.0,1.0,0.3]).draw();
                        }
                        */
                        

                        /*
                        let mut circles = canvas.circles();
                        for b in bots.iter(){
                            circles.add(b.bot.pos);
                        }
                        circles.send_and_draw([1.0,0.0,1.0,2.0],bot_prop.radius.dis()*0.2);
                        */
                        wall_save.uniforms(canvas,&texture,grid.spacing).draw();

                        {
                            let c = 4 + ((counter as f32 * 0.1) as usize % 6);

                            //let mut dinos = canvas.sprites();
                            let mut dinos=canvas.circles();
                            for (i, b) in bots.iter().enumerate() {
                                let k = (c + (i % 6)) as u8;
                                let p=b.bot.pos;
                                dinos.add(p.into());
                                //dinos.add([p.x,p.y], dino_tex.coord_to_index([k, 0]),0.0);
                            }
                            dinos.send_and_uniforms(canvas,bot_prop.radius.dis()*2.0).with_color([0.0,1.0,0.0,0.5]).draw();
                            

                            let mut dirs=canvas.lines(2.0);
                            for b in bots.iter(){
                                let b=&b.bot;
                                let k=b.pos+b.vel.normalize_to(1.0)*bot_prop.radius.dis()*0.5;
                                dirs.add(b.pos.into(),k.into());
                            }
                            dirs.send_and_uniforms(canvas).draw();

                            //dinos.send_and_uniforms(glsys.canvas_mut(),&dino_tex,bot_prop.radius.dis()*2.0).draw();
                        }
                    }

                    counter += 1;

                    glsys.swap_buffers();
                }
            }
            _ => {}
        }
    });
}
