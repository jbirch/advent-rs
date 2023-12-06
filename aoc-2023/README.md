Advent of Code 2023
===================

Stuff I'm actively figuring out
-------------------------------


Things that cropped up that I had to learn
------------------------------------------

Things that cropped up that I worked around, and need to learn more of
----------------------------------------------------------------------


[//]: #day-00 (TODO: Learn more how current_dir works in RustRover)

* RustRover's current_dir is fucked, but I did a dodgy to make it mostly work in most places.

[//]: #day-01 (TODO: Upskill on generics, associated types, and traits)

* I have to reread the TRPL on generics and stuff.

[//]: #day-01 (TODO: I need to understand when to prefer &str over String and the implications of such)

* `&str` vs `String` isn't natural for me yet, although `AsRef<str>` is super convenient for not caring.

[//]: #day-01 (TODO: Learn the implications of Try, From, TryInto, and TryFrom)

* `Try` and `From` are opposites of each other, and their "Try" versions can return Results. I couldn't use these
  effectively as trait bounds yet though, or to implement something.

Things that tangentially cropped up that I am so far ignoring, but it would be neat to know
-------------------------------------------------------------------------------------------

[//]: #day-01 (TODO: Learn more about thiserror and From)

* `thiserror` has annotations for doing magic things, and it would be cool to enhance the sentinel error I'm using
  here to do more ergonomic things, or to capture actual information about what's happening.

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

### Stuff that I started with, procrastinating opening the actual problem

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
    * [!!] This does allow me to operate over anything that can be reduced to a reference to `str` though, which is
      nice.
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
      suggestions to use `thiserror` when creating adhoc errors for this case. I was hoping there would be a nicer way.
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
    * [!!] Using `anyhow` for the library parsing bits was unnecessary. It's fine for application code where lots of
      different things might fail with different errors, and you want to write a function to call them all and be able
      to liberally use `?` yourself. But it's unnecessary for the code I wrote there; I know exactly which concrete
      error it is.
        * I wonder if this implies having library code that returns `Box<Error>` or something to mean it can die in many
          different ways is an antipattern, and that's why `thiserror` is more useful for playing around with those
          concrete `Error` structs.

### Actually doing the thing

* First thought is to chop left and right bits of each line, but this is annoying given all the values it could take.
* On the other hand, using a Regex would be annoying because it can also match a single digit in a line.
* So the really dumb approach is to drop characters from the left and right until a match happens, I suppose.
* The implementation seems straightforward, but now I'm mixing `Option` and `Result` with `?` and it doesn't like that.
* That's okay; I can still match manually, but it hints at why I couldn't mix: I have to explain how a `None` is
  supposed to turn into the error part of the Result.
    * This is problematic for me, as I tried to write the function super generally on a type parameter `E` that I
      never defined, which means I couldn't construct it even if I wanted to.
    * I can fix this by actually typing `E` to an `Error`, and if I want to be able to use `anyhow::Result` as a return
      value from this, or to use `bail!(...)` myself in the body, I'll need to make it an `Error` that anyhow can
      slurp up - namely `E: Error + Send + Sync + 'static`.
        * Doing this breaks my test in Day00 though: Because I'm constructing an `anyhow::Error` not a
          `std::error::Error`. I should go look at `thiserror`.
* Pulling in `thiserror` is straightforward. Using it for a sentinel error that I can pass in the test is less
  straightforward. I can put the annotation on an enum or a struct.
    * Putting it on an enum means I can't ever get a value of the thing - I can only really ever use it as a type.
      This is apparently termed an "uninherited type".
    * Putting it on a struct means I can construct it, as a zero-sized type. On the other hand, I need to implement
      `std::fmt::Display` for a struct, and it can't be derived. This is fine, but slightly annoying.
    * TODO: I should learn how to make a version of this can can be constructed from other things, perhaps as a
      sentinel error and also as a container of a string, and see if I can do `From` magic to see if it makes for
      something ergonomic.
* I'm not actually doing the thing yet hey.
* Wasn't hard, just had to look up string indexing shenanigans. Answer was correct for part one.
* Part two seems fucked at first glance, but I think Rust's Partition can also be a string to find.
* Given that this has a separate testing input, this ruins the assumption of "one test file per one real input file".
  Given that the test inputs are small, I'm wondering if I should just do the iterator over the lines in line in
  those tests, rather than opening a file. This would also let me delete all that custom `current_dir` manipulation
  code, at the cost of making the doctests not work in rustrover natively, without doing the setup inline each time.
  This is probably an okay compromise.
* So strings can be split by Regex, but this wouldn't let me *find* the first and last time this happens, it would
  give me what's on each side of these. I'd then have to re-inspect the string to see which thing specifically
  disappeared, and then match that.
* I could mutate the string to change all occurrences of words with the numeral, and then use the existing
  extraction function. This is pretty memory-churny, but is fast enough for a thousand lines.
* Or I can apply a regex to the line, and see if there's a way to capture first and last matching groups, and
  similarly map strings to numbers - assuming I can make the middle glob as greedy as possible, and the outer globs
  as not-greedy as possible.
* Option 2 seems the easiest, though nice and long with 10 searches over the whole damn line.
* Option 3 seems the most sane. I can match-and-capture everything that's an exact match, get an iterator over those
  matches, and then get the first and last ones, and map them back.
    * I think this means I'm about to learn about `lazy_static` or `OnceCell` or the new-in-1.70 `std::sync::Once::Lock`
* A little bit of dicking around with `Regex`, I got what I needed. It's still not clear exactly what the difference
  between `CaptureMatches` and `Matches` is (that is, when I'd call `Regex::captures_iter` over `Regex::find_iter`).
  Captures really sounded like what I wanted, but I guess I'm not actually capturing anything in my Regex, and so I
  only need to know what twigged the match, not the results of Regex capturing groups.
* However, while this passes the test, it doesn't pass the old data.
* Because some things have multiple matches.
* This got me an answer which was too high.
    * Because my matching is from the right, and thus is greedy when numbers overlap.
    * `twone` should match "two" from the left and "one" from the right, but it only ever matches and consumes "two".
    * Good test case, and it fails. One of the authors of `Regex` says [it can't be done with this crate]
      [burntsushi_overlapping_matches], so I might instead try two regexes - one to match first, the other to greedy
      match a glob and the last.
* While fucking around in the debugger in RustRover, I got frustrated by not being able to evaluate method calls.
  Turns out [this is a long-standing problem][no_trait_information] in the data emitted by `rustc`, and so [Jetbrains
  things can't access that which does not exist][jetbrains_doesnt_know_either] .
* I'm getting hung up on trying to capture `r".*(stuff).*"`, and having the _capture group_ (that is, not the match)
  return the whole string.
* I'll just start searching from each match all over again, until I find no more matches.
* This gets the correct answer.

Day 02
------


[anyhow_implementing]: https://github.com/dtolnay/anyhow/issues/63#issuecomment-581505403

[burntsushi_overlapping_matches]: https://stackoverflow.com/a/57497310

[no_trait_information]: https://github.com/rust-lang/rust/issues/33014

[jetbrains_doesnt_know_either]: https://github.com/intellij-rust/intellij-rust/issues/8672