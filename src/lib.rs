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
const GAME_TITLE: &'static str = "mem2";
const SRC_FOR_CARD_FACE_DOWN: &'static str = "content/img/mem_image_00_cardfacedown.png";

//multiline string literal in Rust ends line with \
const GAME_RULES:&'static str = "The game starts with a grid of 8 randomly shuffled card pairs face down - 16 cards in all. \
The first player flips over two cards with two clicks. \
If the cards do not match, the next player will start his turn with a click to turn both cards back face down, then two clicks to flip over two card. \
If the cards match, they are left face up and the player receives a point and continues with the next turn. No additional third click needed in that case.";

const GAME_DESCRIPTION:&'static str = "This is a programming example for Rust Webassembly Virtual Dom application. \
For the sake of simplicity, it is made as for single player mode. \
The simple memory game is for kids. The images are funny cartoon characters from the alphabet. \
The cards grid is only 4x4.";

//the zero element is card_facedown or empty, alphabet begins with 01 : A
const SPELLING: [&'static str; 27] = [
    "", "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel", "india",
    "juliet", "kilo", "lima", "mike", "november", "oscar", "papa", "quebec", "romeo", "sierra",
    "tango", "uniform", "victor", "whiskey", "xray", "yankee", "zulu",
];

enum CardStatus {
    CardFaceDown,
    CardFaceUpTemporary,
    CardFaceUpPermanently,
}

struct Card {
    status: CardStatus,
    //field for src attribute for HTML element imagea and filename of card image
    card_number_and_img_src: usize,
    //field for id attribute for HTML element image contains the card index
    card_index_and_id: usize,
}

struct CardGrid {
    vec_cards: Vec<Card>,
    //The player in one turn clicks 2 times and open 2 cards. If not match,
    //the third click closes opened cards and
    //it starts the next player turn.
    count_click_inside_one_turn: u32,
    card_index_of_first_click: usize,
    card_index_of_second_click: usize,
    //counts only clicks that flip the card. The third click is not counted.
    count_all_clicks: u32,
}
//endregion

