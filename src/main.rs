
use very_simple_2d::*;
use very_simple_2d::glutin;

use axgeom::Vec2;
use axgeom::vec2;
use fps_counter::FPSCounter;
use glutin::event::WindowEvent;

use glutin::event::VirtualKeyCode;
use glutin::event::Event;
use glutin::event_loop::ControlFlow;

/*
use glutin::event::WindowEvent;

use glutin::event::VirtualKeyCode;
use glutin::event::Event;
use glutin::event_loop::ControlFlow;
*/


//TODO make this a feature dependency.
//mod piston_debug;

/*
fn main(){
    //default_main();
    piston_debug::piston_debug();
}
*/


fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(num_cpus::get_physical()).build_global().unwrap();
     
    let events_loop = glutin::event_loop::EventLoop::new();

    let (mut botsys,area)=pathfind::game::Game::new();

    //let area= vec2(1920,1080);

    let mut glsys=very_simple_2d::WindowedSystem::new(area.inner_as(),&events_loop);
    //let mut glsys=very_simple_2d::FullScreenSystem::new(&events_loop);
    //glsys.set_viewport_from_width(1920.);

    let window_border:Vec2<f32>=glsys.get_dim().inner_as();
    let aspect_ratio=axgeom::AspectRatio(window_border.inner_as());

    //let symbols=Symbols::new();

    

    //let (mut botsys,game_response)=MenuGame::new(aspect_ratio,&symbols);
    

    //let mut border=game_response.new_game_world.unwrap().0;
    //glsys.set_viewport_from_width(border.width as f32);
    //let radius=game_response.new_game_world.unwrap().1;
    
    //let mut color=game_response.color.unwrap();


    let mut mousepos=vec2(0.0,0.0);
    let mut mouse_active=false;
    
    let _fps=FPSCounter::new();

    let mut last_time:Option<std::time::Instant>=None;

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
                let do_run = match last_time{
                    Some(last_time)=>{
                        if last_time.elapsed().as_millis()>=16{
                            true
                        }else{
                            false
                        }
                    },
                    None=>{
                        true
                    }
                };

                if do_run{

                    if mouse_active{
                        /*
                        let v=vec2(border.width as f32*(mousepos.x/window_border.x),
                                   border.vec().y as f32*(mousepos.y/window_border.y));
                   
                        va.push(v);
                        */
                    }

                    botsys.step();
                    let (grid,walls) = botsys.get_wall_grid();
                    let (bot_prop,bots)=botsys.get_bots();

                    {
                        let mut draw_session=glsys.get_sys();

                        {
                            let mut squares = draw_session.squares(grid.spacing*0.5,[1.0,1.0,1.0,0.5]);

                            for x in 0..walls.dim().x{
                                for y in 0..walls.dim().y{
                                    if walls.get(vec2(x,y)){
                                        squares.add(grid.to_world_center(vec2(x,y)));
                                    }
                                }
                            }
                            squares.draw();
                        }

                        {
                            let mut lines = draw_session.lines(1.0,[1.0,0.0,0.0,0.3]);
                            for b in bots.iter(){

                                if let pathfind::game::GridBotState::Moving(a,_b)=b.state{
                                    let curr=a.pos();
                                    let curr_pos=grid.to_world_center(curr);
                                    lines.add(b.bot.pos,curr_pos);
                                }
                            }
                            lines.draw();
                        }
                        {
                            let mut lines = draw_session.lines(1.0,[0.0,0.0,1.0,0.3]);
                            for b in bots.iter(){

                                if let pathfind::game::GridBotState::Moving(a,_b)=b.state{
                            
                                    if let Some(next)=a.peek(){
                                        let next_pos=grid.to_world_center(next);
                                        lines.add(b.bot.pos,next_pos);
                                    }
                                }
                            }
                            lines.draw();
                        }
                        /*
                        for b in bots.iter(){
                            let p=b.bot.pos.inner_as::<f64>();
                            
                            if let pathfind::game::GridBotState::Moving(a,_b)=b.state{
                                let curr=a.pos();
                                let curr_pos=grid.to_world_center(curr).inner_into::<f64>();
                                line([1.0, 0.0, 0.0, 0.3], 1.0, [p.x,p.y, curr_pos.x, curr_pos.y], transform, g);

                                let next=a.peek();
                                if let Some(next)=a.peek(){
                                    let next_pos=grid.to_world_center(next).inner_into::<f64>();
                                    line([0.0, 0.0, 1.0, 0.3], 1.0, [p.x,p.y, next_pos.x, next_pos.y], transform, g);
                                }

                            }


                            let r=bot_prop.radius.dis() as f64;
                            let r=r*0.2;
                            rectangle([1.0,0.0,1.0,2.0], [p.x-r,p.y-r,r*2.,r*2.], transform, g);
                        }
                        */

                        let mut circles = draw_session.circles(bot_prop.radius.dis()*0.2,[1.0,0.0,1.0,2.0]);
                        for b in bots.iter(){
                            circles.add(b.bot.pos);
                        }
                        circles.draw();
                    }
                    glsys.swap_buffers();
                    

                    last_time=Some(std::time::Instant::now());
                }
            },
            _ => {},
        }    
    });

    
    
}
