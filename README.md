# rspaint

A simple raster editor with a familiar interface.


### Compiling

Requires GTK, GDK and Cairo libraries to compile.

Compile using cargo with `cargo build`. 

Binary will exist at `./target/debug/rspaint`.

### Lessons learned

I chose this project because I wanted to learn about developing software with a 
GUI. Having no prior experience GUI programming with the GTK+ library, I 
spent considerable time reading library documentation. This was the largest 
library I've taught myself and the experience of having navigate
the documentation was invaluable.

I learned a lot about Rust's borrow rules and how its language features
interact with them. At first I found them to be frustrating and to be
holding back development, but as I grew more experienced the rules became
more intuitive and I grew to understand and appreciate the safety Rust's
static analysis offers. 

I learned about software architecture. The first working version
of rspaint was written without architecture in mind as I was more concerned with
learning the libraries and having a working program. However, as I went to add
more features I realized that the code I had written was completely unsuitable
for extending and thus I had to rewrite much of the code in order to decouple
the GUI from other aspects of the program.

