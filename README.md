# What is this program?

I've been using a Mac for a little over two years now, and while I love almost everything about it,
one of the biggest pet peeves I have is just that; I don't know the intricacies of the system like I do Windows,
and Apple doesn't make it easy to find them.

Popular Unix peeking software works great on here, primarily Tree and HTOP. Problem is, there's a bunch of stuff
happening in the background that I just don't understand. That stuff leaves tons of files, cache entries,
and images on my system in order to give Mac that 'snappy feel'.

To hell with this; Even if I have to sacrifice a bit of that snap, I will create my own command line utils
for managing this computer.

# CacheInspect
First up, we have this folder which caches a bunch of 'stuff' which, I believe, makes second and beyond loads of that
subsequent asset significantly faster. Problem is; These cache entries are created upon first opening the program.

I believe this means (and my assuption may be wrong here), that downloading a program to try, and using it once
will generate one of the caches for that program. No good.

CacheInspect should simply give us insight into the Mac OSX application cache. I don't even really care about deleting
things while I'm there, just let me know if there's a bloated folder, and I'll go delete it myself.
