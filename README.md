# Mia Zip (Inspired by Miu)

> Note: Please disable Windows Real-Time Protection before running the program. Otherwise, the program will be 
> blocked for the iteration of files.

## About

This program will create a windows context menu action for ease of zipping. This will follow the naming format with 
the name being `mia_zip`.
Otherwise, you may use the `mia` command in the terminal.

Thank you for using!

## Installation
- Download the latest release from [here](https://github.com/Azuyamat/mia_rust/releases/latest)
- Extract the zip file
- Run `mia.bat`
- Follow the instructions on the screen (If any)
- Open a new terminal and run `mia` to start the program

## Usage

```shell
mia create <path> [name] [options]
```
Creates a zip file of the given path with the given name. If no name is given, `mia_zip` will be used.

**Options:**
- `-v` `--verbose` - Verbose output
- `-o <path>` `--out <path>` - Set the output path for the current creation
- `-d` `--default-output` - Set the output path to the default output path rather than the config value
- `-e <file name/folder name/extension>` `--exclude <file name/folder name/extension>` - Exclude a file/folder/extension from the zip file
- `-i <file name/folder name/extension>` `--include <file name/folder name/extension>` - Include a file/folder/extension from the zip file

```shell
mia config <set/add/remove/list> <key> <value>
```
Change config values.

**Keys:**
> Use `set`
- `naming` - Naming scheme for the zip file `:name` `:date`
- `output_dir` - Default output path for the zip file
> Use `add/remove`
- `blacklisted_file_names` - Blacklisted file names
- `blacklisted_folder_names` - Blacklisted folder names
- `blacklisted_file_extensions` - Blacklisted file extensions (Do not include the `.`)

```shell
mia update [version]
```
Update the program to the latest version. If a version is given, it will update to that version.

```shell
mia version
```
Get the current version of the program.

## Examples

```shell
mia create C:\code\fizzbuzz fizzbuzz -v
```
```shell
mia create C:\code\fizzbuzz fizzbuzz -v -o C:\code
```
```shell
mia create C:\code\fizzbuzz fizzbuzz -v -o C:\code -e exe -e fizzbuzz -e fizzbuzz2
```
```shell
mia config set naming :name
```

## License
[MIT](https://choosealicense.com/licenses/mit/)
