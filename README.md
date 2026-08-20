# MIPS Simulator
This is a MIPS CPU simulator written in Rust. It reads a MIPS assembly file and executes it instruction by instruction, simulating the registers, memory, and control flow of a real MIPS processor. Rather than a simplified interpreter, it faithfully models real hardware behavior, the load-delay slot, signed vs. unsigned overflow trapping, word/halfword alignment restrictions, and label-based jumps and branches, the same subtleties an actual MIPS CPU has to handle.

The project centers on the core ideas of computer architecture: bits, bit patterns, operations on those patterns, and how they come to represent both instructions and data. Working through how assembly actually operates has given me a clearer picture of how a computer system behaves at the software level. It's also sharpened my own programming, I now understand, in a way most developers don't take the time to, what's actually happening underneath the code I write.

Note: In the future, I plan to extend this into a fuller assembler, adding more MIPS mnemonics, pseudo-instructions (like li, la, or move, which don't exist in real hardware but expand into one or more real instructions), and possibly macros, closer to what a production MIPS assembler (like MARS or SPIM) provides.

Read the full article on Medium: [Building a MIPS Simulator in Rust: How Assembly Works Under the Hood](https://medium.com/@naufal_fatihul/building-a-mips-simulator-in-rust-how-assembly-works-under-the-hood-f1c624bd1816).
