
use very_simple_2d::glutin;

use axgeom::*;
use fps_counter::FPSCounter;
use glutin::event::WindowEvent;

use glutin::event::VirtualKeyCode;
use glutin::event::Event;
use glutin::event_loop::ControlFlow;


fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(num_cpus::get_physical()).build_global().unwrap();
     
    let events_loop = glutin::event_loop::EventLoop::new();

    let (mut botsys,area)=pathfind::game::Game::new();

    //let mut glsys=very_simple_2d::WindowedSystem::new(area.inner_as(),&events_loop);
    let mut glsys=very_simple_2d::FullScreenSystem::new(&events_loop);
        

    let square_save={
        let (grid,walls) = botsys.get_wall_grid();
                    
        let mut squares = glsys.canvas_mut().rects(); //grid.spacing*0.5

        for x in 0..walls.dim().x{
            for y in 0..walls.dim().y{
                if walls.get(vec2(x,y)){
                    let vv=grid.to_world_topleft(vec2(x,y));


                    squares.add(rect(vv.x,vv.x+grid.spacing,vv.y,vv.y+grid.spacing));
                    //squares.add(grid.to_world_topleft(vec2(x,y)));
                }
            }
        }
        squares.save()
    };

    let mut mousepos=vec2(0.0,0.0);
    let mut mouse_active=false;
    
    let _fps=FPSCounter::new();


    let mut timer=very_simple_2d::RefreshTimer::new(16);
   
    events_loop.run(move |event,_,control_flow| {
        match event {
            Event::WindowEvent{ event, .. } => match event {
                WindowEvent::KeyboardInput{input,..}=>{       
                    match input.virtual_keycode{
                        Some(VirtualKeyCode::Escape)=>{
                            *control_flow=ControlFlow::Exit;
                        },
                        _=>{}
                    }
                },
                WindowEvent::CloseRequested => {*control_flow=ControlFlow::Exit;},
                WindowEvent::Resized(_logical_size) => {
                    
                },
                WindowEvent::CursorMoved{modifiers:_,device_id:_,position:logical_position} => {
                    let glutin::dpi::LogicalPosition{x,y}=logical_position;
                    mousepos=vec2(x as f32,y as f32);
                },
                WindowEvent::MouseInput{modifiers:_,device_id:_,state,button}=>{
                    if button==glutin::event::MouseButton::Left{
                        match state{
                            glutin::event::ElementState::Pressed=>{  
                                mouse_active=true;  
                                
                            }
                            glutin::event::ElementState::Released=>{
                                mouse_active=false;
                            }
                        }
                    }
                },
                _=>{}
            },
            Event::EventsCleared=>{
                if timer.is_ready(){
                    if mouse_active{
                        /*
                        let v=vec2(border.width as f32*(mousepos.x/window_border.x),
                                   border.vec().y as f32*(mousepos.y/window_border.y));
                   
                        va.push(v);
                        */
                    }

                    botsys.step();
                    let (grid,_) = botsys.get_wall_grid();
                    let (bot_prop,bots)=botsys.get_bots();

                    {
                        let canvas=glsys.canvas_mut();
                        canvas.clear_color([0.2,0.2,0.2]);


                        square_save.draw(canvas,[1.0,1.0,1.0,0.5]);

                        {
                            let mut lines = canvas.lines(1.0);
                            for b in bots.iter(){

                                if let pathfind::game::GridBotState::Moving(a,_b)=b.state{
                                    let curr=a.pos();
                                    let curr_pos=grid.to_world_center(curr);
                                    lines.add(b.bot.pos,curr_pos);
                                }
                            }
                            lines.send_and_draw([1.0,0.0,0.0,0.3]);
                        }
                        {
                            let mut lines = canvas.lines(1.0);
                            for b in bots.iter(){

                                if let pathfind::game::GridBotState::Moving(a,_b)=b.state{
                            
                                    if let Some(next)=a.peek(){
                                        let next_pos=grid.to_world_center(next);
                                        lines.add(b.bot.pos,next_pos);
                                    }
                                }
                            }
                            lines.send_and_draw([0.0,0.0,1.0,0.3]);
                        }
                        

                        let mut circles = canvas.circles(bot_prop.radius.dis()*0.2);
                        for b in bots.iter(){
                            circles.add(b.bot.pos);
                        }
                        circles.send_and_draw([1.0,0.0,1.0,2.0]);
                    }
                    glsys.swap_buffers();
                }
            },
            _ => {},
        }    
    });

    
    
}
