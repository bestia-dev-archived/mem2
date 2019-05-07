//! Learning Rust Wasm/WebAssembly with Virtual Dom Dodrio with `WebSocket` communication.
//! mem2 is a simple game for kids.
//! The game UI is divided into vertical <div> sections
//! 1. Title OR Aviation spelling
//! 2. Card Grid (4x4 cards)
//! 3. Players score
//! 4. Click count
//! 5. Rules and desctiptions
//! Constructing a HTML page with Virtual DOM (vdom) is simple because it is rendered completely every tick (animation frame).
//! For the developer it is hard to think what should change in the UI when some data changes.
//! It is easier to think how to render the complete DOM for the given data.
//! The dodrio library has ticks, time intervals when it do something.
//! If a rendering is scheduled it will be done on the next tick.
//! If a rendering is not scheduled I believe nothing happens.
//!

//region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    //variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,

)]
#![allow(
    //library from dependencies have this clippy warnings. Not my code.
    clippy::cargo_common_metadata,
    clippy::multiple_crate_versions,
    clippy::wildcard_dependencies,
    //Rust is more idiomatic without return statement
    clippy::implicit_return,
    //I have private function inside a function. Self does not work there.
    clippy::use_self,
    //Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    //because then wasm-pack build --target web returns an error: export `run` not found 
    clippy::missing_inline_in_public_items
)]
//endregion

//region: extern and use statements
#[macro_use]
extern crate log;
extern crate console_error_panic_hook;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate web_sys;
//Strum is a set of macros and traits for working with enums and strings easier in Rust.
extern crate strum;
extern crate strum_macros;

use dodrio::builder::*;
use dodrio::bumpalo::{self, Bump};
use dodrio::{Cached, Node, Render};
use futures::prelude::*;
use js_sys::Reflect;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::FromEntropy;
use rand::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, WebSocket};
//Strum is a set of macros and traits for working with enums and strings easier in Rust.
use strum_macros::AsRefStr;

use std::cell::RefCell;
use std::rc::{Rc, Weak};
//endregion

//region: enum, structs, const,...
///game title
const GAME_TITLE: &str = "mem2";
///fixed filename for card face down
const SRC_FOR_CARD_FACE_DOWN: &str = "content/img/mem_image_00_cardfacedown.png";

///Text of game rules.
///Multiline string literal just works.
///End of line in the code is simply and intuitively end of line in the string.
///The special character \ at the end of the line in code means that it is NOT the end of the line for the string.
///The escape sequence \n means end of line also.
const GAME_RULES:& str = "This game is for exactly 2 players. 
Both players must have the webpage simultaneously opened in their browsers to allow communication.
To start over just refresh the webpage.
The first player clicks on 'Ask Player2 to play?' and broadcasts the message over WebSocket.
Player2 then sees on the screen 'Click here to Accept play!', clicks it and sends the message back to Player1.
The game starts with a grid of 8 randomly shuffled card pairs face down - 16 cards in all.
On the screen under the grid are clear signals which player plays and which waits.
Player1 flips over two cards with two clicks.
If the cards do not match, the other player clicks on 'Click here to Take your turn !' and both cards are flipped back face down. Then it is his turn and he clicks to flip over his two cards.
If the cards match, they are left face up permanently and the player receives a point. He continues to play, he opens the next two cards.
The player with more points wins.";

///game description
const GAME_DESCRIPTION:& str = "Learning to use Rust Wasm/WebAssembly with Dodrio Virtual Dom and WebSockets communication - second iteration. 
The simple memory game is for kids. 
The images are funny cartoon characters from the alphabet. 
The cards grid is only 4x4. 
For fun I added the sounds of Morse alphabet codes and 
show the International Aviation spelling on the screen.";

///Spelling for the alphabet - morse style
///the zero element is card face down or empty, alphabet begins with 01 : A
const SPELLING: [&str; 27] = [
    "", "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel", "india",
    "juliet", "kilo", "lima", "mike", "november", "oscar", "papa", "quebec", "romeo", "sierra",
    "tango", "uniform", "victor", "whiskey", "xray", "yankee", "zulu",
];

///`WsMessage` enum for websocket
#[derive(Serialize, Deserialize)]
enum WsMessage {
    ///connection test
    ConnectionTest {
        ///anything
        test: String,
    },
    ///want to play
    WantToPlay {
        ///ws client instance unique id. To not listen the echo to yourself.
        ws_client_instance: usize,
    },
    /// accept play
    AcceptPlay {
        ///ws client instance unique id. To not listen the echo to yourself.
        ws_client_instance: usize,
        ///act is the action to take on the receiver
        card_grid_data: String,
    },
    ///player click
    PlayerClick {
        ///ws client instance unique id. To not listen the echo to yourself.
        ws_client_instance: usize,
        ///card_index
        card_index: usize,
        ///count click inside one turn
        count_click_inside_one_turn: usize,
    },
    ///player change
    PlayerChange {
        ///ws client instance unique id. To not listen the echo to yourself.
        ws_client_instance: usize,
    },
}

