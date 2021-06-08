# smokey

Program that allows training wpm from the comfort of a terminal.

[![asciicast](https://asciinema.org/a/419067.svg)](https://asciinema.org/a/419067)

## Program State

Project is in very early stages.
The basic functionality is there, but there are many missing features and rough edges.

Works fine on linux and propably on mac.
Works ok on windows too, but the cursor might be buggy sometimes.


## Giving it a go
standard rust stuff

```
https://github.com/ukmrs/smokey && cd smokey
cargo run --release
```
## Actually Installing
### through cargo
```
cargo install smokey
```

### Building from source
```
https://github.com/ukmrs/smokey && cd smokey
cargo build --release
```
then copy target/release/smokey to a known location

## Navigation
### Typing test screen

TAB => reset test

ESC => settings

ctrl-c => exit

ctrl-BACKSPACE => del word

### Settings Screen

TAB => start test

hjkl | :arrow_left: :arrow_down: :arrow_up: :arrow_right:  => movement

d | ESC => deselect

s | ENTER => select

q | ESC | ctrl-c => exit

### Resutls Screen

TAB => new test

s => settings

q | ESC | ctrl-c => exit

## word lists
Smokey ships with a sizeable english word list (~60_000 words) which on linux can be found in

~/.local/smokey/storage/words

Other lists can be added to the folder. Smokey expects a list sorted by word frequency with each
word separated by newline character. Other languages are not provided but most of the time can be easily DIYed.
For example for polish, one could use [Otwarty słownik frekwencyjny leksemów](https://web.archive.org/web/20091116122442/http://www.open-dictionaries.com/slownikfrleks.pdf). It needs to be converted to text,
perhaps by unix pdftotext utility, and then sorted and cleaned by a short script.


### English word list
The list contains around 60 000 words.
It is derived from  [1/3 million most frequent English words](https://norvig.com/ngrams/count_1w.txt)
compiled by [Peter Norvig](https://github.com/norvig).
I filtred it using python bindings of [Enchant](https://abiword.github.io/enchant/)
and checked against the [MauriceButler/badwords](https://github.com/MauriceButler/badwords)
and  [LDNOOBW](https://github.com/LDNOOBW/List-of-Dirty-Naughty-Obscene-and-Otherwise-Bad-Words).
I kept "sex" though. Otherwise it wouldn't be fair to plants who
just cross-pollinate without causing too much of a ruckus. 
