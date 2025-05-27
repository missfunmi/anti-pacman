pub mod maze;
pub mod player;

use maze::MazeGrid;
use player::Player;
use std::cell::RefCell;
use std::rc::Rc;
use log::{log, Level};
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

const BOARD_SIZE: usize = 640;

#[function_component(App)]
pub fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());
    // let player: Player = Player {
    //     avatar: "👻".to_string(),
    //     x: 7,
    //     y: 0,
    //     speed: 0.0,
    // };

    let maze_ref = use_mut_ref(|| {
        let mut maze = MazeGrid::init();
        maze.add_walls();
        maze.generate_veggies(5);
        maze
    });

    let player_ref = use_mut_ref(|| Player {
        avatar: "👻".to_string(),
        x: 7,
        y: 0,
        speed: 0.0,
    });
    
    let cell_size = BOARD_SIZE / 8;
    // let mut maze = MazeGrid::init();
    // maze.add_walls();
    // maze.add_player(&player);
    // maze.generate_veggies(5);

    // let maze_ref = Rc::new(RefCell::new(maze));
    // let player_ref = Rc::new(RefCell::new(player));

    {
        let maze_ref = maze_ref.clone();
        let player_ref = player_ref.clone();
        use_effect_with((), move |_| {
            let mut maze = maze_ref.borrow_mut();
            let player = player_ref.borrow();
            maze.add_player(&player);
            || {}
        });
    }
    
    let redraw_trigger = use_state(|| 0);

    let maze_ref_clone = maze_ref.clone();
    let player_ref_clone = player_ref.clone();
    let redraw_trigger_clone = redraw_trigger.clone();

    let on_keydown = Callback::from(move |e: KeyboardEvent| {
        let mut maze = maze_ref_clone.borrow_mut();
        let mut player = player_ref_clone.borrow_mut();

        let (x, y) = (player.x, player.y);
        let (new_x, new_y) = match e.key().as_str() {
            "ArrowUp" | "w" => (x, y.saturating_sub(1)),
            "ArrowDown" | "s" => (x, y + 1),
            "ArrowLeft" | "a" => (x.saturating_sub(1), y),
            "ArrowRight" | "d" => (x + 1, y),
            _ => return,
        };

        maze.move_player((x, y), (new_x, new_y));
        player.x = new_x;
        player.y = new_y;
        redraw_trigger_clone.set(*redraw_trigger_clone + 1);
    });

    {
        use_effect_with((), move |_| {
            let on_keydown = on_keydown.clone();
            let closure = Closure::<dyn Fn(KeyboardEvent)>::wrap(Box::new(move |e| {
                on_keydown.emit(e);
            }));
            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
            || {}
        });
    }

    let maze = maze_ref.borrow();
    let player_ref = player_ref.borrow();
    let avatar = &player_ref.avatar;

    html! {
      <div class="maze-container"
            style={format!("position: relative; width: {}px; height: {}px;", BOARD_SIZE, BOARD_SIZE)}
        >
          <div class="maze">
              { for maze.cells.iter().enumerate().map(|(y, row)| html! {
                  <div class="row" style={format!("height: {}px;", cell_size)}>
                      {
                          for row.iter().enumerate().map(|(x, cell)| {
                            log!(Level::Info, "cell.has_player: {:?}", cell.has_player);
                            let class = if cell.is_wall { "wall" } else { "path" };
                            let has_veggie = maze.veggies.iter().any(|veggie| veggie.x == cell.x && veggie.y == cell.y && !veggie.is_eaten);
                            html! {
                              <div class={classes!("cell", class)} style={format!("width: {}px; height: {}px;", cell_size, cell_size)}>
                                {
                                    if cell.has_player {
                                        html! { <span id="player">{ &avatar }</span> } }
                                    else {
                                        html! {}
                                    }
                                }
                                {
                                    if has_veggie {
                                        html! { <span class="veggie">{"🥦"}</span> }
                                    } else {
                                        html! {}
                                    }
                                }
                                </div>
                            }
                        }
                      )
                    }
                  </div>
              })}
          </div>
        </div>
    }
}
