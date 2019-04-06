// An attribute to hide warnings for unused code.
//#![allow(dead_code)]
//region: use statements
use dodrio::bumpalo::{self, Bump};
use dodrio::{Node, Render};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
//cannot use rand::thread_rng; with wasm. instead use OsRng
//clarification: https://medium.com/@rossharrison/generating-sudoku-boards-pt-3-rust-for-webassembly-85bd7294c34a
use rand::rngs::OsRng;
use rand::Rng;
//endregion
//region: enum, structs, const,...
const SRC_FOR_CARD_FACE_DOWN: &str = "content/img/mem_image_closed.png";
//multiline string literal in Rust ends line with \
static GAME_RULES: &str = "The game starts with a grid of 8 randomly shuffled card pairs face down - 16 cards in all. \
The first player flips over two cards with two clicks. \
If the cards do not match, the next player will start his turn with a click to turn both cards back face down, then two clicks to flip over two card. \
If the cards match, they are left face up and the player receives a point and continues with the next turn. No additional third click needed in that case.";

static GAME_DESCRIPTION: &str = "This is a programming example for Rust Webassembly Virtual Dom application. \
For the sake of simplicity, it is made as for single player mode. \
The simple memory game is for kids. The images are funny cartoon characters from the alphabet. \
The cards grid is only 4x4.";

enum CardStatus {
    CardFaceDown,
    CardFaceUpTemporary,
    CardFaceUpPermanently,
}

struct Card {
    status: CardStatus,
    //src attribute for HTML element image
    src_as_image_source: String,
    //id attribute for HTML element image contains the card index
    id_as_card_index: String,
}

struct CardGrid {
    vec_cards: Vec<Card>,
    //The player in one turn clicks 2 times and open 2 cards. If not match,
    //the third click closes opened cards and
    //it starts the next player turn.
    count_click_inside_one_turn: i32,
    card_index_of_first_click: usize,
    card_index_of_second_click: usize, //card_index_of_second_click
    //counts only clicks that flip the card. The third click is not counted.
    count_all_clicks: i32,
}
//endregion

impl CardGrid {
    /// Construct a new `CardGrid` component.
    fn new() -> CardGrid {
        let mut vec_of_random_numbers = Vec::new();
        //region: find 8 distinct random numbers between 1 and 26 for the alphabet cards
        let mut rng = rand::thread_rng();
        let mut i = 0;
        while i < 8 {
            //gen_range is lower inclusive, upper exclusive
            let num: i32 = rng.gen_range(1, 26 + 1);
            if dbg!(vec_of_random_numbers.contains(&num)) {
                //do nothing if the random number is repeated
                dbg!(num);
            } else {
                //push a pair of the same number
                vec_of_random_numbers.push(num);
                vec_of_random_numbers.push(num);
                i += 1;
            }
        }
        //endregion
        //region: shuffle the numbers
        let mut vrndslice = vec_of_random_numbers.as_mut_slice();
        //cannot use rand_rng and new Slice Shuffle with wasm.
        //instead use OsRng with deprecated rand::Rng::shuffle
        //gslice.shuffle(&mut thread_rng());
        OsRng::new().unwrap().shuffle(&mut vrndslice);
        //endregion
        //region: create Cards from random numbers
        dbg!("vec_of_random_numbers values");
        let mut vec_card_from_random_numbers = Vec::new();
        for (index, random_number) in vec_of_random_numbers.iter().enumerate() {
            let src = String::from(format!("content/img/mem_image_{:02}.png", random_number));
            dbg!(&src);
            let new_card = Card {
                status: CardStatus::CardFaceDown,
                src_as_image_source: src,
                id_as_card_index: format!("img{}", index),
            };
            vec_card_from_random_numbers.push(new_card);
        }
        //endregion
        //region: return from constructor
        CardGrid {
            vec_cards: vec_card_from_random_numbers,
            count_click_inside_one_turn: 0,
            card_index_of_first_click: 0,
            card_index_of_second_click: 0,
            count_all_clicks: 0,
        }
        //endregion
    }
}

//region: The `Render` implementation.
//It is called for every Dodrio animation frame to render the vdom.
impl Render for CardGrid {
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        use dodrio::builder::*;
        //the card grid is a html css flex object (like a table) with <img> inside
        //other html elements are pretty simple.

        //region: here I use Closures only for readability, to avoid deep code nesting.
        //The closures are used later in this code.

