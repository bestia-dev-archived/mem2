Things are changing fast. This is the situation at 2019-04-06.
# mem2

Learning to use Rust Dodrio Virtual DOM on a simple memory game for kids - second iteration. 
 
The first iteration with all the instructions is here:  
https://github.com/LucianoBestia/mem1  

# Upgrades, refactoring and enhancement of mem1
I decided that the project "mem1" is good as it is.   
It is a tutorial how to create a simple wasm with Rust ad Dodrio Virtual Dom and turn it into a webpage, electron and PhoneGap app.   
Adding anything more would make it difficult to understand and to follow the code. 
That is why I started a second project "mem2". I continue on the foundation of mem1 and will add stuff.  
Hopefully more advanced and interesting.

## Readability
1. I added `//region:` to the code, because I think it makes it more readable when the regions are folded in VS Code.  
2. The multiline strings I decided to put at the begining like this `static GAME_RULES: &str = ".."`;  
3. I was thinking to make the variable names shorter to make it more readable. I use really long names. But then the code would be difficult to understand. With shorter names it would be easier to read, but one must first learn what the short names mean. This is practical for the first coder, he knows all the names, but it is unpractical later for other coders to maintain.  
4. The Virtual Dom structure can be a big and deep tree. I try to avoid deep code nesting with Closures. I do it only for readability. It is similar to using functions, but it is more clear, that nobody else will call that functions. That code is very local to where is used.  

## TODO
1. Get rid of bumpalo::format!(in bump, "xxx{}", "").into_bump_str() where is not needed.
2. Rand shuffle is deprecated. Find the new correct way.
3. build without --target no-modules ?


