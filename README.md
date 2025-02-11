# custom-ls
- custom-ls is my personal flavor of best information and format to display contents of a given directory
- atm no flags are implemented
- recommended usage in `.bashrc`:
```
cd() {
    builtin cd "$@" || return  # Change directory and return if it fails
    ll
}
```
