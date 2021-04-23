# smokey

Program that allows training wpm from the comfort of one's terminal.
Project in very early stages, works properly on linux.

## Giving it a go
standard rust stuff

```
https://github.com/ukmrs/smokey && cd smokey
cargo run --release
```

### English word list
The list contains around 60 000 words.
It is derived from  [1/3 million most frequent English words](https://norvig.com/ngrams/count_1w.txt)
compiled by [Peter Norvig](https://github.com/norvig).
I filtred it using python bindings of [Enchant](https://abiword.github.io/enchant/)
and checked against the [MauriceButler/badwords](https://github.com/MauriceButler/badwords)
and  [LDNOOBW](https://github.com/LDNOOBW/List-of-Dirty-Naughty-Obscene-and-Otherwise-Bad-Words).
I kept "sex" though. Otherwise it wouldn't be fair to plants who
just cross-pollinate without causing too much of a ruckus. 