///the game can be in various states and that differentiate the UI and actions
#[derive(AsRefStr)]
enum GameState {
    ///the start of the game
    Start,
    ///Player1 Asking WantToPlay
    Asking,
    ///Player2 is asked WantToPlay
    Asked,
    ///play (the turn is in cardgrid.player_turn)
    Play,
}

///the 3 possible states of one card
#[derive(Serialize, Deserialize)]
enum CardStatusCardFace {
    ///card face down
    Down,
    ///card face Up Temporary
    UpTemporary,
    ///card face up Permanently
    UpPermanently,
}

///all the data for one card
#[derive(Serialize, Deserialize)]
struct Card {
    ///card status
    status: CardStatusCardFace,
    ///field for src attribute for HTML element imagea and filename of card image
    card_number_and_img_src: usize,
    ///field for id attribute for HTML element image contains the card index
    card_index_and_id: usize,
}

///player score (cacheable)
///TODO: I don't know how to make a parent in Rust sturust. So a temporary solution is to put here all the fields it needs.
/// Not ideal.
struct PlayersAndScores {
    ///What player am I
    this_machine_player_number: usize,
    ///whose turn is now:  player 1 or 2
    player_turn: usize,
    ///player1 points
    player1_points: usize,
    ///player2 points
    player2_points: usize,
    ///parent
    parent: RefCell<Weak<CardGridRootRenderingComponent>>,
}

///The static parts can be cached
pub struct RulesAndDescription {}

///the card grid struct has all the needed data for play logic and rendering
struct CardGridRootRenderingComponent {
    ///vector of cards
    vec_cards: Vec<Card>,
    //First turn: Player1 clicks 2 times and opens 2 cards.
    //If cards match, Player1 receives one point and countinues: 2 click for 2 cards.
    //If not match: Player2 clicks the Change button to close opened cards.
    //Then starts the Player2 turn.
    ///count click inside one turn
    count_click_inside_one_turn: usize,
    ///card index of first click
    card_index_of_first_click: usize,
    ///card index of second click
    card_index_of_second_click: usize,
    ///counts only clicks that flip the card. The third click is not counted.
    count_all_clicks: usize,
    ///web socket. used it to send message onclick.
    ws: WebSocket,
    ///score
    players_and_scores: PlayersAndScores,
    ///my ws client instance unique id. To not listen the echo to yourself.
    my_ws_client_instance: usize,
    ///other ws client instance unique id. To listen only to one accepted other player.
    other_ws_client_instance: usize,
    ///game state: Start,Asking,Asked,Player1,Player2
    game_state: GameState,
    ///the static parts can be cached. I am not sure if a field in this struct is the best place to put it.
    cached_rules_and_description: Cached<RulesAndDescription>,
}
//endregion

impl Render for PlayersAndScores {
    ///This rendering will be rendered and then cached . It will not be rerendered untill invalidation.
    ///It is ivalidate, when the points change.
    ///html element to with scores for 2 players
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        //return
        div(bump)
            .attr("class", "grid_container_players")
            .attr(
                "style",
                bumpalo::format!(in bump, "grid-template-columns: auto auto auto;{}","")
                    .into_bump_str(),
            )
            .children([
                div(bump)
                    .attr("class", "grid_item")
                    .attr(
                        "style",
                        bumpalo::format!(in bump,"text-align: left;color:{};text-decoration:{}",
                            if self.player_turn==1 {"green"} else {"red"},
                            if self.this_machine_player_number==1 {"underline"} else {"none"}
                        )
                        .into_bump_str(),
                    )
                    .children([text(
                        bumpalo::format!(in bump, "player1: {}",self.player1_points)
                            .into_bump_str(),
                    )])
                    .finish(),
                div(bump)
                    .attr("class", "grid_item")
                    .attr("style", "text-align: center;")
                    .children([text("")])
                    .finish(),
                div(bump)
                    .attr("class", "grid_item")
                    .attr(
                        "style",
                        bumpalo::format!(in bump,"text-align: right;color:{};text-decoration:{}",
                            if self.player_turn==2 {"green"} else {"red"},
                            if self.this_machine_player_number==2 {"underline"} else {"none"}
                        )
                        .into_bump_str(),
                    )
                    .children([text(
                        bumpalo::format!(in bump, "player2: {}",self.player2_points)
                            .into_bump_str(),
                    )])
                    .finish(),
            ])
            .finish()
    }
}

impl PlayersAndScores {
    //???set parent How to use a weak reference ???
    fn set_parent(&mut self, card_grid: &CardGridRootRenderingComponent) {
        //How to add a parent with a weak reference ???
        /*
        let rcxxx = Rc::new(card_grid);
        *self.parent.borrow_mut() = Rc::downgrade(rcxxx);
        */
    }
}

