
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

    let a=vec2((1920./1.2f32).floor(),(1080./1.2f32).floor());
    let mut glsys=very_simple_2d::WindowedSystem::new(a.inner_as(),&events_loop,"pathfind demo");
    glsys.set_viewport_from_width(area.x);
    //let mut glsys=very_simple_2d::FullScreenSystem::new(&events_loop);
        


    let mut texture=glsys.canvas_mut().texture("tileset.png",vec2(22,9)).unwrap();
    //let mut texture=glsys.canvas_mut().texture("tileset2.png",vec2(15,4)).unwrap();
    
    /*
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
    */
    let wall_save={
        let (grid,walls) = botsys.get_wall_grid();
                    
        let mut sprites = glsys.canvas_mut().sprites();

        for x in 0..walls.dim().x{
            for y in 0..walls.dim().y{
                let curr=vec2(x,y);
                if walls.get(curr){
                    let vv=grid.to_world_topleft(curr);

                    
                    const TOP_LEFT:Vec2<u32>=vec2(1,1);
                    const TOP:Vec2<u32>=vec2(2,1);
                    const TOP_RIGHT:Vec2<u32>=vec2(3,1);
                    const LEFT:Vec2<u32>=vec2(1,2);
                    const RIGHT:Vec2<u32>=vec2(3,2);

                    const BOTTOM_LEFT:Vec2<u32>=vec2(1,3);
                    const BOTTOM:Vec2<u32>=vec2(2,3);
                    const BOTTOM_RIGHT:Vec2<u32>=vec2(3,3);
                    const INNER:Vec2<u32>=vec2(2,2);
                    

                    const T:bool=true;
                    const F:bool=false;

                    /*
                    let c:[([bool;4],Vec2<u32>);8]=[
                        ([T,T,F,F],TOP_LEFT),
                        ([T,T,T,F],TOP),
                        ([F,T,T,F],TOP_RIGHT),
                        ([F,T,T,T],RIGHT),
                        ([F,F,T,T],BOTTOM_RIGHT),
                        ([T,F,T,T],BOTTOM),
                        ([T,F,F,T],BOTTOM_LEFT),
                        ([T,T,F,T],LEFT),
                    ];
                    */


                    //RIGHT,BOTTOM,lEFT,TOP
                    let ans1=[
                        walls.get_option(curr+vec2(1,0)).unwrap_or(T),
                        walls.get_option(curr+vec2(0,1)).unwrap_or(T),
                        walls.get_option(curr+vec2(-1,0)).unwrap_or(T),
                        walls.get_option(curr+vec2(0,-1)).unwrap_or(T)
                        ];

                    //BOTTOM RIGHT,BOTTOM LEFT,TOP LEFT,TOP RIGHT
                    let ans2=[
                        walls.get_option(curr+vec2(1,1)).unwrap_or(T),
                        walls.get_option(curr+vec2(-1,1)).unwrap_or(T),
                        walls.get_option(curr+vec2(-1,-1)).unwrap_or(T),
                        walls.get_option(curr+vec2(1,-1)).unwrap_or(T)
                        ];

                    let coord=match (ans1,ans2){
                        ([T,T,F,F],_)=>TOP_LEFT,
                        ([T,T,T,F],_)=>TOP,
                        ([F,T,T,F],_)=>TOP_RIGHT,
                        ([F,T,T,T],_)=>RIGHT,
                        ([F,F,T,T],_)=>BOTTOM_RIGHT,
                        ([T,F,T,T],_)=>BOTTOM,
                        ([T,F,F,T],_)=>BOTTOM_LEFT,
                        ([T,T,F,T],_)=>LEFT,
                        ([T,T,T,T],[T,T,T,F])=>vec2(1,5),
                        ([T,T,T,T],[T,T,F,T])=>vec2(2,5),
                        ([T,T,T,T],[F,T,T,T])=>vec2(1,4),
                        ([T,T,T,T],[T,F,T,T])=>vec2(2,4),


                         ([T,T,T,T],[T,F,T,F])=>vec2(3,4),
                         ([T,T,T,T],[F,T,F,T])=>vec2(3,5),
                        _=>INNER
                    };
                    /*
                    let coord=match ans{
                        [T,T,F,F]=>TOP_LEFT,
                        [T,T,T,F]=>TOP,
                        [F,T,T,F]=>TOP_RIGHT,
                        [F,T,T,T]=>RIGHT,
                        [F,F,T,T]=>BOTTOM_RIGHT,
                        [T,F,T,T]=>BOTTOM,
                        [T,F,F,T]=>BOTTOM_LEFT,
                        [T,T,F,T]=>LEFT,
                        [T,T,T,T]=>INNER,
                        _=>INNER
                    };
                    */







                    //let coord=TOP;

                    sprites.add(grid.to_world_center(vec2(x,y)),texture.coord_to_index(coord));
                }
            }
        }
        sprites.save()
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
            Event::MainEventsCleared=>{
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


                        wall_save.draw(canvas,&mut texture,[1.0,1.0,1.0,1.0],grid.spacing/2.0);
                        //square_save.draw(canvas,[1.0,1.0,1.0,0.5]);

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
                        

                        let mut circles = canvas.circles();
                        for b in bots.iter(){
                            circles.add(b.bot.pos);
                        }
                        circles.send_and_draw([1.0,0.0,1.0,2.0],bot_prop.radius.dis()*0.2);
                    }
                    glsys.swap_buffers();
                }
            },
            _ => {},
        }    
    });

    
    
}
