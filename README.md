# smokey

Program that allows training wpm from the comfort of a terminal.

[![asciicast](https://asciinema.org/a/419067.svg)](https://asciinema.org/a/419067)

## Program State

Project is in the early stages.
The basic functionality is there, but there are many missing features and rough edges.

Works fine on linux and propably on mac.
Works ok on windows too, but the cursor might be buggy sometimes.


## Giving it a go
standard rust stuff

```
git clone https://github.com/ukmrs/smokey && cd smokey
cargo run --release
```
## Actually Installing
### With cargo
```
cargo install smokey
```

### Building from source
```
git clone https://github.com/ukmrs/smokey && cd smokey
cargo build --release
```
then copy target/release/smokey to a known location

## Navigation

### Typing Test Screen

<table>
  <tr><th>Key</th><th>Function</th><tr>
  <tr><td>TAB</td><td>Reset the current test</td></tr>
  <tr><td>ESC</td><td>Open the settings</td></tr>
  <tr><td>CTRL + C</td><td>Exit</td></tr>
  <tr><td>CTRL + Backspace</td><td>Delete a word</td></tr>
</table>

### Settings Screen

<table>
  <tr><th>Key</th><th>Function</th><tr>
  <tr><td>TAB</td><td>Start a new test</td></tr>
  <tr><td>h j k l / Arrow Keys</td><td>Movement</td></tr>
  <tr><td>d / ESC</td><td>Deselect</td></tr>
  <tr><td>s / ENTER</td><td>Select</td></tr>
  <tr><td>q / ESC / CTRL + C</td><td>Exit</td></tr>
</table>

### Results Screen

<table>
  <tr><th>Key</th><th>Function</th><tr>
  <tr><td>TAB</td><td>Start a new test</td></tr>
  <tr><td>s</td><td>Open the settings</td></tr>
  <tr><td>q / ESC / CTRL + C</td><td>Exit</td></tr>
</table>

## word lists
Smokey ships with a sizeable english word list (~60_000 words) which on linux can be found in

~/.local/smokey/storage/words

Otherwise location can be found with the --storage flag.

More lists can be added to the folder. Smokey expects a list sorted by word frequency with each
word separated by a newline character.
Other languages are not provided but most of the time can be easily DIYed.

### Suggestions/Examples for word sources
#### French
Grab [Lexique382.zip](https://github.com/chrplr/openlexicon/blob/master/datasets-info/Lexique382/README-Lexique.md)
and unzip it to find Lexique382.tsv.
If you have [xsv](https://github.com/BurntSushi/xsv) installed,
here is an **almost one liner** to convert it to smokey-friendly format:

```bash
xsv sort -s freqlivres -N -R Lexique383.tsv | xsv select ortho > french
sed '1d' french > tmpfile && mv tmpfile french
```
The only purpose of the sed command is to delete the first line which will be "ortho" - the column name.
That could be done manually but I included it for convenience.

#### Polish

Grab [Otwarty słownik frekwencyjny leksemów](https://web.archive.org/web/20091116122442/http://www.open-dictionaries.com/slownikfrleks.pdf)
pdftotext it, sort it and clean it by a short script. Godspeed mój przyjacielu.

#### Supported languages/scripts
Smokey should handle all simple scripts like
- latin derivatives
- cyrylic
- greek
- etc

Complex scripts that require mulitple inputs for one glyph like *Hangul* won't work.
The same goes for right_to_left scripts.

### English word list
The list contains around 60 000 words.
It is derived from  [1/3 million most frequent English words](https://norvig.com/ngrams/count_1w.txt)
compiled by [Peter Norvig](https://github.com/norvig).
I filtred it using python bindings of [Enchant](https://abiword.github.io/enchant/)
and checked against the [MauriceButler/badwords](https://github.com/MauriceButler/badwords)
and  [LDNOOBW](https://github.com/LDNOOBW/List-of-Dirty-Naughty-Obscene-and-Otherwise-Bad-Words).
I kept "sex" though. Otherwise it wouldn't be fair to plants who
just cross-pollinate without causing too much of a ruckus.
Future me here, I forgot about accursed pollen allergies, I might reconsider my stance on this.

## Config
You can create smokey.toml configuration file that allows to
change colors or set default test settings. On linux:

~/.config/smokey/smokey.toml

Other OS:
```
smokey --config
```

### Example smokey.toml

For colors you can use either hex codes or standard colors
([supported names](https://docs.rs/tui/0.15.0/tui/style/enum.Color.html)).

```toml
[colors]
# test colors
todo = "grey"
done = "#96BB7C"
mistake = "#C64756"

# settings colors
active = "#93a1bf"
hover = "#aa78bf"

[test]
# default test settings
name = "english"
mods = ["punctuation", "numbers"]
len = 20
pool = 60000
```
