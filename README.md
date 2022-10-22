# pam-template
PAM development made easy!

## Prerequisites
[just](https://github.com/casey/just)

libpam library (Ubuntu):
```
sudo apt-get install -y build-essential libpam0g-dev
```

## Installing
to install from source, run:
```
git clone https://github.com/abrandec/pam-template
just install
```

## Using
creating a PAM project:
```
pam-template create project-name
```
**Warning**
project names must contain a '-' inbetween two charaters from the English alphabet.  All other chars are prohibited.

## Installing pam module
go to the pam project's directory and run:
```
just run
```
**Warning**
In the Justfile in your project, assert that the pam_module_dir variable matches your system's path  or installation will fail.

## License

[MIT](https://github.com/abrandec/pam-rust-template/blob/master/LICENSE)

## Acknowledgements
- [anowell/pam-rs](https://github.com/anowell/pam-rs)
- [1wilkens/pam](https://github.com/1wilkens/pam)