impl Render for RulesAndDescription {
    ///This rendering will be rendered and then cached . It will not be rerendered untill invalidation.
    ///In this case I don't need to invalidate because it is a static content.
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        div(bump)
        .children([
            h4(bump)
            .children(text_with_br_newline(GAME_DESCRIPTION,bump))
            .finish(),
            h2(bump)
            .children([text(
                bumpalo::format!(in bump, "Memory game rules: {}", "").into_bump_str(),
            )])
            .finish(),
            h4(bump)
            .children(text_with_br_newline(GAME_RULES, bump))
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

///change the newline lines ending into <br> node
fn text_with_br_newline<'a>(txt: &'a str, bump: &'a Bump) -> Vec<Node<'a>> {
    let mut vec_text_node = Vec::new();
    let spl = txt.lines();
    for part in spl {
        vec_text_node.push(text(part));
        vec_text_node.push(br(bump).finish());
    }
    vec_text_node
}

//region: wasm_bindgen(start) is where everything starts
#[wasm_bindgen(start)]
///wasm_bindgen runs this functions at start
pub fn run() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    // Get the document's container to render the virtual dom component.
    let window = web_sys::window().expect("error: web_sys::window");
    let document = window.document().expect("error: window.document");
    let div_for_virtual_dom = document
        .get_element_by_id("div_for_virtual_dom")
        .expect("No #div_for_virtual_dom");

    let mut rng = SmallRng::from_entropy();
    //gen_range is lower inclusive, upper exclusive 26 + 1
    let my_ws_client_instance: usize = rng.gen_range(1, 9999);

    //todo: find out URL
    let location_href = window.location().href().expect("href not known");

    //websocket connection
    let ws = setup_ws_connection(location_href.as_str());
    //I don't know why is needed to clone the websocket connection
    let ws_c = ws.clone();

    // Construct a new `CardGridRootRenderingComponent`.
    //I added ws_c so that I can send messages on websocket
    let card_grid = CardGridRootRenderingComponent::new(ws_c, my_ws_client_instance);

    //TODO: I need crazy jumps for having a parent field
    //card_grid.players_and_scores.set_parent(&card_grid);

    // Mount the component to the `<div id="div_for_virtual_dom">`.
    let vdom = dodrio::Vdom::new(&div_for_virtual_dom, card_grid);

    //websocket on receive message callback
    setup_ws_msg_recv(&ws, &vdom);

    // Run the component forever. Forget to drop the memory.
    vdom.forget();

    Ok(())
}
//endregion

//region:CardGrid struct is the only persistant data we have in Rust Virtual Dom.dodrio
//in the constructor we initialize that data.
//Later onclick we change this data.
//at every animation frame we use only this data to render the virtual Dom.
//It knows nothing about HTML and Virtual dom.
impl CardGridRootRenderingComponent {
    /// Construct a new `CardGrid` component. Only once at the begining.
    pub fn new(ws: WebSocket, my_ws_client_instance: usize) -> Self {
        //region: find 8 distinct random numbers between 1 and 26 for the alphabet cards
        //vec_of_random_numbers is 0 based
        let mut vec_of_random_numbers = Vec::new();
        let mut rng = SmallRng::from_entropy();
        let mut i = 0;
        while i < 8 {
            //gen_range is lower inclusive, upper exclusive 26 + 1
            let num: usize = rng.gen_range(1, 27);
            if vec_of_random_numbers.contains(&num) {
                //do nothing if the random number is repeated
                debug!("random duplicate {} in {:?}", num, vec_of_random_numbers);
            } else {
                //push a pair of the same number
                vec_of_random_numbers.push(num);
                vec_of_random_numbers.push(num);
                i += 1;
            }
        }
        //endregion

        //region: shuffle the numbers
        let vrndslice = vec_of_random_numbers.as_mut_slice();
        vrndslice.shuffle(&mut rng);
        //endregion

        //region: create Cards from random numbers
        let mut vec_cards = Vec::new();

        //Index 0 is special and reserved for FaceDown. Cards start with base 1
        let new_card = Card {
            status: CardStatusCardFace::Down,
            card_number_and_img_src: 0,
            card_index_and_id: 0,
        };
        vec_cards.push(new_card);

        //create the 16 card and push to the vector
        for (index, random_number) in vec_of_random_numbers.iter().enumerate() {
            let new_card = Card {
                status: CardStatusCardFace::Down,
                //dereference random number from iterator
                card_number_and_img_src: *random_number,
                //card base index will be 1. 0 is reserved for FaceDown.
                card_index_and_id: index.checked_add(1).expect("usize overflow"),
            };
            vec_cards.push(new_card);
        }
        //endregion
        let game_rule_01 = RulesAndDescription {};
        let cached_rules_and_description = Cached::new(game_rule_01);

        let players_and_scores = PlayersAndScores {
            player1_points: 0,
            player2_points: 0,
            this_machine_player_number: 0, //unknown until WantToPlay+Accept
            player_turn: 0,
            parent: RefCell::new(Weak::new()), //empty parent. I will fill it later.
        };

        //return from constructor
        CardGridRootRenderingComponent {
            vec_cards,
            count_click_inside_one_turn: 0,
            card_index_of_first_click: 0,
            card_index_of_second_click: 0,
            count_all_clicks: 0,
            ws,
            players_and_scores,
            my_ws_client_instance,
            other_ws_client_instance: 0, //zero means not accepted yet
            game_state: GameState::Start,
            cached_rules_and_description,
        }
    }
    ///The onclick event passed by javascript executes all the logic
    ///and changes only the fields of the Card Grid struct.
    ///That stuct is the only permanent data storage for later render the virtual dom.
    fn card_on_click(&mut self) {
        //get this_click_card_index from card_grid
        let this_click_card_index = if self.count_click_inside_one_turn == 1 {
            self.card_index_of_first_click
        } else {
            self.card_index_of_second_click
        };

        if self.count_click_inside_one_turn == 1 || self.count_click_inside_one_turn == 2 {
            //region: audio play
            //prepare the audio element with src filename of mp3
            let audio_element = web_sys::HtmlAudioElement::new_with_src(
                format!(
                    "content/sound/mem_sound_{:02}.mp3",
                    self.vec_cards
                        .get(this_click_card_index)
                        .expect("error this_click_card_index")
                        .card_number_and_img_src
                )
                .as_str(),
            );

            //play() return a Promise in JSValue. That is too hard for me to deal with now.
            audio_element
                .expect("Error: HtmlAudioElement new.")
                .play()
                .expect("Error: HtmlAudioElement.play() ");
            //endregion

            //flip the card up
            self.vec_cards
                .get_mut(this_click_card_index)
                .expect("error this_click_card_index")
                .status = CardStatusCardFace::UpTemporary;

            if self.count_click_inside_one_turn == 2 {
                //if is the second click, flip the card and then check for card match

                //if the cards match, player get one point and continues another turn
                if self
                    .vec_cards
                    .get_mut(self.card_index_of_first_click)
                    .expect("error card_grid.card_index_of_first_click")
                    .card_number_and_img_src
                    == self
                        .vec_cards
                        .get(self.card_index_of_second_click)
                        .expect("error card_grid.card_index_of_second_click")
                        .card_number_and_img_src
                {
                    //give points
                    if self.players_and_scores.player_turn == 1 {
                        self.players_and_scores.player1_points += 1;
                    } else {
                        self.players_and_scores.player2_points += 1;
                    }

                    // the two cards matches. make them permanent FaceUp
                    self.vec_cards
                        .get_mut(self.card_index_of_first_click)
                        .expect("error card_grid.card_index_of_first_click")
                        .status = CardStatusCardFace::UpPermanently;
                    self.vec_cards
                        .get_mut(self.card_index_of_second_click)
                        .expect("error card_grid.card_index_of_second_click")
                        .status = CardStatusCardFace::UpPermanently;
                    self.count_click_inside_one_turn = 0;
                }
            }
        }
    }
    ///fn on change for both click and we msg.
    fn take_turn(&mut self) {
        self.players_and_scores.player_turn = if self.players_and_scores.player_turn == 1 {
            2
        } else {
            1
        };

        //click on Change button closes first and second card
        self.vec_cards
            .get_mut(self.card_index_of_first_click)
            .expect("error card_grid.card_index_of_first_click ")
            .status = CardStatusCardFace::Down;
        self.vec_cards
            .get_mut(self.card_index_of_second_click)
            .expect("error card_grid.card_index_of_second_click")
            .status = CardStatusCardFace::Down;
        self.card_index_of_first_click = 0;
        self.card_index_of_second_click = 0;
        self.count_click_inside_one_turn = 0;
    }
}
//endregion

//region: `Render` trait implementation on CardGrid struct
///It is called for every Dodrio animation frame to render the vdom.
///Probably only when something changes. Here it is a click on the cards.
///Not sure about that, but I don't see a reason to make execute it otherwise.
///It is the only place where I create HTML elements in Virtual Dom.
impl Render for CardGridRootRenderingComponent {
    #[inline]
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        //the card grid is a html css grid object (like a table) with <img> inside
        //other html elements are pretty simple.

        //region: private helper fn for Render()
        //here I use private functions for readability only, to avoid deep code nesting.
        //I don't understand closures enought to use them properly.
        //These private functions are not in the "impl Render forCardGrid" because of the error
        //method `from_card_number_to_img_src` is not a member of trait `Render`
        //there is not possible to write private and public methods in one impl block there are only pub methods.
        //`pub` not permitted there because it's implied
        //so I have to write functions outside of the impl block but inside my "module"

        ///format the src string
        fn from_card_number_to_img_src(bump: &Bump, card_number: usize) -> &str {
            bumpalo::format!(in bump, "content/img/mem_image_{:02}.png",card_number).into_bump_str()
        }

        ///prepare a vector<Node> for the Virtual Dom for 'css grid' item with <img>
        ///the grid container needs only grid items. There is no need for rows and columns in 'css grid'.
        fn div_grid_items<'a, 'bump>(
            cr_gr: &'a CardGridRootRenderingComponent,
            bump: &'bump Bump,
        ) -> Vec<Node<'bump>> {
            use dodrio::builder::*;
            let mut vec_grid_item_bump = Vec::new();
            for x in 1..=16 {
                let index: usize = x;
                //region: prepare variables and closures for inserting into vdom
                let img_src = match cr_gr.vec_cards.get(index).expect("error index").status {
                    CardStatusCardFace::Down => SRC_FOR_CARD_FACE_DOWN,
                    CardStatusCardFace::UpTemporary | CardStatusCardFace::UpPermanently => {
                        from_card_number_to_img_src(
                            bump,
                            cr_gr
                                .vec_cards
                                .get(index)
                                .expect("error index")
                                .card_number_and_img_src,
                        )
                    }
                };

                let img_id =
                    bumpalo::format!(in bump, "img{:02}",cr_gr.vec_cards.get(index).expect("error index").card_index_and_id)
                        .into_bump_str();

                let opacity = if img_src == SRC_FOR_CARD_FACE_DOWN {
                    bumpalo::format!(in bump, "opacity:{}", 0.2).into_bump_str()
                } else {
                    bumpalo::format!(in bump, "opacity:{}", 1).into_bump_str()
                };
                //endregion

                //creating 16 <div> in loop
                let grid_item_bump = div(bump)
                    .attr("class", "grid_item")
                    .children([img(bump)
                        .attr("src", img_src)
                        .attr("id", img_id)
                        .attr("style", opacity)
                        //on click needs a code Closure in Rust. Dodrio and wasm-bindgen
                        //generate the javascript code to call it properly.
                        .on("click", move |root, vdom, event| {
                            //we need our Struct CardGrid for Rust to write any data.
                            //It comes in the parameter root.
                            //All we can change is inside the struct CardGrid fields.
                            //The method render will later use that for rendering the new html.
                            let card_grid = root.unwrap_mut::<CardGridRootRenderingComponent>();

                            //the click on grid is allowed only when is the turn of this player
                            if (card_grid.game_state.as_ref() == GameState::Play.as_ref()
                                && card_grid.players_and_scores.player_turn == 1
                                && card_grid.players_and_scores.this_machine_player_number == 1)
                                || (card_grid.game_state.as_ref() == GameState::Play.as_ref()
                                    && card_grid.players_and_scores.player_turn == 2
                                    && card_grid.players_and_scores.this_machine_player_number == 2)
                            {
                                // If the event's target is our image...
                                let img = match event
                                    .target()
                                    .and_then(|t| t.dyn_into::<web_sys::HtmlImageElement>().ok())
                                {
                                    None => return,
                                    //?? Don't understand what this does. The original was written for Input element.
                                    Some(input) => input,
                                };

                                //id attribute of image html element is prefixed with img ex. "img12"
                                let this_click_card_index =
                                    (img.id().get(3..).expect("error slicing"))
                                        .parse::<usize>()
                                        .expect("error parse img id to usize");

                                //click is usefull only od facedown cards
                                if let CardStatusCardFace::Down = card_grid
                                    .vec_cards
                                    .get(this_click_card_index)
                                    .expect("error this_click_card_index")
                                    .status
                                {
                                    //the begining of the turn is count_click_inside_one_turn=0
                                    //on click imediately increase that. So first click is 1 and second click is 2.
                                    //all other clicks on the grid are not usable.
                                    card_grid.count_click_inside_one_turn += 1;

                                    if card_grid.count_click_inside_one_turn == 1 {
                                        card_grid.card_index_of_first_click = this_click_card_index;
                                        card_grid.card_index_of_second_click = 0;
                                        card_grid.count_all_clicks += 1;
                                    } else if card_grid.count_click_inside_one_turn == 2 {
                                        card_grid.card_index_of_second_click =
                                            this_click_card_index;
                                        card_grid.count_all_clicks += 1;
                                    } else {
                                        //nothing
                                    }

                                    //region: send WsMessage over websocket
                                    card_grid
                                        .ws
                                        .send_with_str(
                                            &serde_json::to_string(&WsMessage::PlayerClick {
                                                ws_client_instance: card_grid.my_ws_client_instance,
                                                card_index: this_click_card_index,
                                                count_click_inside_one_turn: card_grid
                                                    .count_click_inside_one_turn,
                                            })
                                            .expect("error sending PlayerClick"),
                                        )
                                        .expect("Failed to send PlayerClick");
                                    //endregion
                                    card_grid.card_on_click();
                                }
                                // Finally, re-render the component on the next animation frame.
                                vdom.schedule_render();
                            }
                        })
                        .finish()])
                    .finish();
                vec_grid_item_bump.push(grid_item_bump);
            }
            vec_grid_item_bump
        }

        ///the header can show only the game title or two spellings. Not everything together.
        fn div_grid_header<'a, 'bump>(
            cr_gr: &'a CardGridRootRenderingComponent,
            bump: &'bump Bump,
        ) -> Node<'bump> {
            use dodrio::builder::*;
            //if the Spellings are visible, than don't show GameTitle, because there is not
            //enought space on smartphones
            if cr_gr.card_index_of_first_click != 0 || cr_gr.card_index_of_second_click != 0 {
                //if the two opened card match use green else use red color
                let color; //haha variable does not need to be mutable. Great !

                if cr_gr
                    .vec_cards
                    .get(cr_gr.card_index_of_first_click)
                    .expect("error index")
                    .card_number_and_img_src
                    == cr_gr
                        .vec_cards
                        .get(cr_gr.card_index_of_second_click)
                        .expect("error index")
                        .card_number_and_img_src
                {
                    color = "green";
                } else if cr_gr.card_index_of_first_click == 0
                    || cr_gr.card_index_of_second_click == 0
                {
                    color = "yellow";
                } else {
                    color = "red";
                }

                {
                    //return
                    div(bump)
                .attr("class", "grid_container_header")
                .attr(
                    "style",
                    bumpalo::format!(in bump, "grid-template-columns: auto auto; color:{}",color)
                        .into_bump_str(),
                )
                .children([
                    div(bump)
                        .attr("class", "grid_item")
                        .attr("style", "text-align: left;")
                        .children([text(
                            SPELLING.get(cr_gr.vec_cards.get(cr_gr.card_index_of_first_click).expect("error index")
                                .card_number_and_img_src).expect("error index"),
                        )])
                        .finish(),
                    div(bump)
                        .attr("class", "grid_item")
                        .attr("style", "text-align: right;")
                        .children([text(
                            SPELLING.get(cr_gr.vec_cards.get(cr_gr.card_index_of_second_click).expect("error index")
                                .card_number_and_img_src).expect("error index"),
                        )])
                        .finish(),
                ])
                .finish()
                }
            } else {
                {
                    div(bump)
                        .attr("class", "grid_container_header")
                        .attr("style", "grid-template-columns: auto;")
                        .children([div(bump)
                            .attr("class", "grid_item")
                            .attr("style", "text-align: center;")
                            .children([text(GAME_TITLE)])
                            .finish()])
                        .finish()
                }
            }
        }

