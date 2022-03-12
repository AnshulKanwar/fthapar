# fthapar

A CLI tool written in rust that can be used to check if the password/pin for a particular enrollment number or a range of enrollment numbers on [Thapar Webkiosk](https://webkiosk.thapar.edu) is correct or not.

# ⚠️ Please use this tool responsibly

# Usage

To check the default password for 102016001
```shell
fthapar 102016001
```
To check a custom password
```shell
fthapar 102016001 -p "a_custon_password"
```
To check for a range of enrollment numbers
```shell
fthapar 102016001 --last 102016010
```

For more information try `fthapar --help`
