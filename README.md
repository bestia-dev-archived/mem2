Things are changing fast. This is the situation on 2019-05-10. Luciano Bestia  
Read the `Last project` first:  
https://github.com/LucianoBestia/mem1  
# mem2
Learning to use Rust Wasm/WebAssembly with Dodrio Virtual Dom and WebSockets communication - second iteration.  
This is a simple memory game for kids. The images are funny cartoon characters from the alphabet.  
The cards grid is only 4x4.  
For fun I added  
- the sounds of Morse alphabet codes and  
- the International Aviation spelling.  
  
You can play the game here:  
https://bestiavm02.southeastasia.cloudapp.azure.com  
Warning: Sometimes the server is down, because I use it for development. But if you contact me, I will be happy to start it. I can do it basically anywhere. On my android phone I have the Azure app and the ConnectBot app as SSH console with my private SSH key.  

## Build
Clone and build
```
git clone git@github.com:LucianoBestia/mem2.git
cd mem2
wasm-pack build --target web  
```
You cannot use this project without a html/WebSocket server. Read the next chapter.  
## Serve
The mem2_server project and instructions is here:  
https://github.com/LucianoBestia/mem2_server  
For development, you have to run the game from the mem2_server. So you will have both the server and client side working.  
Copy the `mem2/pkg/` folder to `mem2_server/mem2/pkg`. This is the compiled wasm code.  
After building and running the server,  
`cd mem2_server
cargo run`  
it will print the External IP Address e.g. 192.168.0.22  
Open your browser and use that address.  
The game is made for exactly 2 players. Open 2 browser windows with the same address.  
Preferably use 2 smartphones on the same WiFi network.  
# Memory game rules
This game is for exactly 2 players.  
Both players must have the webpage simultaneously opened in the browser to allow communication.  
To start over just refresh the webpage.  
The first player clicks on 'Ask Player2 to play?' and broadcasts the message over WebSocket.  
Player2 then sees on the screen 'Click here to Accept play!', clicks it and sends the message back to Player1.  
The game starts with a grid of 8 randomly shuffled card pairs face down - 16 cards in all.  
On the screen under the grid are clear signals which player plays and which waits.  
Player1 flips over two cards with two clicks.  
If the cards do not match, the other player clicks on 'Click here to Take your turn' and both cards are flipped back face down. Then it is his turn and he clicks to flip over his two cards.  
If the cards match, they are left face up permanently and the player receives a point. He continues to play, he opens the next two cards.  
The player with more points wins.  

# Upgrades, refactoring and enhancement over mem1
I decided that the project "mem1" is good as it is.  
It is a tutorial how to create a simple wasm with Rust Wasm/WebAssembly with Dodrio Virtual Dom and turn it into a webpage, electron and PhoneGap app. Very multiplatform !  
Adding anything more would make it difficult to understand and to follow the code.  
  
That is why I started a second project "mem2". I continue on the foundation of mem1 and will add stuff.  
Hopefully more advanced and interesting.  
Read the interesting StructModel.md to understand the basics of the new structs and data.  
I built a 2 player mode over WebSockets. With lot of refactoring and enhancements to make the code more Rust idiomatic. I added image transitions and sounds. All 100% Rust code. I learned to use Clippy and the Browser F12 Console. The html+JavaScript+css part didn't change much. It is just "boilerplate code".  
I opened an account on Azure and create a Linux Virtual Machine to host the game server mem2_server. I learned how to build with Rust and Warp a http + WebSocket server that listen on the same port.  
I learned a lot!  
And there is more to learn. The parts of Rust that are very different from other languages are the toughest. A totally new way of thinking.  
## VSCode
I use `//region:` and `//endregion` a lot. To Fold it and UnFold it press `F1` type `fold` and choose from a variety of options. Start with `Fold All`   
## TODO:
- how to create documentation from code comments? Now it looks awful. This readme.md looks fine. It would be nice to have it included in the documentation, but how? Look at dodrio. 
- use cache for players score  
- on iPhone Safari the local click plays the sound, but the WebSocket message does not play it. On android and windows (Chrome) it works for both events. 
- read text from server text.json file in Rust. Now it does it in JavaScript.  
- Restart button and re-ask player.
- where to choose the content folder?
## Next projects
Don't know yet.  
## References
Rust  
https://doc.rust-lang.org/book/  
https://rust-lang-nursery.github.io/rust-cookbook/  
virtual Dom  
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
Reference counting, Borrow Checker in runtime instead of compile time  
https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees/  


