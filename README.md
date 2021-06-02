# EZ Cloze
A command line tool for generating cloze deletions for Anki from a list of sentences.

## What is...
### A command line tool?
It's a program run using the command line or an equivalent program. Windows users will know this as `cmd`, or Command Prompt. You would run a program by opening up the command line, navigating to the folder the program is in, typing `program_name.exe`, and pressing enter.

### A cloze deletion?
From [Anki's website](https://docs.ankiweb.net/editing.html#cloze-deletion): "the process of hiding one or more words in a sentence". In other words, instead of a question in the format "What is CD's favorite color? Answer: blue.", you would instead have "\_\_\_\_\_ is CD's favorite color". Cloze deletions are excellent at providing context when learning through self-testing, which can greatly accelerate the learning process.

### Anki?
Anki is a program for spaced repetition. It's my personal favorite of the available tools for that, and one of my most commonly expressed regrets is that I don't make enough use of it. [https://apps.ankiweb.net/](https://apps.ankiweb.net/)

### Spaced repetition?
This is the idea of efficiently maintaining knowledge (that is, remembering stuff) through testing yourself repeatedly over time. However, due to the way memory works, you want to space out those tests further and further apart over time, provided you remember the items clearly, to maximize efficiency. This lets you spend less time trying to remember things and/or learn more things than otherwise. There are numerous methods and algorithms that can indicate when your next test for a particular item. I personally greatly prefer digital tools for this, which can not only use more complex and accurate algorithms without me needing to think about it, but can hold a huge amount of information and test me anywhere I have my phone or computer, or even a browser with an internet connection.

Note that spaced repetition is good for maintaining knowledge and studying, but is more inefficient for learning in most cases. If you are in school or otherwise learning anything, please help yourself out and make daily use of Anki. If you aren't learning anything, consider learning something interesting or useful, and see my previous sentence.

### A list of sentences?
A list of sentences is a list of sentences, or sentences in a list; namely, pairs of sentences between your original language and a target language. See below for specific formatting.

## Using EZ Cloze
### Installation
Compile using `cargo build [--release]` (with building for release being optional). `ezcloze.exe` (or equivalent) will be in `/target/release`. Alternatively, you can directly run the program with `cargo run [--release] (argument1) (argument2)` (described below).

### Running EZ Cloze
EZ Cloze takes two command line arguments. The first is an absolute (not relative) path to a tsv (tab-separated value) file that contains the sentences to apply cloze formatting to (see below).

### TSV Formatting
TSV files contain items separated by tabs. They are most easily generated in a spreadsheet program, most of which can output to TSV specifically.

The file should contain a minimum of two columns: pairs of sentences, one in the language to be cloze'd and one in the other language. Technically, this program works with just the former, but you'll likely want at least those two for Anki use.

My personal preferred format has six columns in this order: sentence, translated sentence, extra (for hints and notes), first language name, second language name (these two are so my Anki cards can display the two languages being used to prevent confusion), and tags.

### Output
The output will be another TSV file, with one new column appended to the end: the cloze text. This text contains cloze deletions for every word, and is suitable for any Anki card format that makes use of cloze deltions.

## Future Improvements
Planned improvements (in no particular order) include:
- Running without the command line
- Accepting CSVs and other formats
- Accepting lists of learned and unknown words, such that only learned words can be deleted in cloze output.
- Automatically removing column headers, to remove any manual steps for Anki import that relate to deleting those headers

Potential future improvements (in no particular order) include:
- A GUI (graphical user interface)
- Bulk import of sentences from specific formats

See the EZ Cloze project in the GitHub repository for future plans. Note that all milestone dates are currently estimates, and very much not set in stone.