        ///html element to inform player what to do and get a click action from user
        fn div_game_status_and_player_actions<'a, 'bump>(
            card_grid: &'a CardGridRootRenderingComponent,
            bump: &'bump Bump,
        ) -> Node<'bump> {
            use dodrio::builder::*;

            if let GameState::Start = card_grid.game_state {
                // 1S Ask Player2 to play!
                console::log_1(&"GameState::Start".into());
                //return Ask Player2 to play!
                h3(bump)
                    .attr("id", "ws_elem")
                    .attr("style", "color:green;")
                    .children([text(
                        //show Ask Player2 to Play!
                        bumpalo::format!(in bump, "Ask other Player to play! {}", "")
                            .into_bump_str(),
                    )])
                    .on("click", move |root, vdom, _event| {
                        let card_grid = root.unwrap_mut::<CardGridRootRenderingComponent>();
                        //region: send WsMessage over websocket
                        card_grid.players_and_scores.this_machine_player_number = 1;
                        card_grid.game_state = GameState::Asking;
                        card_grid
                            .ws
                            .send_with_str(
                                &serde_json::to_string(&WsMessage::WantToPlay {
                                    ws_client_instance: card_grid.my_ws_client_instance,
                                })
                                .expect("error sending test"),
                            )
                            .expect("Failed to send");
                        //endregion
                        vdom.schedule_render();
                    })
                    .finish()
            } else if let GameState::Asking = card_grid.game_state {
                //return wait for the other player
                div_wait_for_other_player(bump)
            } else if let GameState::Asked = card_grid.game_state {
                // 2S Click here to Accept play!
                console::log_1(&"GameState::Asked".into());
                //return Click here to Accept play
                h3(bump)
                    .attr("id", "ws_elem")
                    .attr("style", "color:green;")
                    .children([text(
                        //show Ask Player2 to Play!
                        bumpalo::format!(in bump, "Click here to Accept play! {}", "")
                            .into_bump_str(),
                    )])
                    .on("click", move |root, vdom, _event| {
                        let card_grid = root.unwrap_mut::<CardGridRootRenderingComponent>();
                        //region: send WsMessage over websocket
                        card_grid.players_and_scores.this_machine_player_number = 2;
                        card_grid.players_and_scores.player_turn = 1;
                        card_grid.game_state = GameState::Play;

                        card_grid
                            .ws
                            .send_with_str(
                                &serde_json::to_string(&WsMessage::AcceptPlay {
                                    ws_client_instance: card_grid.my_ws_client_instance,
                                    //send the vector of cards because both players need cards in the same location.
                                    card_grid_data: serde_json::to_string(&card_grid.vec_cards)
                                        .expect("error serde_json"),
                                })
                                .expect("error sending test"),
                            )
                            .expect("Failed to send");
                        //endregion
                        vdom.schedule_render();
                    })
                    .finish()
            } else if card_grid.count_click_inside_one_turn >= 2 {
                if card_grid.players_and_scores.this_machine_player_number
                    == card_grid.players_and_scores.player_turn
                {
                    //return wait for the other player
                    div_wait_for_other_player(bump)
                } else {
                    //return Click here to take your turn
                    h3(bump)
                        .attr("id", "ws_elem")
                        .attr("style", "color:green;")
                        .children([text(
                            bumpalo::format!(in bump, "Click here to take your turn !{}", "")
                                .into_bump_str(),
                        )])
                        .on("click", move |root, vdom, _event| {
                            let card_grid = root.unwrap_mut::<CardGridRootRenderingComponent>();
                            //region: send WsMessage over websocket
                            card_grid
                                .ws
                                .send_with_str(
                                    &serde_json::to_string(&WsMessage::PlayerChange {
                                        ws_client_instance: card_grid.my_ws_client_instance,
                                    })
                                    .expect("error sending PlayerChange"),
                                )
                                .expect("Failed to send PlayerChange");
                            //endregion
                            card_grid.take_turn();
                            // Finally, re-render the component on the next animation frame.
                            vdom.schedule_render();
                        })
                        .finish()
                }
            } else if card_grid.count_click_inside_one_turn < 2 {
                if card_grid.players_and_scores.this_machine_player_number
                    == card_grid.players_and_scores.player_turn
                {
                    h3(bump)
                        .attr("id", "ws_elem")
                        .attr("style", "color:orange;")
                        .children([text(
                            bumpalo::format!(in bump, "Play !{}", "").into_bump_str(),
                        )])
                        .finish()
                } else {
                    //return wait for the other player
                    div_wait_for_other_player(bump)
                }
            } else {
                //unpredictable situation
                //return
                h3(bump)
                    .attr("id", "ws_elem")
                    .children([text(
                        bumpalo::format!(in bump, "gamestate: {} player {}", card_grid.game_state.as_ref(),card_grid.players_and_scores.this_machine_player_number)
                            .into_bump_str(),
                    )])
                    .finish()
            }
        }
        ///the text 'wait for other player' is used multiple times
        fn div_wait_for_other_player(bump: &Bump) -> Node {
            h3(bump)
                .attr("id", "ws_elem")
                .attr("style", "color:red;")
                .children([text(
                    bumpalo::format!(in bump, "Wait for the other player.{}", "").into_bump_str(),
                )])
                .finish()
        }
        //endregion

        //region: create the whole virtual dom. The verbose stuff is in private functions
        div(bump)
            .attr("class", "m_container")
            .children([
                div_grid_header(self, bump),
                //div for the css grid object defined in css with <img> inside
                div(bump)
                    .attr("class", "grid_container")
                    .attr("style", "margin-left: auto;margin-right: auto;")
                    .children(div_grid_items(self, bump))
                    .finish(),
                self.players_and_scores.render(bump),
                div_game_status_and_player_actions(self, bump),
                h5(bump)
                    .children([text(
                        bumpalo::format!(in bump, "Count of Clicks: {}", self.count_all_clicks)
                            .into_bump_str(),
                    )])
                    .finish(),
                self.cached_rules_and_description.render(bump),
            ])
            .finish()
        //endregion
    }
}
//endregion

