# Hello new rust!

I noticed glitch has a really outdated rust version, so I decided to do something about it.

This project allows you to run up-to-date rust apps with minimal effort, however, they will take
a while to start up whenever they are shut down due to inactivity.

In this example, we are running an async Rust🦀 program that serves a simple .http and .html combo.
It is now interactive, that would go beyond the scope of this demo.

## Awesome, how do I use it?

You can put any rust project you want in the rust_app/ directory. It will then be run using the
latest stable rust release.

## But it's so slow!

Once you're done working with it, simply change the value in run-only.bool to true and it will 
stop recompiling every time. When you want to continue work on it again, simply set it to false 
again.

## It doesn't update when I change code

This is because of a glitch quirk. Simply change package.json in a minor but valid way to trigger
recompilation. I prefer simply adding a space to the end of the file to recompile, and removing it
when I want to recompile the next time.
