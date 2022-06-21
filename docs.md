# The Concept
A lua vm obfuscator (obf) that addresses the issue of constant dumping. As I remember it, many obfs see constant dumping as a non-threat and focus on full deobfuscation (deobf) prevention. This obf will attempt to properly address constant dumping by making a constant dump require a near full deobf of the entire system. Fully deobfing your code is still possible, but writing tools to deobf every single wanderer encrypted script is extremely hard.

The main way that I see this being accomplished is by using the ideas below, and by "sharding" vm data all throughout the end result. Essentially, a constant dump won't require printing some table, or break pointing a couple of encryption functions, instead requiring to make annotations in every single handler, or something near that scale.

