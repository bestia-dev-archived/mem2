# mem2 changelog
2019-04-07  
- Readability - I added `//region:` to the code, because I think it makes it more readable when the regions are folded in VS Code. Very nice.  
- Readability - The multiline strings I decided to put at the begining like this `const GAME_RULES:&'static str = ".."`  
- Readability - I was thinking to make the variable names shorter to make it more readable. I use really long names. But then the code would be difficult to understand. With shorter names it would be easier to read for me, but one must first learn what the short names mean. This is great for the first coder, he knows all the names, but it is unpractical later for other coders to maintain.  
- Readability - The Virtual Dom structure can be a big and deep tree. I try to avoid deep code nesting with Closures. I do it only for readability. It is similar to using functions, but it is clear, that nobody else will call that functions. That code is very local to where is used.  
- Readability - I don't understand Closures enough. So I rewrite them into private functions. I discovered, that private functions don't exist inside `impl` block. Surprising. And I don't know how to return Closures from function - it was just an atempt
2019-04-08  
- Refactoring - instead of having a string for src, use an usize as card number. Use Closure to format src string.  
- Refactoring - instead of having a string for id, use an usize as index. Use inline bumpalo format to format id string.  
- Refactoring - Arrays and vectors are usually 0 based. For card number and card index I find it more practical to use base 1. The zero is reserved for card face down.   
- Refactoring - Changed flex to CSS grid. It looks simpler.
2019-04-09  
- Enhancement - Added spelling for alphabet letters in the header. There is dual possibility: the header contains only the game title or two spellings. On the smartphone there is not enought space for all three.  
- Enhancement - Added morse audio with inline javascript `var audio = new Audio('content/sound/mem_sound_{:02}.mp3');audio.play();`  I asked the community if that can be achieved with WebAudio.  
- Enhancement - Wanted to add the flip card transition. Failed miserably. Don't know why it does not work well with flex or CssGrid when there is more than one row. Some problems with absolute position. Now I use Opacity transition and it looks quite ok.  
- Enhancement - When cards match change color to green, when don't match to red.  
- Enhancement - Audio play now from rust with HtmlAudioElement.   
2019-04-12
- Enhancement - Added Websocket communication. I use the ws-server from ws-rs as simple echo server. In lib.rs added the connection, send and receive msg callback. The message is saved in CardGrid field. That is used to write it in vdom.  
2019-04-15  
Built a special server with http+websocket on the same port: mem2_server  
2019-04-16  
Socket address is the same as the http address.  
2019-05-04  
Using serde-json to match the right enum variant for websocket message.  
