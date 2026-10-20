# deadlock-keyboard
* Pretty much just uses the Logitech LED SDK to change the lighting on my keyboard (or any rgb device i didn't specify the device type)
* for now it just reads the console's log in deadlock, it does this by launching the game with -condebug
* Features: hero specific lighting
    * Only heroes added are: graves, infernus, holliday, abrams, and celeste.

## Roadmap
Eventually i want to support reading the ingame values through reading deadlock's memory, I have an implementation curerntly in place for that reading memory in src/deadlock/memory, but its not wired to the lighting system yet.