        //The on_click event passed by javascript executes all the logic
        //to change the fields of the CardGrid struct.
        //That stuct is the only source of data to later render the virtual dom.
        let closure_on_click = |card_grid: &mut CardGrid, img: web_sys::HtmlImageElement| {
            //we have 3 possible clicks in one turn with different code branches.
            if card_grid.count_click_inside_one_turn >= 2 {
                //third click closes first and second card
                card_grid.vec_cards[card_grid.card_index_of_first_click].status =
                    CardStatus::CardFaceDown;
                card_grid.vec_cards[card_grid.card_index_of_second_click].status =
                    CardStatus::CardFaceDown;
                card_grid.count_click_inside_one_turn = 0;
            } else {
                //id attribute of image html element is prefixed with img ex. "img12"
                let this_click_card_index = (img.id()[3..]).parse::<usize>().unwrap();

                match card_grid.vec_cards[this_click_card_index].status {
                    //if card facedown, flip it
                    CardStatus::CardFaceDown => {
                        card_grid.vec_cards[this_click_card_index].status =
                            CardStatus::CardFaceUpTemporary;
                        if card_grid.count_click_inside_one_turn == 0 {
                            //if is first click, just count the clicks
                            card_grid.card_index_of_first_click = this_click_card_index;
                            card_grid.count_click_inside_one_turn += 1;
                            card_grid.count_all_clicks += 1;
                        } else if card_grid.count_click_inside_one_turn == 1 {
                            //if is second click, flip the card and then check for card match
                            card_grid.card_index_of_second_click = this_click_card_index;
                            card_grid.count_click_inside_one_turn += 1;
                            card_grid.count_all_clicks += 1;
                            //if the cards match, we don't need the third click
                            if card_grid.vec_cards[card_grid.card_index_of_first_click]
                                .src_as_image_source
                                == card_grid.vec_cards[card_grid.card_index_of_second_click]
                                    .src_as_image_source
                            {
                                // the two cards matches. make them permanent FaceUp
                                card_grid.vec_cards[card_grid.card_index_of_first_click].status =
                                    CardStatus::CardFaceUpPermanently;
                                card_grid.vec_cards[card_grid.card_index_of_second_click].status =
                                    CardStatus::CardFaceUpPermanently;
                                card_grid.count_click_inside_one_turn = 0;
                            }
                        }
                    }
                    //do nothing if player clicks the faceUp cards
                    CardStatus::CardFaceUpTemporary => (),
                    CardStatus::CardFaceUpPermanently => (),
                };
            }
        };

        //prepare vectors for the Virtual Dom flex_col with image,
        //then push them in flex_row to create the vector vec_flex_row_bump
        let closure_vec_flex_row_bump = {
            let mut vec_flex_row_bump = Vec::new();
            for x in 1..5 {
                let mut vec_flex_col_bump = Vec::new();
                for y in 1..5 {
                    let index = (x - 1) * 4 + y - 1;
                    let src = match self.vec_cards[index].status {
                        CardStatus::CardFaceDown => SRC_FOR_CARD_FACE_DOWN,
                        CardStatus::CardFaceUpTemporary => {
                            self.vec_cards[index].src_as_image_source.as_str()
                        }
                        CardStatus::CardFaceUpPermanently => {
                            self.vec_cards[index].src_as_image_source.as_str()
                        }
                    };
                    let id = self.vec_cards[index].id_as_card_index.as_str();
                    let flex_col_bump = div(bump)
                        .attr("class", "m_flex_col")
                        .children([img(bump)
                            .attr("src", src)
                            .attr("id", id)
                            //on click needs a code Closure in Rust. Dodrio and wasm-bindgen
                            //generate the javascript code to call it properly.
                            .on("click", move |root, vdom, event| {
                                // If the event's target is our image...
                                let img = match event
                                    .target()
                                    .and_then(|t| t.dyn_into::<web_sys::HtmlImageElement>().ok())
                                {
                                    None => return,
                                    //?? Don't understand what this does. The original was written for Input element.
                                    Some(input) => input,
                                };
                                //we need our Struct CardGrid for Rust to write something.
                                //It comes in the parameter root.
                                //All we have to change is the struct CardGrid fields.
                                //The method render will later use that for rendering the new html.
                                let card_grid = root.unwrap_mut::<CardGrid>();
                                closure_on_click(card_grid, img);
                                // Finally, re-render the component on the next animation frame.
                                vdom.schedule_render();
                            })
                            .finish()])
                        .finish();
                    vec_flex_col_bump.push(flex_col_bump);
                }
                let flex_row_bump = div(bump)
                    .attr("class", "m_flex_row")
                    .children(vec_flex_col_bump)
                    .finish();
                vec_flex_row_bump.push(flex_row_bump);
            }
            vec_flex_row_bump
        };
        //endregion

        //region: create the virtual dom
        div(bump)
            .attr("class", "m_container")
            .children([
                h1(bump)
                    .children([text(bumpalo::format!(in bump, "mem2{}","").into_bump_str())])
                    .finish(),
                //div for the flex table object defined in css with <img> inside
                div(bump)
                    .attr("style", "margin-left: auto;margin-right: auto;")
                    .children(closure_vec_flex_row_bump)
                    .finish(),
                h3(bump)
                    .children([text(
                        bumpalo::format!(in bump, "Count of Clicks: {}", self.count_all_clicks)
                            .into_bump_str(),
                    )])
                    .finish(),
                h4(bump)
                    .children([text(GAME_DESCRIPTION)])
                    .finish(),
                h2(bump)
                    .children([text(
                        bumpalo::format!(in bump, "Memory game rules: {}", "").into_bump_str(),
                    )])
                    .finish(),
                h4(bump)
                    .children([text(GAME_RULES)])
                    .finish(),
                h6(bump)
                    .children([
                        text(bumpalo::format!(in bump, "Learning Rust programming: {}", "").into_bump_str(),),
                        a(bump)
                            .attr("href", "https://github.com/LucianoBestia/mem2")  
                            .attr("target","_blank")              
                            .children([text(bumpalo::format!(in bump, "https://github.com/LucianoBestia/mem2{}", "").into_bump_str(),)])
                            .finish(),
                    ])
                    .finish(),
            ])
            .finish()
    }
}
//endregion

#[wasm_bindgen(start)]
pub fn run() {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    // Get the document's `<body>`.
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    // Construct a new `CardGrid` rendering component.
    let card_grid = CardGrid::new();

    // Mount the component to the `<body>`.
    let vdom = dodrio::Vdom::new(&body, card_grid);

    // Run the component forever.
    vdom.forget();
}
