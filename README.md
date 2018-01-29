# L_KaranProjects_Rust_TextEditor

## Introduction:
This is a basic text editor written in rust.

## Functionality:
The text editor has just a few basic functions right now:

- Appending lines to the end of the document
- Quitting the text editor
- Printing the document text
- Editing text in the document

## Installing:
Clone the repo to your local machine.
Make sure the you have rustc and cargo Rusts package manager installed.

### Cargo build:
for building the binary.

### Cargo run
for running the debugging version.

## Using:
Upon opening the editor you'll be presented with an empty prompt.
text or commands can be provided here at the prompt.

Typing in text and pressing enter will append that text to the end of the file.

### Commands:
- :p will print the file to the screen.
- :q will exit the file
- :e will begin the editing of a file
	* after entering the command you will be prompted for a line number to edit. It's recommended you first print the file to see the lines
	* The program will then print the line out that you are editing. If you change your mind copy paste the text back into the console
	* enter will submit the changes and you will be taken back to the original console for appending.
- :w will begin writing the file to disk.
	* After entering the command you will be prompted for a file path.
	* enter the file path exactly correctly followed by enter.

Currently the program lacks a way to save to disk.


