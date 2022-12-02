Advent of Code 2022
===================

Things that cropped up that I had to learn
------------------------------------------

* You [can't do workspaces of workspaces yet][ws-of-ws].
* It's possible to have no default executable for a Cargo project — Just don't have a `main.rs`.
* It's not possible to `#[derive(Error)]` natively, but there are [some][error-1] [crates][error-2] [for][error-3] 
  [it][error-4] (in order of popularity).
* `Result::expect` needs a `self` that is `fmt::Debug`, because it will include the representation of `self` in the 
  resultant panic. So at the very least, sentinel errors require `#[derive(Debug)]`.
* There doesn't seem to be much use in `#[derive(Default)]` for a struct with no members, but you can do it!
* Turn `Option` into `Result` with `ok_or(self, err: E)` or `ok_or_else(self, err: F)` (`FnOnce` and stuff) for 
  laziness.
* Turn `Option<&...>` into `Option<...>` with `cloned()`.

Things that cropped up that I worked around, and need to learn more of
----------------------------------------------------------------------

* Lifetimes of structs that contain a `&str`. Specifically in the context of having a `MyError<'a>` containing an 
  `&'a str` member.
* Is it better to parameterise the world on `Box<dyn std::err:Error>`, or deal with concrete `MyError`?
* Relatedly: How does the Rust ecosystem feel about sentinel errors? I'm drawn to them, but only because they're 
  about the best you can do in Golang. Is there a better way forward in Rust?
* File input is probably worth doing for this task, and not just copy-pasting my input into a giant static constant.
* Some things I want will return references, but I find myself choosing to implement concrete types. The moment I 
  return references, I need to care about lifetimes. But if I have a Result/Box/Option of a reference, I'd like to 
  turn what's inside into an owned copy. Or I annotate with `<'a>` wherever the compiler tells me.


Things that tangentially cropped up that I am so far ignoring, but it would be neat to know
-------------------------------------------------------------------------------------------

* [Documentation as tests][doctests].

Day 01
------

Train of thought:

* If I can take this flattened list and turn it into a list of lists, the inner lists can be reduced into a single 
  number, and then we can take the maximum.
* Though part of my mind wonders about parallelism of each elf for the luls, it's likely that accruing on a single 
  traverse is faster than slurping the whole thing and then summing. It also doesn't fucking matter, because there's 
  only a few thousand lines.
* So let's go with a dumbass solution first instead of some amazing functional masterpiece.
* I'll need to process the input line by line (Is there some kind of `split` I can use on `String`/`&str`?).
* I'll need to accrue into a growing vector, because I won't know up front how many elves there are. All hail The Heap.
* I have no idea how to have a `Vec<i32>`, and then reverse/take3. I can reverse in place and split a slice, but 
  accruing some `Take<Rev<...>>`, I'm not sure how to coalesce this into a type. `collect()` will give me references.
* Don't forget to read questions carefully — part two wanted the _sum_ of the top three, not the top three.


[doctests]: https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html
[error-1]: https://github.com/dtolnay/thiserror
[error-2]: https://github.com/JelteF/derive_more
[error-3]: https://github.com/rushmorem/derive-error
[error-4]: https://gitlab.com/torkleyy/err-derive
[ws-of-ws]: https://github.com/rust-lang/cargo/issues/5042