# Instruction Offsets
The core VM inside of this obfuscator does not use the program counter. Instead, each instruction offsets the program token with a randon number thats unique for every single op code. This creates random looking numbers which are unique to a point of where a program currently is. You then use the instruction offset to decode a chunk of the bytecode. Therefore, all data is lazily unpacked which requires a lot more analysis to reverse. 

# Instruction format
All instructions take up 12 bytes in the bytecode. Each field (Opcode, A, B, C, Bx, sBx) has a size of 3 bytes, no matter how long it truly is in the vanilla 5.1 lua format.