//region: websocket communication
///setup websocket connection
fn setup_ws_connection(location_href: &str) -> WebSocket {
    //web-sys has websocket for Rust exactly like javascript has¸
    console::log_1(&"location_href".into());
    console::log_1(&wasm_bindgen::JsValue::from_str(location_href));
    //location_href comes in this format  http://localhost:4000/
    let mut loc_href = location_href.replace("http://", "ws://");
    //Only for debugging in the development environment
    //let mut loc_href = String::from("ws://192.168.1.57:80/");
    loc_href.push_str("mem2ws/");
    console::log_1(&wasm_bindgen::JsValue::from_str(&loc_href));
    //TODO: same server address and port as http server
    let ws = WebSocket::new(&loc_href).expect("WebSocket failed to connect.");

    //I don't know why is clone neede
    let ws_c = ws.clone();
    //It looks that the first send is in some way a handshake and is part of the connection
    //it will be execute onopen as a closure
    let open_handler = Box::new(move || {
        console::log_1(&"Connection opened, sending 'test' to server".into());
        ws_c.send_with_str(
            &serde_json::to_string(&WsMessage::ConnectionTest {
                test: String::from("test"),
            })
            .expect("error sending test"),
        )
        .expect("Failed to send 'test' to server");
    });

    let cb_oh: Closure<Fn()> = Closure::wrap(open_handler);
    ws.set_onopen(Some(cb_oh.as_ref().unchecked_ref()));
    //don't drop the open_handler memory
    cb_oh.forget();
    ws
}

