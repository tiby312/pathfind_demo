
use piston_window::*;
use axgeom::*;

use axgeom::ordered_float::*;

pub fn piston_debug() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get_physical())
        .build_global()
        .unwrap();

    let area = vec2(1024, 768);
    
    let window = WindowSettings::new("dinotree test", [area.x, area.y])
        .exit_on_esc(true)
        .fullscreen(false)
        .resizable(false);

    println!("window size={:?}", window.get_size());

    let mut window: PistonWindow = window.build().unwrap();


    println!("Press \"N\" to go to the next example");
    //println!("Press \"C\" to turn off verification against naive algorithms.");
    println!("Performance suffers from not batching draw calls (piston's built in rectangle drawing primitives are used instead of vertex buffers). These demos are not meant to showcase the performance of the algorithms. See the dinotree_alg_data project for benches.");

    let mut cursor: Vec2<_> = vec2(0.0, 0.0).inner_try_into().unwrap();


    let mut botsys=pathfind::game::Game::new();

    while let Some(e) = window.next() {
        e.mouse_cursor(|[x, y]| {
            //cursor = vec2(x,y).inner_into::<f32>().inner_try_into::<F32n>().unwrap();
            cursor.x = NotNan::new(x as f32).unwrap();
            cursor.y = NotNan::new(y as f32).unwrap();
        });

        botsys.step();

        let (grid,walls) = botsys.get_wall_grid();

        let (bot_prop,bots)=botsys.get_bots();
        /*
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::N {
                curr = demo_iter.next(area);
            }

            if key == Key::C {
                check_naive = !check_naive;
                if check_naive {
                    println!("Naive checking is on. Some demo's will now check the tree algorithm against a naive non tree version");
                } else {
                    println!("Naive checking is off.");
                }
            }
        };
        */
        window.draw_2d(&e, |c, mut g, _| {
            clear([0.2; 4], g);
            c.view(); //trans(500.0,500.0);
            //curr.step(cursor, &c, &mut g, check_naive);
            for x in 0..walls.dim().x{
                for y in 0..walls.dim().y{
                    if walls.get(vec2(x,y)){
                        let topleft=grid.to_world_topleft(vec2(x,y)).inner_as::<f64>();
                        let r=grid.spacing as f64;
                        rectangle([1.0,1.0,1.0,0.5], [topleft.x,topleft.y,r,r], c.transform, g);
                    }
                }
            }
            for b in bots.iter(){
                let p=b.bot.pos.inner_as::<f64>();
                let r=bot_prop.radius.dis() as f64;
                let r=r*0.2;
                rectangle([1.0,0.0,1.0,2.0], [p.x-r,p.y-r,r*2.,r*2.], c.transform, g);
            }
        });
    }
}

