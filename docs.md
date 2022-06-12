# The Concept
A lua vm obfuscator (obf) that addresses the issue of constant dumping. As I remember it, many obfs see constant dumping as a non-threat and focus on full deobfuscation (deobf) prevention. This obf will attempt to properly address constant dumping by making a constant dump require a near full deobf of the entire system. Fully deobfing your code is still possible, but maybe hard?

# The Ideas
 - The constants should never be held in one table in the memory. Instead, every single handler should grab its own constants from the giant bytecode string.
 - Additionally, the giant bytecode string should be completely scrambled, and every single handler needs to somehow find the right values in order to query it correctly in order to gain the constants and anything else.
    - These right values need to be changed every single time an instruction is run, so we can insert fake and pointless instructions in order to fight off any attacks that just simply analyze the changes in the right values.
  - Due to the previous reasons, the bytecode is probably secure enough and won't need like 999 layers of encryption lmao. If it isn't, we can probably do some encryption or something. 

## Extremely Big Ideas
 - Possibly use a different instruction set than lua 5.1
 - Possibly switch between instruction sets mid execution?
 - Wave function collapse algorithm to generate completely random handlers (this idea is kinda insane)
 - Handler web: jump around handlers in a web-like structer, good graph theory use case