impl CardGrid {
    /// Construct a new `CardGrid` component. Only once on the begining.
    fn new() -> CardGrid {
        //region: find 8 distinct random numbers between 1 and 26 for the alphabet cards
        //vec_of_random_numbers is 0 based
        let mut vec_of_random_numbers = Vec::new();
        let mut rng = rand::thread_rng();
        let mut i = 0;
        while i < 8 {
            //gen_range is lower inclusive, upper exclusive
            let num: usize = rng.gen_range(1, 26 + 1);
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

        //region: Index 0 is special and reserved for FaceDown. Cards start with base 1
        let new_card = Card {
            status: CardStatus::CardFaceDown,
            card_number_and_img_src: 0,
            card_index_and_id: 0,
        };
        vec_card_from_random_numbers.push(new_card);
        //endregion

        //region: create the 16 card and push to the vector
        for (index, random_number) in vec_of_random_numbers.iter().enumerate() {
            let new_card = Card {
                status: CardStatus::CardFaceDown,
                //dereference random number from iterator
                card_number_and_img_src: *random_number,
                //card base index will be 1. 0 is reserved for FaceDown.
                card_index_and_id: index + 1,
            };
            vec_card_from_random_numbers.push(new_card);
        }
        //endregion
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

//region: `Render` implementation.
//It is called for every Dodrio animation frame to render the vdom.
//Probably only when something changes. Here it is a click on the cards.
//Not sure about that, but I don't see a reason to make execute it otherwise.
impl Render for CardGrid {
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        use dodrio::builder::*;
        //the card grid is a html css grid object (like a table) with <img> inside
        //other html elements are pretty simple.

        //region: here I use Closures only for readability, to avoid deep code nesting.
        //The closures are used later in this code.

        //format the src string
        let from_card_number_to_img_src = |card_number: usize| {
            bumpalo::format!(in bump, "content/img/mem_image_{:02}.png",card_number).into_bump_str()
        };

        //The on_click event passed by javascript executes all the logic
        //and changes only the fields of the CardGrid struct.
        //That stuct is the only permanent data storage for later render the virtual dom.
        let closure_on_click = |card_grid: &mut CardGrid, img: web_sys::HtmlImageElement| {
            //we have 3 possible clicks in one turn with different code branches.
            if card_grid.count_click_inside_one_turn >= 2 {
                //third click closes first and second card
                card_grid.vec_cards[card_grid.card_index_of_first_click].status =
                    CardStatus::CardFaceDown;
                card_grid.vec_cards[card_grid.card_index_of_second_click].status =
                    CardStatus::CardFaceDown;
                card_grid.card_index_of_first_click = 0;
                card_grid.card_index_of_second_click = 0;
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
                            //if is the first click, just count the clicks and open one card.
                            //before the first click reset the spelling.
                            //Usefull when there is no third click.
                            card_grid.card_index_of_first_click = 0;
                            card_grid.card_index_of_second_click = 0;
                            card_grid.card_index_of_first_click = this_click_card_index;
                            card_grid.count_click_inside_one_turn += 1;
                            card_grid.count_all_clicks += 1;
                        } else if card_grid.count_click_inside_one_turn == 1 {
                            //if is the second click, flip the card and then check for card match
                            card_grid.card_index_of_second_click = this_click_card_index;
                            card_grid.count_click_inside_one_turn += 1;
                            card_grid.count_all_clicks += 1;
                            //if the cards match, we don't need the third click
                            if card_grid.vec_cards[card_grid.card_index_of_first_click]
                                .card_number_and_img_src
                                == card_grid.vec_cards[card_grid.card_index_of_second_click]
                                    .card_number_and_img_src
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

        //prepare a vector for the Virtual Dom for grid_item with <img>
        //the grid_container needs only grid_items. There is no need for rows and columns in css grid.
        let closure_vec_grid_item_bump = {
            let mut vec_grid_item_bump = Vec::new();
            for x in 1..=16 {
                let index: usize = x;
                let img_src = match self.vec_cards[index].status {
                    CardStatus::CardFaceDown => SRC_FOR_CARD_FACE_DOWN,
                    CardStatus::CardFaceUpTemporary => {
                        from_card_number_to_img_src(self.vec_cards[index].card_number_and_img_src)
                    }
                    CardStatus::CardFaceUpPermanently => {
                        from_card_number_to_img_src(self.vec_cards[index].card_number_and_img_src)
                    }
                };
                // code for sound and opacity transition
                let mut onclick_sound_and_opacity_transition = "";
                if self.count_click_inside_one_turn <= 1 {
                    onclick_sound_and_opacity_transition = bumpalo::format!(in bump,
                        "this.style.opacity=1; var audio = new Audio('content/sound/mem_sound_{:02}.mp3');audio.play();",
                        self.vec_cards[index].card_number_and_img_src
                        )
                        .into_bump_str();
                }
                let img_id =
                    bumpalo::format!(in bump, "img{:02}",self.vec_cards[index].card_index_and_id)
                        .into_bump_str();
                let mut opacity = bumpalo::format!(in bump, "opacity:{}", 1).into_bump_str();
                if img_src == SRC_FOR_CARD_FACE_DOWN {
                    opacity = bumpalo::format!(in bump, "opacity:{}", 0.2).into_bump_str();
                }
                //creating 16 <div> in loop
                let grid_item_bump = div(bump)
                    .attr("class", "grid_item")
                    .children([img(bump)
                        .attr("src", img_src)
                        .attr("id", img_id)
                        .attr("style", opacity)
                        .attr("onclick", onclick_sound_and_opacity_transition)
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
                vec_grid_item_bump.push(grid_item_bump);
            }
            vec_grid_item_bump
        };

        // the header can show only the game title or two spellings. Not everything together.
        //game title is visible in the header
        let mut closure_grid_header = {
            div(bump)
                .attr("class", "grid_container_header")
                .attr("style", "grid-template-columns: auto;")
                .children([div(bump)
                    .attr("class", "grid_item")
                    .attr("style", "text-align: center;")
                    .children([text(GAME_TITLE)])
                    .finish()])
                .finish()
        };
        // if the Spellings are visible, than don't show GameTitle, because there is not
        //enought space on smartphones
        if self.card_index_of_first_click != 0 || self.card_index_of_second_click == !0 {
            closure_grid_header = {
                div(bump)
                    .attr("class", "grid_container_header")
                    .attr("style", "grid-template-columns: auto auto;")
                    .children([
                        div(bump)
                            .attr("class", "grid_item")
                            .attr("style", "text-align: left;")
                            .children([text(
                                SPELLING[self.vec_cards[self.card_index_of_first_click]
                                    .card_number_and_img_src],
                            )])
                            .finish(),
                        div(bump)
                            .attr("class", "grid_item")
                            .attr("style", "text-align: right;")
                            .children([text(
                                SPELLING[self.vec_cards[self.card_index_of_second_click]
                                    .card_number_and_img_src],
                            )])
                            .finish(),
                    ])
                    .finish()
            };
        }
        //endregion

        //region: create the whole virtual dom
        div(bump)
            .attr("class", "m_container")
            .children([
                closure_grid_header,
                //div for the css grid object defined in css with <img> inside
                div(bump)
                    .attr("class", "grid_container")
                    .attr("style", "margin-left: auto;margin-right: auto;")
                    .children(closure_vec_grid_item_bump)
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
