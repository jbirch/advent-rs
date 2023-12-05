Advent of Code 2023
===================

Stuff I'm actively figuring out
-------------------------------


Things that cropped up that I had to learn
------------------------------------------

Things that cropped up that I worked around, and need to learn more of
----------------------------------------------------------------------

[//]: #day-00 (TODO: I don't know how current_dir works in RustRover)
  But I did some dodgy shit to """fix""" it!

[//]: #day-01 (TODO: I need to upskill on generics, associated types, and traits)
[//]: #day-01 (TODO: I need to understand when to prefer &str over String and the implications of such)
[//]: #day-01 (TODO: I don't know the implications of Try, From, TryInto, and TryFrom)

Things that tangentially cropped up that I am so far ignoring, but it would be neat to know
-------------------------------------------------------------------------------------------

Day 00
------

* Set up IDE under Windows this time, to see what fresh hell awaits me.
* Turned on clippy's `-D clippy::pedantic` to see just how cooked last year's code was.
* It was cooked.
* Real cooked.
* I'm glad it's that cooked; there was a lot to pick up on.
* Copied vague structure from previous year.
* TODO: Figure out how IntelliJ wants to deal with `std::env::current_dir()` across:
  * Binaries nested in Workspaces (base dir)
  * Tests for binaries nested in Workspaces (workspace dir)
  * Documentation for libraries nested in Workspaces (workspace dir)
* learned about docstrings in a way that makes `clippy::pedantic` happy

Day 01
------

## Stuff that I started with, procrastinating opening the actual problem

* I tried rejigging the "read the whole file" helper into something that returns a bunch of lines. This was 
  straightforward enough when implementing the function, although the return type is incredibly more concrete than I 
  was expecting. However, writing a function that _took_ something that generically looks like "An iterable thing 
  that might give you strings", I ended up hitting somewhat of a wall when it comes to generics and traits.
* I understand _why_ I can't just write something like `fn derp(bonghits: Iterator<Result<&str>> -> ()`, when I 
  think about it. `Iterator` isn't Sized, `Iterator` has an associated `Item` type that has to be defined, etc. I 
  just don't think in that way when attempting to implement these things - hidden implied vtable shenanigans in 
  other languages has conditioned me to just be like "whatever give me whatever I can call a function on plz". Still,
  ending up with signatures like `fn derp<I, E>(bonghits: I) -> () where I: IntoIterator<Item = Result<String, E>>` 
  takes me by surprise.
* TODO: This means I should reread about generics and traits and associated types in the Rust book again.
* I'd also like to express this in terms of `&str`, which would involve having lifetimes associated with it. 
  Changing the signature of this function is fine, but I'd also need to change the signature of the utility function.
  I'm not sure why my brain says to prefer `&str` over `String`, but certainly the native `BufReader::lines()` talks 
  in terms of `String`, and so this might be the wrong choice to make. 
  * I can take something that has the ability to be a reference to a string - `AsRef<str>` - and then get the 
    reference that way. But now we end up with function signatures that are Godless and confounding. I don't 
    particularly like `fn derp<I, S: AsRef<str>, E>(i: I) -> () where I: IntoIterator<Item = Result<S, E>>`, and 
    again I'm wondering about the mixed location of qualification of generics. I can put `S: AsRef<str>` into the 
    where clause, but that feels... also crappy. I just want to say this thing is an Iterator over something that 
    can give you strings! I don't even do anything with E here - I just drop it!
  * [!!] This does allow me to operate over anything that can be reduced to a reference to `str` though, which is nice.
* Let's talk errors.
  * Attempting to turn the error from parsing into a possible error returned from the big function is 
    problematic.
  * I couldn't figure out how to make this work with some light poking at "construct an `anyhow::Result` from a 
    `std::error::Error`", so I went to look at the documentation for `anyhow`. It simply recommended letting `?` 
    perform the magic for me, which is awesome. That's what I want to see.
  * But I can't do this when mapping - there's no way to map some function that calls `?` because of something called 
    `std::ops::FromResidual<...>`. This is better than the information I was getting when writing the for loop 
    manually, which simply said something about my operation not being threadsafe. I guess ultimately the concept of 
    iteration says nothing about whether there's multiple threads going on, and whether the data being worked on is 
    threadsafe.
  * Quick googling of `FromResidual` seems to imply it's not what I was thinking of - I was thinking it meant "you 
    might not have completely consumed the thing you were working on, and that work might be happening in a way that 
    short-circuiting it is problematic, unless you add additional bounds to tell me it won't do that". However, it 
    looks like it might be some internal compiler concern, and the documentation of `Try::Residual` seems to hint at 
    why the error is inscrutable, but is similarly inscrutable itself.
  * [!!] `anyhow` mentions in its basic documentation that it operates similarly to boxing an error, except that it 
    requires that the error be `Send + Sync + 'static`. If I restrict my Error to also need these bounds, then 
    everything compiles as expected. It's not immediately clear what the consequences of this is, other than making 
    my function signatures ridiculous. Will need to study.
  * I also still don't know exactly how to write this in terms of `map`, but following the compiler errors led me to 
    `try_for_each`, which allows me to use `?` in the mapped function, at the cost of needing to return `Ok(...)` 
    from that function. This makes sense - the function itself needs to return a `Result` at that point, whereas 
    before there was no intermediate function to need to conform to. I think I prefer the manual iteration in this 
    case, but only because of some expectation that there's some wastage by needing to """return""" something 
    intermediate from the mapped function. I'm not sure to validate how this is or isn't stripped from the resulting 
    binary; it would be interesting to run this with `--emit mir` or `--emit llvm-ir` to see what happens.
  * I'd like to test by simply creating a `vec!(Ok(...), Ok(...), Err(...)`. Creating an `Err` on the fly like this 
    is annoying, so I attempted to use `Err(anyhow!("derp"))`, which promptly shat the bed. Apparently 
    `std::error::Error` isn't implemented by `anyhow::Error`. This makes [some amount of sense][anyhow_implementing],
    but I don't understand the syntax suggested in that comment. It seems to come down to "anyhow errors aren't 
    core errors, and so you can go from core errors to anyhow errors, but maybe not vice versa." There are 
    suggestions to use `thiserror` when creating adhoc errors for this case. I was hoping there'd be a nicer way.
  * Bounding on `Into<anyhow::Error>` doesn't work either, bringing back all the old issues about sized and whatnot. 
    Even re-adding these bounds (`Into<anyhow::Error> + Send + Sync + 'static`) runs back into the "residual" 
    problems from before.
  * TODO: This means I've got to read up on `From` and `Into` and `TryFrom` and `TryInto` as trait bounds.
  * [!!] I was confused with `?` having anything to do with errors. It doesn't. It's just "whatever the E is in 
    Result". Thus, I can remove the `E` bounds entirely, as long as I specify that the function returns 
    `anyhow::Result<..., E>` rather than simply `anyhow::Result<...>`. This makes the function signature way more 
    sane as a result.
  * Lordy I do not understand how `Ok(some_anyhow_error?)` ends up doing the magic. Normally this would be a 
    pointless conversion, except in this case it can promote a core `Error` into an anyhow `Error`, presumably 
    because there's a `From` implemented for `anyhow::Error`, but still, magic. And when it was a useless call, the 
    compiler complained at me, so the compiler is smart enough to know that `Ok(bonghits?)` may or may not be 
    tautological.


[anyhow_implementing]: https://github.com/dtolnay/anyhow/issues/63#issuecomment-581505403