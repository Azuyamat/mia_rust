# Mia Zip (Inspired by Miu)

> Note: Please disable Windows Real-Time Protection before running the program. Otherwise, the program will be 
> blocked for the iteration of files.

## About

This program will create a windows context menu action for ease of zipping. This will follow the naming format with 
the name being `mia_zip`.
Otherwise, you may use the `mia` command in the terminal.

Thank you for using!

![GitHub release (with filter)](https://img.shields.io/github/v/release/Azuyamat/mia_rust?style=for-the-badge&logo=github)
![GitHub release (by tag)](https://img.shields.io/github/downloads/Azuyamat/mia_rust/latest/total?style=for-the-badge&logo=github&label=Downloads&color=FFFFFF)



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
mia create D:\code\fizzbuzz fizzbuzz -v
#Output: Zipped 18 files in 1850ms (222 lines)
```
```shell
mia create D:\code\fizzbuzz fizzbuzz -v -o D:\code
#Output:
#--------------------------------------
#Zipping: "mia_zip_2023-11-22_2346922.zip"
#Output: "D:\\code\\mia_zip_2023-11-22_2346922.zip"
#Excluding: [["zip", "pdf"], [], [".git", "bin", "obj", ".idea", ".vs", "target", "node_modules", ".idea", ".next"], []] (Use --exclude or -e)
#Including: [] (Use --include or -i)
#--------------------------------------
#[FILE] + ".eslintrc.json" (3 lines)
#[DIR] / "D:\\code\\nextjs\\revolv\\.git"
#[FILE] + ".gitignore" (28 lines)
#[DIR] / "D:\\code\\nextjs\\revolv\\.idea"
#[DIR] / "D:\\code\\nextjs\\revolv\\.next"
#[FILE] + "next-env.d.ts" (4 lines)
#[FILE] + "next.config.js" (5 lines)
#[DIR] / "D:\\code\\nextjs\\revolv\\node_modules"
#[FILE] + "package-lock.json" (4341 lines)
#[FILE] + "package.json" (27 lines)
#[FILE] + "postcss.config.js" (6 lines)
#[FILE] + "public\\favicon.ico"
#[FILE] + "public\\next.svg" (1 lines)
#[FILE] + "public\\vercel.svg" (1 lines)
#[DIR] + "D:\\code\\nextjs\\revolv\\public"
#[FILE] + "README.md" (25 lines)
#[FILE] + "src\\pages\\api\\hello.ts" (11 lines)
#[DIR] + "D:\\code\\nextjs\\revolv\\src\\pages\\api"
#[FILE] + "src\\pages\\index.tsx" (111 lines)
#[FILE] + "src\\pages\\_app.tsx" (5 lines)
#[FILE] + "src\\pages\\_document.tsx" (12 lines)
#[DIR] + "D:\\code\\nextjs\\revolv\\src\\pages"
#[FILE] + "src\\styles\\globals.css" (24 lines)
#[DIR] + "D:\\code\\nextjs\\revolv\\src\\styles"
#[DIR] + "D:\\code\\nextjs\\revolv\\src"
#[FILE] + "tailwind.config.ts" (19 lines)
#[FILE] + "tsconfig.json" (22 lines)
#Zipped 18 files in 20ms (222 lines)
#--------------------------------------
#TypeScript: 162 lines (72.97%)
#Other: 4423 lines
#CSS: 24 lines (10.81%)
#JavaScript: 11 lines (4.95%)
#Markdown: 25 lines (11.26%)
#--------------------------------------
```
```shell
mia create C:\code\fizzbuzz fizzbuzz -v -o C:\code -e exe -e fizzbuzz -e fizzbuzz2
#Output: Similar to above
```
```shell
mia config set naming :name
#Output: Successfully changed key `naming` to `:name`
```

## License
[MIT](https://choosealicense.com/licenses/mit/)
