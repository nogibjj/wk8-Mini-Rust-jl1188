# wk8-mini-rust-jl1188:
A Rust Mini Project command-line tool that reverse the binary bits of an integer. 

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [rust-new-project-template](https://github.com/noahgift/rust-new-project-template)
* [rust-marco-polo-example](https://github.com/noahgift/rust-mlops-template/tree/main/MarcoPolo)

## Usage
<code>cargo run -- reverse --input 43261596</code>
 
 The command line tool uses the subcommand "<code>reverse</code>" and takes in one argument, a <code>u32</code> named "<code>input</code>," as the above examples shows. The result should be outputted as "Reversing the bits in the binary representation of 43261596  is: 964176192". In binary, the input is "00000010100101000001111010011100" and the output is "00111001011110000010100101000000".