/// receive websocket msg callback. I don't understand this much. Too much future and promises.
fn setup_ws_msg_recv(ws: &WebSocket, vdom: &dodrio::Vdom) {
    //Player1 on machine1 have a button Ask player to play! before he starts to play.
    //Click and it sends the WsMessage want_to_play. Player1 waits for the reply and cannot play.
    //Player2 on machine2 see the WsMessage and Accepts it.
    //It sends a WsMessage with the vector of cards. Both will need the same vector.
    //The vector of cards is copied.
    //Player1 click a card. It opens locally and sends WsMessage with index of the card.
    //Machine2 receives the WsMessage and runs the same code as the player would click. The cardgrid is blocked.
    //The method with_component() needs a future (promise) It will be executed on the next vdom tick.
    //This is the only way I found to write to CardGrid fields.
    let weak = vdom.weak();
    let msg_recv_handler = Box::new(move |msg: JsValue| {
        let data: JsValue =
            Reflect::get(&msg, &"data".into()).expect("No 'data' field in websocket message!");

        //serde_json can find out the variant of WsMessage
        //parse json and put data in the enum
        let msg: WsMessage =
            serde_json::from_str(&data.as_string().expect("Field 'data' is not string"))
                .unwrap_or_else(|_x| WsMessage::ConnectionTest {
                    test: String::from("error"),
                });

        //match enum by variant and prepares the future that will be executed on the next tick
        match msg {
            WsMessage::ConnectionTest { test } => console::log_1(&test.into()),
            WsMessage::WantToPlay { ws_client_instance } => {
                wasm_bindgen_futures::spawn_local(
                    weak.with_component({
                        let v2 = weak.clone();
                        move |root| {
                            let cg = root.unwrap_mut::<CardGridRootRenderingComponent>();
                            if let GameState::Start = cg.game_state {
                                console::log_1(&"rcv wanttoplay".into());
                                cg.game_state = GameState::Asked;
                                cg.other_ws_client_instance = ws_client_instance;
                                v2.schedule_render();
                            }
                        }
                    })
                    .map_err(|_| ()),
                );
            }
            WsMessage::AcceptPlay {
                ws_client_instance,
                card_grid_data,
            } => {
                wasm_bindgen_futures::spawn_local(
                    weak.with_component({
                        let v2 = weak.clone();
                        move |root| {
                            console::log_1(&"rcv AcceptPlay".into());
                            let card_grid = root.unwrap_mut::<CardGridRootRenderingComponent>();
                            //if let GameState::Asking = cg.game_state {
                            card_grid.players_and_scores.player_turn = 1;
                            card_grid.game_state = GameState::Play;
                            let v: Vec<Card> = serde_json::from_str(card_grid_data.as_str())
                                .expect("Field 'text' is not Vec<Card>");
                            card_grid.vec_cards = v;
                            card_grid.other_ws_client_instance = ws_client_instance;
                            v2.schedule_render();
                            //}
                        }
                    })
                    .map_err(|_| ()),
                );
            }
            WsMessage::PlayerClick {
                ws_client_instance,
                card_index,
                count_click_inside_one_turn,
            } => {
                wasm_bindgen_futures::spawn_local(
                    weak.with_component({
                        let v2 = weak.clone();
                        console::log_1(&"player_click".into());
                        move |root| {
                            let cg = root.unwrap_mut::<CardGridRootRenderingComponent>();
                            //rcv only from one other player
                            if ws_client_instance == cg.other_ws_client_instance {
                                console::log_1(&"other_ws_client_instance".into());
                                cg.count_click_inside_one_turn = count_click_inside_one_turn;
                                if count_click_inside_one_turn == 1 {
                                    cg.card_index_of_first_click = card_index;
                                } else if count_click_inside_one_turn == 2 {
                                    cg.card_index_of_second_click = card_index;
                                } else {
                                    //nothing
                                }
                                cg.card_on_click();
                                v2.schedule_render();
                            }
                        }
                    })
                    .map_err(|_| ()),
                );
            }
            WsMessage::PlayerChange { ws_client_instance } => {
                wasm_bindgen_futures::spawn_local(
                    weak.with_component({
                        let v2 = weak.clone();
                        move |root| {
                            let cg = root.unwrap_mut::<CardGridRootRenderingComponent>();
                            //rcv only from other player
                            if ws_client_instance == cg.other_ws_client_instance {
                                console::log_1(&"PlayerChange".into());
                                cg.take_turn();
                                v2.schedule_render();
                            }
                        }
                    })
                    .map_err(|_| ()),
                );
            }
        }
    });

    //magic ??
    let cb_mrh: Closure<Fn(JsValue)> = Closure::wrap(msg_recv_handler);
    ws.set_onmessage(Some(cb_mrh.as_ref().unchecked_ref()));

    //don't drop the eventlistener from memory
    cb_mrh.forget();
}
//endregion
