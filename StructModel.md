# Struct model
I cannot say "Object model", because Rust does not have objects, but structs.  
  
## Game components
The game has visually few distinct vertical sections:  
1. Title OR Aviation spelling  
2. Card Grid (4x4 cards)  
3. Players score  
4. Click count  
5. Rules and descriptions  

I understand, that the idea is to have only one Virtual Dom.  
It must contain only one Root rendering component.  
The Root component can contain more subcomponents.  
A component is a struct with a Render Trait.  
All of that components need to have access to the game data.  
## Wrap it boy
Rust is all about wrappers.  
https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees/  
If you want to add something, very often you do it with a wrapper.  
If a variable is nullable in Rust we must wrap it in an Option struct.  
If we return a result that can be an error, we wrap it in an Result struct.  
If we need more references we wrap it in a ReferenceCount.  
If we need interior mutability, we wrap and wrap and wrap.  
For a beginner (me) all this wrapping looks scary.  
Because of this every change and refactoring is painful, because after wrapping, the interface of the object changes.  
But probably this is good and protect us from stupid errors.  
You just have to get used to it. Wrap you mind around that idea ;-)  
## Rc<>
I want all components to have access to the GameData. So one reference is not enough.  
I need to use a Reference Counter wrapper Rc<>. It is like a small garbage collector.  
Rc<> counts the number of references and Drops the data when the count comes to zero.  
But Rule No 1 of the Borrow checker is:  
1. you can have only one mutable reference.  
  
All the point of having an Rc<> is to have more References.  
The logical conclusion is that this references all must be immutable.  
Every time you need a new reference, make a Rc.Clone(). It gives the same reference, but it increases the interior Rc.counter field.
## RefCell
Rust has this idea of Interior mutability.  
The struct itself is not mutable, but the fields can be if you really want.  
Confusing is a mild word for this concept, but the rabbit hole goes deeper.  
So I could wrap the GameData inside a RefCell<> and then wrap it in Rc<>.  
I have seen this so many times in tutorials and examples and didn't know why it has to be so complicated.  
I hope I know now.  
BTW RefCell<> uses a small/efficient RunTime machine, that Checks the borrows at runtime.  
You can borrow and borrow_mut in runtime.  
Your borrows errors will now come out only at Runtime, because "Compile time BorrowChecker" is going bye-bye. And there will be errors, trust me.  
It is now something like a super simple runtime GarbageCollector.  
```
  -------------------------                               -------------------   
  |    RootComponent      |    Reference Counter          |                 |   
  |                       |   Interior mutability   |     |     GameData    |   
  |  ------------------   |                         |     |                 |   
  |  |                |   |                         |     |    - Players    |   
  |  |  Component1    | ---------------------->     |     |                 |   
  |  |                |   |                         |     |    - Score      |   
  |  ------------------   |                         |     |                 |   
  |                       |                         |     |    - Count      |   
  |  ------------------   |---------------> Rc<RefCell<>> |                 |   
  |  |                |   |                         |     |                 |   
  |  |  Component2    | ---------------------->     |     |                 |   
  |  |                |   |                         |     |                 |   
  |  ------------------   |                         |     |                 |   
  |                       |                         |     |                 |   
  |  ------------------   |                         |     |                 |   
  |  |                |   |                         |     |                 |   
  |  |  Component3    |------------------------->   |     |                 |   
  |  |                |   |                         |     |                 |   
  |  ------------------   |                         |     |                 |   
  |                       |                         |     |                 |   
  -------------------------                               -------------------   
```

