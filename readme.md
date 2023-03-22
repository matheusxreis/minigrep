# minigrep

### About

A simple grep tool, from chapter 12 of the Rust Book with some modifications. 

This cli program filter in files by lines that contains determinated word.

### How to run 

```bash 
    cargo run [WORD] [FILENAME] --[PARAMS]
    ex: 
        cargo run nobody poem.txt
        cargo run nobody poem.txt --case_sensitive save=output.txt

        ### --case_sensitive param filter by case_sensitive
        ### --save param saves the result in another file
```

You also can run all unit tests:

```bash
    cargo test
```

That's all folks! Never stop learning! :metal: