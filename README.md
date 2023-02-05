



### Creating a Project with Cargo

If you want to use it, (and not bother with code), use this command:

```
$ cargo new hello_cargo
$ cd hello_cargo
```

### Building and Running a Cargo Project
Now let’s look at what’s different when we build and run the “Hello, world!” program with Cargo! From your hello_cargo directory, build your project by entering the following command:

```
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs

```
This command creates an executable file in target/debug/hello_cargo (or target\debug\hello_cargo.exe on Windows) rather than in your current directory. Because the default build is a debug build, Cargo puts the binary in a directory named debug. You can run the executable with this command:
```
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!

```

If you don't have cargo, you can download the [latest release](https://github.com/Dhravya/readme-generator/releases)
This isn't recommended, because it won't be registered as a binary and you'll have to run the exe file

For contributing:

```
git clone https://github.com/Dhravya/readme-generator.git
cd Readme Generator
cargo install
```

### Usage
windows:
```

```
Linux:
```
#give it permission to run


