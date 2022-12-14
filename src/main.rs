use gloo_timers::callback::Interval;
use yew::html::Scope;
use yew::{classes, html, Component, Context, Html};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::start_app::<App>();
}


#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct App {
    width: u32,
    height: u32,
    _interval: Option<Interval>,
    cells: Vec<Cell>,
}

impl App {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        // Реализуйте эту функцию.
        // Возвращает количество Alive клеток вокруг клетки (`row`, `column`).
        todo!()
    }

    pub fn new(interval: Option<Interval>) -> App {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|_| Cell::Dead)
            .collect();

        App {
            width,
            height,
            _interval: interval,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn tick(&mut self) {
        // Реализуйте эту функцию.

        // Rule 1: Любая живая клетка с менее чем двумя соседями умирает
        // Rule 2: Любая живая клетка с двумя или тремя соседями живет дальше
        // Rule 3: Любая живая клетка с более чем тремя соседями умирает
        // Rule 4: Любая мертвая клетка с тремя соседями становится живой
        // Все остальные клетки остаются в том же состоянии.

        todo!();
    }
}

fn view_cell(idx: usize, cell: &Cell, _link: &Scope<App>) -> Html {
    let status = match cell {
        Cell::Alive => "cellule-live",
        Cell::Dead => "cellule-dead",
    };

    html! {
        <div key={idx} class={classes!("game-cellule", status)}></div>
    }
}

pub enum Msg {
    Tick,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(200, move || callback.emit(()));

        Self::new(Some(interval))
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => self.tick(),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_rows = self.cells.chunks(self.width as usize)
            .enumerate()
            .map(|(y, cells)| {
                let cells = cells.iter().enumerate().map(|(x, cell)| {
                    let idx = x + y * self.width as usize;
                    view_cell(idx, cell, ctx.link())
                });
                html! {
                    <div key={y} class="game-row">
                        { for cells }
                    </div>
                }
            });
        html! {
            <div>
                <section class="game-container">
                    <header class="app-header">
                        <h1 class="app-title">{ "Game of Life" }</h1>
                    </header>
                    <section class="game-area">
                        <div class="game-of-life">
                            { for cell_rows }
                        </div>
                    </section>
                </section>
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_live_neighbor_count() {
        let mut app = App::new(None);
        app.cells = vec![Cell::Dead; 64 * 64];
        for i in 0..64 {
            for j in 0..64 {
                assert_eq!(app.live_neighbor_count(i, j), 0);
            }
        }
        // Верхний левый угол
        app.cells[0] = Cell::Alive;
        assert_eq!(app.live_neighbor_count(0, 0), 0);
        app.cells[1] = Cell::Alive;
        assert_eq!(app.live_neighbor_count(0, 0), 1);
        // Поле замкнуто со всех сторон (представляет собой поверхность тора)
        assert_eq!(app.live_neighbor_count(63, 0), 2);
        assert_eq!(app.live_neighbor_count(0, 63), 1);
        assert_eq!(app.live_neighbor_count(63, 63), 1);
        // Правый верхний угол
        app.cells[63] = Cell::Alive;
        assert_eq!(app.live_neighbor_count(1, 63), 2);
    }
}
