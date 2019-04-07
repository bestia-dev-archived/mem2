Things are changing fast. This is the situation at 2019-04-07.
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
5. instead of having a string for src, use an usize as card number. Use Closure to format src string. 
6. instead of having a string for id, use an usize as index. Use inline bumpalo format format id string. 
7. Arrays and vectors are usually 0 based. For card number and card index I use base 1. The zero is facedown.
8. Added spelling for letters in the header. There is dual possibility: the header contains only the game title or two spellings. On the smartphone there is not enought space for all three.
9. Changed flex to CSS grid. It looks simpler.
10. Wanted to add the flip card transition. Failed. Don't know why in does not work well with flex or CssGrid. Now I use Opaciti transition and it looks ok.



## TODO
1. Get rid of bumpalo::format!(in bump, "xxx{}", "").into_bump_str() where is not needed.
2. Rand shuffle is deprecated. Find the new correct way.
3. build without --target no-modules ?
4. use clippy to avoid variable shadowing
5. how to create documentation from code comments?

## References
Favicon from https://www.favicon-generator.org/search/BLACK/M  

