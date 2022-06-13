# The Concept
A lua vm obfuscator (obf) that addresses the issue of constant dumping. As I remember it, many obfs see constant dumping as a non-threat and focus on full deobfuscation (deobf) prevention. This obf will attempt to properly address constant dumping by making a constant dump require a near full deobf of the entire system. Fully deobfing your code is still possible, but writing tools to deobf every single wanderer encrypted script is extremely hard.

The main way that I see this being accomplished is by using the ideas below, and by "sharding" vm data all throughout the end result. Essentially, a constant dump won't require printing some table, or break pointing a couple of encryption functions, instead requiring to make annotations in every single handler, or something near that scale.

# The Ideas
 - The constants should never be held in one table in the memory. Instead, every single handler should grab its own constants from the giant bytecode string.
 - Additionally, the giant bytecode string should be completely scrambled, and every single handler needs to somehow find the right values in order to query it correctly in order to gain the constants and anything else.
    - These right values need to be changed every single time an instruction is run, so we can insert fake and pointless instructions in order to fight off any attacks that just simply analyze the changes in the right values.
  - Due to the previous reasons, the bytecode is probably secure enough and won't need like 999 layers of encryption lmao. If it isn't, we can probably do some encryption or something. 

## Extremely Big Ideas
 - Possibly use a different instruction set than lua 5.1
 - Possibly switch between instruction sets mid execution?
 - Wave function collapse algorithm to generate completely random handlers (this idea is kinda insane)
 - Handler web: jump around handlers in a web-like structure, good graph theory use case

# The Function
Let's describe the actual method we're going to be doing for changing the right values, and how we're going to be using them. If we look from the perspective of the lua vm and not the obfuscator, the process will be relatively simple. From now one, we will call the right values seeds. 

Some unpack function gets the beginning seed from some kind of bytecode. This part obviously doesn't need to be secure, but can be using conventional methods like encryption to make it even harder for attackers. The script then enters the first instruction handler. The handler will query constants with the special function, but this won't be the final step to get the constants. An attacker could still hook the special function. The handler will offset the result he got by some random value, which will give him the final constant. The handler must also calls to the special function that will be in unreachable positions or won't be used so that attackers can't just analyze the offsets by some simple regexes. Perhaps even include completely real handlers inside of other handlers that work with dummy values on the stack.

After that, the handler will offset the seed and the next handler will be found by the seed, repeating this process until all the bytecode is executed. Notice how at anytime, if the seed is offsetted wrong, the vm will crash itself or not work properly since it won't be getting the correct constants and other data. This means we can throw in fake handlers which will never run in order to throw off data flow analysis tools that process every single handler they see.

We need to create describe a function which meets our current goals for generating seeds to query bytecode with. It's requirements are:
 - Some seed x needs to be inputted
 - The function needs to be symmetrical so we can determine the right seeds during obf time.

There are already many contenders for this type of function, I'm thinking perlin noise, library of babel algorithm, or some kind of noise. Additionally, the function used can be one that's extremely unpopular and one that lacks proper documentation so that it's even harder to reverse. Finding a function like that shouldn't be so difficult considering the amounts of good psuedo-randomness in mathematics and what not.