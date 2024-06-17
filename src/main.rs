use leptos::*;

struct Dimensions {
    width: usize,
    height: usize,
}

struct Cell {
    example: bool,
}

#[component]
fn App() -> impl IntoView {
    let (dimensions, set_dimensions) = create_signal(Dimensions { width: 5, height: 5 });

    let mut grid: Vec<(ReadSignal<Cell>, WriteSignal<Cell>)> = vec![];
    for _ in 0..dimensions.with_untracked(|d| d.width * d.height) {
        grid.push(create_signal(Cell { example: false }));
    }
    let (grid, set_grid) = create_signal(grid);

    let resize = move |dim: Dimensions| {
        set_dimensions.update(|dimensions| *dimensions = dim);
        let mut new_vec = vec![];
        for _ in 0..dimensions.with(|d| d.width * d.height) {
            new_vec.push(create_signal(Cell { example: false }));
        }
        set_grid.update(|grid|
            *grid = new_vec
        );
    };

    view! {
        <button on:click=move |_| { resize(Dimensions { width: 5, height: 5 }); }>
            {"5x5"}
        </button>
        <button on:click=move |_| { resize(Dimensions { width: 10, height: 10 }); }>
            {"10x10"}
        </button>
        <button on:click=move |_| { resize(Dimensions { width: 15, height: 15 }); }>
            {"15x15"}
        </button>
        <GoodTable
            grid
            dimensions
        />
        <br />
        <br />
        <BadTable
            grid
            dimensions
        />

    }
}

#[component]
fn GoodTable(
    grid: ReadSignal<Vec<(ReadSignal<Cell>, WriteSignal<Cell>)>>,
    dimensions: ReadSignal<Dimensions>,
) -> impl IntoView {
    let rows = move || {
        with! { |grid, dimensions|
            grid.chunks(dimensions.width)
                .enumerate()
                .map(|(i, cells)| {
                    let cells = cells.to_vec();
                    // using cells.len() in the key somehow fixes the rendering
                    let key = format!("{}-{}-{}-{}", cells.len(), dimensions.width, dimensions.height, i);
                    (key, i, cells)
                })
                .collect::<Vec<(String, usize, Vec<(ReadSignal<Cell>, WriteSignal<Cell>)>)>>()
        }
    };

    view! {
        <div>Good Table</div>
        <For
            each=rows
            key=|tuple| tuple.0.clone()
            children=move |(key, y, row)| {
                view! {
                    <div key={key} style="width: fit-content; border: solid red;">
                        <div style="display: table-cell; width: 20px; color: blue;">{format!("{}", row.len())}</div>
                        <Row row y dimensions />
                    </div>
                }
            }
        />
    }
}

#[component]
fn BadTable(
    grid: ReadSignal<Vec<(ReadSignal<Cell>, WriteSignal<Cell>)>>,
    dimensions: ReadSignal<Dimensions>,
) -> impl IntoView {
    let rows = move || {
        with! { |grid, dimensions|
            grid.chunks(dimensions.width)
                .enumerate()
                .map(|(i, cells)| {
                    let cells = cells.to_vec();
                    // Not calling cells.len() and keeping the result leaves the row missing cells
                    let key = format!("{}-{}-{}", dimensions.width, dimensions.height, i);
                    (key, i, cells)
                })
                .collect::<Vec<(String, usize, Vec<(ReadSignal<Cell>, WriteSignal<Cell>)>)>>()
        }
    };

    view! {
        <div>Bad Table</div>
        <For
            each=rows
            key=|tuple| tuple.0.clone()
            children=move |(key, y, row)| {
                view! {
                    <div key={key} style="width: fit-content; border: solid red;">
                        <div style="display: table-cell; width: 20px; color: blue;">{format!("{}", row.len())}</div>
                        <Row row y dimensions />
                    </div>
                }
            }
        />
    }
}

#[component]
fn Row(
    row: Vec<(ReadSignal<Cell>, WriteSignal<Cell>)>,
    y: usize,
    dimensions: ReadSignal<Dimensions>,
) -> impl IntoView {
    let row_cells = move || {
        row.iter()
            .cloned()
            .enumerate()
            .map(|(x, cell)| {
                let index_offset = y * with!(|dimensions| dimensions.width);
                (index_offset + x, cell)
            })
            .collect::<Vec<(usize, (ReadSignal<Cell>, WriteSignal<Cell>))>>()
    };

    view! {
        <For
            each=row_cells
            key=|tuple| tuple.0
            children=move |(index, cell)| {
                view! {
                    <Cell index cell />
                }
            }
        />
    }
}

#[component]
fn Cell(
    index: usize,
    cell: (ReadSignal<Cell>, WriteSignal<Cell>),
) -> impl IntoView {
    let (cell, set_cell) = cell;

    view! {
        // example use of attribute in cell
        <div style:color=move || cell.with(|c| if c.example { "red" } else { "black" })
        style="display: table-cell; border: solid black; width: 25px;"
        on:click=move |e| {
                e.stop_propagation();
                leptos::logging::log!("clicking cell {}", index);
                set_cell.update(|c| c.example = !c.example);
            }
        >
            {index}
        </div>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
