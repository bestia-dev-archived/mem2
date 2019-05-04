Things are changing fast. This is the situation on 2019-04-16. Luciano Bestia  
# mem2

Learning to use Rust Wasm/WebAssembly with Dodrio Virtual Dom and WebSockets on a simple memory game for kids - second iteration.  
  
You can play the game here:  
https://bestiavm02.southeastasia.cloudapp.azure.com  
Warning: Sometimes the server is down, because I use it for development. But if you contact me, I will be happy to start it.  

The backend Rust http + WebSocket server code is here:  
https://github.com/LucianoBestia/mem2_server  
 
## Build
Run in mem2/ folder  
```
wasm-pack build --target no-modules  
```
## Serve
Clone and `cargo run` the mem2_server from here:  
```
https://github.com/LucianoBestia/mem2_server  
```  
The server will print the External IP Address e.g. 192.168.0.22  
Open your browser and use that address.  
The game is made for exactly 2 players. Open 2 browser windows with the same address.  
Preferably on 2 smartphones on the same WiFi network.  
  
The frontend files are all in the folder mem2/.  
You can replace them eventually with the new version built with wasm-pack.  
  
# Memory game rules
This game is for exactly 2 players.  
The first player clicks on "Want to play?" and broadcasts the message over WebSocket.  
Player2 then sees on the screen a "Accept the game" link, clicks it and sends the message to Player1.  
The game starts with a grid of 8 randomly shuffled card pairs face down - 16 cards in all.  
On the screen under the grid are clear signals which player plays and which waits.  
Player1 flips over two cards with two clicks.  
If the cards do not match, the other player clicks on "Take your turn" and both cards are flipped back face down. Then it is his turn and he clicks to flip over his two cards.  
If the cards match, they are left face up permanently and the player receives a point. He continues to play, he opens the next two cards.  

# Upgrades, refactoring and enhancement over mem1
I decided that the project "mem1" is good as it is.  
It is a tutorial how to create a simple wasm with Rust Wasm/WebAssembly with Dodrio Virtual Dom and turn it into a webpage, electron and PhoneGap app. Very multiplatform !  
Adding anything more would make it difficult to understand and to follow the code.  
  
That is why I started a second project "mem2". I continue on the foundation of mem1 and will add stuff.  
Hopefully more advanced and interesting.  
I built a 2 player mode over WebSockets. With lot of refactoring and enhancements to make the code more Rust idiomatic. I added image transitions and sounds. All 100% Rust code. I learned to use Clippy and the Browser F12 Console. The html+JavaScript+css part didn't change much. It is just "boilerplate code".  
I opened an account on Azure and create a Linux Virtual Machine to host the game server mem2_server. I learned how to build with Rust and Warp a http + WebSocket server that listen on the same port.  
I learned a lot!  
And there is more to learn. The parts of Rust that are very different from other languages are the toughest. A totally new way of thinking.  

## TODO:
- how to create documentation from code comments? Now it looks awful. This readme.md look fine. It would be nice to have in the documentation, but how?  
- serde_json can recognize msgs types and then return different structs. Then I can use pattern match.  
- the performance is sometimes very strange. From click on one client to the event on the other client takes time. Even from the click locally to the sound locally is sometimes very slow.  
- understand how to use cache for vdom  

## References
The first iteration with all the instructions like a tutorial is here:  
https://github.com/LucianoBestia/mem1  
  
Rust  
https://doc.rust-lang.org/book/  
https://rust-lang-nursery.github.io/rust-cookbook/  
virtual dom  
https://github.com/fitzgen/dodrio  
web, http, css  
https://github.com/brson/basic-http-server  
https://www.w3schools.com/w3css/  
WebSocket  
https://ws-rs.org/
https://github.com/housleyjk/ws-rs  
wasm, wasm-bindgen  
https://rustwasm.github.io/docs/wasm-bindgen  
https://github.com/anderejd/wasm-bindgen-minimal-example  
https://github.com/grizwako/rust-wasm-chat-frontend  
JsValue, future, promises  
https://crates.io/crates/wasm-bindgen-futures  
https://github.com/fitzgen/dodrio/blob/master/examples/todomvc/src/router.rs  
random  
https://rust-random.github.io/book/  
Images included free cartoon characters:  
https://vectorcharacters.net/alphabet-vectors/alphabet-cartoon-characters  
Favicon from  
https://www.favicon-generator.org/search/BLACK/M  

