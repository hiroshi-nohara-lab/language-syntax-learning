Rust empowers you to reach farther, to program with confidence in a wider variety of domains than you did before.


For e.g, systems-level work that deals with low-level details of memory management data representation, and concurrency.


1. rustup is the how you can check whether rust is installed or not, for single file rust compilation or even for some smaller projects you can use ```rustc``` command, but for complex projects you will require something like cargo

2. cargo works as package manager, in case of cargo the package manager file is written in toml

3. ```cargo build ``` command builds the executable file, that you can fiind under target/debug folder generated when running this command

4. you can run ```cargo run``` to run the project without doing the above steps, becaues it does those step for you, also it checks whether to compile or not based on whether code changed or not

5. to only check (but not run) if the code will compile or not use ```cargo check```

6. when your project is finally ready for release, you can use ```cargo build --release``` more of a production build  


7.  always you have to update the cargo.toml whenever you are updating the version of a crate