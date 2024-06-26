# Getting the most out of your IDE

All code in this book assumes that you use a simple terminal to build your code,
run it, and interact with it. It also makes no assumption about your text editor.

However, you may have your favourite IDEs, providing you auto-complete, type annotation,
your preferred shortcuts and much more. This section explains how to get the most out
of your IDE using the code obtained from this book's repo.

# IDE configuration

Below, we explain how to configure your IDE to get the most out of this book.
If your IDE is not listed below, please improve this book by adding a section, so that the next
reader can get the best experience out of it.

## How to build with IntelliJ

When editing the IntelliJ build configuration, here are a few non-default values:
* You should edit the command. When this book tells you to run `cargo embed FLAGS`,
You'll need to replace the default value `run` by the command `embed FLAGS`,
* You should enable "Emulate terminal in output console". Otherwise, your program will fail to print text to a terminal
* You should ensure that the working directory is `microbit/src/N-name`, with `N-name` being the directory of the chapter you
are reading. You can not run from the `src` directory since it contains no cargo file.
