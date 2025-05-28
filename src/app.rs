pub mod maze;
pub mod player;

use crate::app::maze::{BOARD_SIZE, CELL_COUNT};
use crate::app::player::Movable;
use maze::MazeGrid;
use player::Player;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

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
    
    let cell_size = BOARD_SIZE/CELL_COUNT;

    let redraw_trigger = use_state(|| 0);
    let maze_ref_clone = maze_ref.clone();
    let player_ref_clone = player_ref.clone();
    let redraw_trigger_clone = redraw_trigger.clone();

    let on_keydown = Callback::from(move |e: KeyboardEvent| {
        let maze = maze_ref_clone.borrow();
        let mut player = player_ref_clone.borrow_mut();

        match e.key().as_str() {
            "ArrowUp" | "w" => player.try_move(0, -1, &maze),
            "ArrowDown" | "s" => player.try_move(0, 1, &maze),
            "ArrowLeft" | "a" => player.try_move(-1, 0, &maze),
            "ArrowRight" | "d" => player.try_move(1, 0, &maze),
            _ => return,
        }

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
              { for maze.cells.iter().enumerate().map(|(_, row)| html! {
                  <div class="row" style={format!("height: {}px;", cell_size)}>
                      {
                          for row.iter().enumerate().map(|(_, cell)| {
                            let class = if cell.is_wall { "wall" } else { "path" };
                            let has_veggie = maze.veggies.iter().any(|veggie| veggie.x == cell.x && veggie.y == cell.y && !veggie.is_eaten);
                            html! {
                              <div class={classes!("cell", class)} style={format!("width: {}px; height: {}px;", cell_size, cell_size)}>
                                {
                                    if cell.x == player_ref.x && cell.y == player_ref.y {
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
                        })
                    }
                  </div>
              })}
          </div>
        </div>
    }
}
