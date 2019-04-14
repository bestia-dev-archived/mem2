Things are changing fast. This is the situation on 2019-04-14. Luciano Bestia
# mem2

Learning to use Rust Wasm/Webassembly with Dodrio Virtual Dom and WebSocket on a simple memory game for kids - second iteration. 
 
You can start the game here:  
https://lucianobestia.github.io/mem2_webpage/index.html

The first iteration with all the instructions is here:  
https://github.com/LucianoBestia/mem1  
 
## Build
Run in mem2/ folder
```
wasm-pack build --target no-modules
```
## Serve
Run the html server in mem2/ folder in a second terminal. 
So it can continuosly run while you incrementaly build your changes in the first terminal.  
It serves the html, js, css, wasm, img and mp3 static files.  
```
basic-http-server
```
If you don't have it yet on your machine, install it with:
```
cargo install basic-http-server
```
For websocket communication between 2 players you will need websocket_broadcast_simple_server. 
Run it in a third terminal. 
Clone the code  
`git clone git@github.com:LucianoBestia/websocket_broadcast_simple_server.git`   
and then   
`cargo run`  
or download the executable in Releases 
`https://github.com/LucianoBestia/websocket_broadcast_simple_server/releases/download/v0.1/websocket_broadcast_simple_server.exe`  
and run  
`websocket_broadcast_simple_server` 

Open the default URI in your browser
http://localhost:4000/  
For 2 players open 2 browser windows.  
Or open it on your 2 smartphones on the same wifi network.  
Change localhost with the IP address of your computer.  
# Memory game rules
The game starts with a grid of 8 randomly shuffled card pairs face down - 16 cards in all.  
The first player flips over two cards with two clicks.  
If the cards do not match, the next player starts his turn with a click to turn both cards back face down. Then two clicks to flip over two cards.  
If the cards match, they are left face up and the player receives a point and continues with the next turn. No additional third click needed in that case.  
This is a programming example for Rust Webassembly Virtual Dom application. 
For the sake of simplicity, it is made as for single player mode. 


# Upgrades, refactoring and enhancement of mem1
I decided that the project "mem1" is good as it is.   
It is a tutorial how to create a simple wasm with Rust Wasm/Webassembly with Dodrio Virtual Dom and turn it into a webpage, electron and PhoneGap app. Very multiplatform !  
Adding anything more would make it difficult to understand and to follow the code. 
That is why I started a second project "mem2". I continue on the foundation of mem1 and will add stuff.  
Hopefully more advanced and interesting.

## TODO
5. how to create documentation from code comments? Now it looks awful.
10. find a cheap virtual server to use for websocket server.


## References
https://doc.rust-lang.org/book/  
https://github.com/fitzgen/dodrio  
https://github.com/brson/basic-http-server    
https://rust-lang-nursery.github.io/rust-cookbook/    
https://github.com/anderejd/wasm-bindgen-minimal-example  
https://www.w3schools.com/w3css/  
https://ws-rs.org/
https://github.com/housleyjk/ws-rs
https://github.com/grizwako/rust-wasm-chat-frontend
https://crates.io/crates/wasm-bindgen-futures
https://github.com/fitzgen/dodrio/blob/master/examples/todomvc/src/router.rs (for vdom.with_component future) 
https://rust-random.github.io/book/  

Images included free cartoon characters:  
https://vectorcharacters.net/alphabet-vectors/alphabet-cartoon-characters  

Favicon from https://www.favicon-generator.org/search/BLACK/M  

