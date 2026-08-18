# MIPS Simulator
This is a MIPS CPU simulator written in Rust. It reads a MIPS assembly file and executes it instruction by instruction, simulating the registers, memory, and control flow of a real MIPS processor. Rather than a simplified interpreter, it faithfully models real hardware behavior, the load-delay slot, signed vs. unsigned overflow trapping, word/halfword alignment restrictions, and label-based jumps and branches, the same subtleties an actual MIPS CPU has to handle.

The project centers on the core ideas of computer architecture: bits, bit patterns, operations on those patterns, and how they come to represent both instructions and data. Working through how assembly actually operates has given me a clearer picture of how a computer system behaves at the software level. It's also sharpened my own programming, I now understand, in a way most developers don't take the time to, what's actually happening underneath the code I write.

Read the full article on Medium: [Building a MIPS Simulator in Rust: How Assembly Works Under the Hood](https://medium.com/@naufal_fatihul).
