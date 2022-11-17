# tuemensa

tuemensa is a simple cli tool to retrieve the current meal plan.

## Usage

Options:

- `-m`, `--morgenstelle`  Show Mensa Morgenstelle
- `-s`, `--shedhalle`     Show Mensa Shedhalle
- `-p`, `--plaintext`     Format as plain text
- `-o`, `--oneline`       Use very short format (oneline)
- `-d`, `--days <DAYS>`   Offset of days in the future (valid inputs 0-7) [default: 0]
- `-v`, `--vegetarian`    Show the vegetarian menu
- `-h`, `--help`          Print help information
- `-V`, `--version`       Print version information

### Examples

Display the current plan for both canteens:

```sh
tuemensa -s -m
```

Display the plan for the next day:

```sh
tuemensa -s -d 1
```

The oneline is useful if you integrate this tool as a desktop widget
and like to get just a basic idea of the current available meal.

```sh
tuemensa -s -o
```

![CommandOutput](screenshots/kde_command_output.png)

For KDE this can be achieved with the Plasma 5 Applet [Command Output](https://store.kde.org/p/1166510/).

## Build

Use the `cargo build -r` command inside the root of this project.
