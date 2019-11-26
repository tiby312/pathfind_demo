use demodesktopgraphics::glutin;
use demodesktopgraphics::GlSys;
use demodesktopgraphics::circle_program;
use demodesktopgraphics::vbo::*;


use axgeom::Vec2;
use axgeom::vec2;
use fps_counter::FPSCounter;
use glutin::event::WindowEvent;

use glutin::event::VirtualKeyCode;
use glutin::event::Event;
use glutin::event_loop::ControlFlow;



//TODO make this a feature dependency.
mod piston_debug;

fn main(){
    //default_main();
    piston_debug::piston_debug();
}

fn default_main() {
    rayon::ThreadPoolBuilder::new().num_threads(num_cpus::get_physical()).build_global().unwrap();
     

    let events_loop = glutin::event_loop::EventLoop::new();


    let mut botsys=pathfind::game::Game::new();

    let mut glsys=GlSys::new(&events_loop);
    let mut circle_program=circle_program::CircleProgram::new();

    let dim=glsys.get_dim();
    println!("dim={:?}",dim);
    
    //let mut border=compute_border(game_response.new_game_world.unwrap().0,[startx as f32,starty as f32]);
    let border=axgeom::Rect::new(0.0,1920.,0.0,1080.);
    
    let _radius=10.0;

    

    let mut bot_buffer=Buffer::create_vbo(0);

    let mut wall_buffer=Buffer::create_vbo(0);
        

    struct Ba{
        pos:Vec2<f32>,
        id:u64
    }

    let mut mousepos=vec2(0.0,0.0);
    let mut mouse_active=false;
    let mut poses:Vec<Ba>=Vec::new(); 
    
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
                WindowEvent::Touch(touch)=>{
                    let glutin::dpi::LogicalPosition{x,y}=touch.location;
                    //let x=(x*0.84) as f32; //TODO why needed????
                    //let y=(y*0.84) as f32; 
                    let x=x as f32;
                    let y=y as f32;

                    match touch.phase{
                        glutin::event::TouchPhase::Started=>{

                            let mut found=false;
                            for i in poses.iter(){
                                if i.id == touch.id{
                                    //panic!("There is a problem!");
                                    found=true;
                                    break;
                                }
                            }
                            if found==false{
                                poses.push(Ba{id:touch.id,pos:vec2(x,y)});
                            }
                        },
                        glutin::event::TouchPhase::Ended | glutin::event::TouchPhase::Cancelled=>{
                            //poses.clear();
                            poses.retain(|a|a.id!=touch.id);
                        },
                        glutin::event::TouchPhase::Moved=>{
                            let mut ok=false;
                            for k in poses.iter_mut(){
                                if k.id==touch.id{
                                    k.pos=vec2(x,y);
                                    ok=true;
                                    break;
                                }
                            }
                            
                            if ok ==false{
                                panic!("Didnt find touch");
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

                    let mut va:Vec<Vec2<f32>>=poses.iter().map(|a|a.pos).collect();
                    if mouse_active{
                        let mouseposx=mousepos.x-(dim.x as f32/2.0);
                        let mouseposy=mousepos.y-(dim.y as f32/2.0);
                    
                        let ((x1,x2),(y1,y2))=border.get();
                        let w=x2-x1;
                        let h=y2-y1;

                        let mouseposx=mouseposx*(w/dim.x as f32);
                        let mouseposy=mouseposy*(h/dim.y as f32);
                       
                        va.push(vec2(mouseposx,mouseposy));
                    }

                    botsys.step();


                    let wall_radius={
                        let (grid,walls) = botsys.get_wall_grid();

                        if walls.len()!=wall_buffer.get_num_verticies(){
                            wall_buffer.re_generate_buffer(walls.len()); 
                        }


                        let ww=wall_buffer.get_verts_mut().iter_mut();

                        for ((a,w),b) in walls.iter().zip(ww){
                            let alpha=if w{
                                1.0
                            }else{
                                0.2
                            };
                            let a=grid.to_world_center(a);
                            //let a=a+grid.cell_radius()/2.0;

                            *b=circle_program::Vertex([a.x,a.y,alpha]);
                        }

                        /*
                        for (a,b) in wall_buffer.get_verts_mut().iter_mut().zip(botsys.wall_iter()){
                            *a=Vertex([b.x,b.y,1.0]);
                        }
                        */
                        wall_buffer.update();

                        grid.cell_radius() 
                    };
                    
                    
                    if botsys.bot_len()!=bot_buffer.get_num_verticies(){
                        bot_buffer.re_generate_buffer(botsys.bot_len()); 
                    }
                    let (bot_prop,bots)=botsys.get_bots();
                    for (a,b) in bot_buffer.get_verts_mut().iter_mut().zip(bots.iter()){
                        let b=&b.bot;
                        let _alpha=b.vel.magnitude2()*0.01;
                        *a=circle_program::Vertex([b.pos.x,b.pos.y,1.0])
                    }
                    bot_buffer.update();
                    

                    let mut ss=circle_program.new_draw_session([0.0,0.0,0.0],border);
                    ss.draw_vbo_section(dim,&wall_buffer,0,botsys.get_wall_grid().1.len(),[1.0,0.0,1.0],wall_radius*0.9,false);
                    ss.draw_vbo_section(dim,&bot_buffer,0,botsys.bot_len(),[1.0,1.0,0.0],bot_prop.radius.dis()*0.3,true);
                    glsys.swap_buffers();
        
                    /*
                    let mut ss=glsys.new_draw_session([0.0,0.0,0.0],border,radius,false);
                    ss.draw_vbo_section(&bot_buffer,10,botsys.bot_len(),[1.0,0.0,1.0]);
                    ss.finish();
                    */

                    last_time=Some(std::time::Instant::now());
                }
            },
            _ => {},
        }    
    });

    
    
}
