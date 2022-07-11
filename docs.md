# Concepts
 - Each instruction handler will use a global token to decode constants and info from the stack
 - The token will be changed every time an instruction gets executed.

# The Main Issue
The current issue is every single instruction doesn't know where it is in the grand scheme of things, it doesn't know what index of instruction it is in order to reference the constant list. 
Solutions:
 - A global bitfield which represents the number of instructions which have already been executed, an instruction handler can reference this to figure out their position.
 - When jumping, figure out the instructions we jump over and calculate a random offset combined by the instrs we skipped. Every opcode generates their own token offset.

# Or.. Instruction handling
There are two ways we can spawn instruction handlers to execute the actual vm.
Spawn an instruction handler for every single instruction.
 - Pros: 
    - It would be somewhat easy to implement for me, as I don't need to keep some kind of instruction state secure
    - A lot better security, the random offset idea would work extremely well because an instruction handler will always know where it is in the context of the script
    - Could do some crazy jumping around functions. Also fake instructions.
 - Cons: 
    - Massive script sizes for big projects, we're talking ~37,000 lines for a rerubi obfuscation (formatted). 

Spawn an instruction handler for every single opcode used, so a maximum of 38.
 - Pros:
    - Smaller script size
    - Already a proved formula, many obfuscators do this
 - Cons:
    - Not great security. I'm gonna have to figure out a way for instruction handlers to tell where they are, and global state is always sooooo hookable.