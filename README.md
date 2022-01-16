# mca

This repository contains a compression algorithm written by
me ([Michael Grigoryan](https://github.com/michaelgrigoryan25/)).
The algorithm is only able to compress and decompress text files
and is not guaranteed to work with other file types.

The algorithm works best with repetitive texts.

## Explanation with an example

Suppose you have this repetitive text:

```txt
Nory was a Catholic because her mother was a Catholic,
and Nory’s mother was a Catholic because her father
was a Catholic, and her father was a Catholic because
his mother was a Catholic, or had been.
```

> Taken from https://thejohnfox.com/2021/08/17-fantastic-repetition-examples-in-literature/

As you can see the text is very repetitive. The compression
algorithm will loop through all the lines and the words in
them.

If a word is not present in the shared index and is used multiple
times throughout the text body, the algorithm will append that word
to the shared index. If the word is only used once, then it is added
directly to the compressed file, without being added to the shared index.

After the compression the file will produce a `compressed.mca`
file which will have the following content:

```txt
["Nory","was","a","Catholic","because","her","mother","Catholic,","and","father","or"]
0 1 2 3 4 5 6 1 2 7
8 Nory’s 6 1 2 3 4 5 9
1 2 7 8 5 9 1 2 3 4
his 6 1 2 7 10 had been.
```

The first line of the output contains the shared index. If you attempt
to decompress a file without this "header" the program will throw a
corruption error.

### Usage

You can start the program by running:

```bash
cargo run --release
```

after which you should get a prompt, asking
you to choose an action:

```txt
select an option: (c)ompress/(d)ecompress:
```

Both, compression and decompression are supported. Compressing a
file will create a file named `compressed.mca` and decompression
will output a file named `decompressed.txt`.
