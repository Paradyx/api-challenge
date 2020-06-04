# API challenge
This project implements a REST API that is a little bit challenging to use. 

It was first created as a challenge during a job application, but then evolved into a fun challenge for anybody who likes APIs and their specialities. 
So except for training purposes, there is **nothing** useful to this project. 

## The Challenge
A customer bought a new application and wants us to retrieve usage data 
from its internal API. Unfortunately, we don't know how the application works. 
The customer only gave us the executable which exposes the usage data 
on `127.0.0.1:8000/usage/{pageNo}`.

We are only interested in the __full name__ and the __email address__ of those users 
which used the application from a __Linux client__ in __January 2020__.

Can you try to retrieve as many usages as possible? Please, prepare a short (maximum 15 minutes) 
presentation, which should include some details on:

- How the API works?
- What you tried? What worked and what didn't?
- Which technologies you used?

## Build
You need rust and cargo installed for this to work. 

```
cargo build --release
```

## Cross compiling
On [Arch linux](https://wiki.archlinux.org/index.php/rust#Cross_compiling)
- install `mingw-w64-gcc-base` and `mingw-w64-gcc`
- configure the toolchain
```
$ rustup toolchain install stable-x86_64-pc-windows-gnu
$ rustup target add x86_64-pc-windows-gnu
```
- Fix libraries 
```
for lib in crt2.o dllcrt2.o libmsvcrt.a; do cp -v /usr/x86_64-w64-mingw32/lib/$lib $HOME/.rustup/toolchains/$CHANNEL-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/; done
```

## Solution
The api is paginated and every page contains procedurally generated 
data, which looks like this:

```json
{
  "usage": [
    {
      "uuid": "881982-12929312-129122-191",
      "user": {
        "first_name": "Abby",
        "last_name": "Johnson",
        "full_name": "Abby Grey Johnson",
        "last_usage": "2020-01-01T13:37:00+UTC",
        "ACCOUNT_OPTIONS": {
          "is_demo": true
        }
      },
      "device": {
        "hostname": "The best PC eva",
        "ip_address": "127.0.0.1",
        "operating_system": "Linux x86",
        "cpu": "Intel"
      },
      "usage_date": "2020-01-01T13:37:00+UTC",
      "usage_time": 1300
    }
  ]
}
```

With every `n` pages, the API get's harder to use.

### Level 1: Plain sane Json
- without the field `ACCOUNT_OPTIONS`
- very sane behaviour

### Level 2: Out of order
- shuffle json fields

### Level 3: Missing fields
- randomly drop one of `name`, `first_name`, `last_name`
- randomly keey `email` empty
- randomly drop some device information
- randomly drop `usage_time`

### Level 4: Random timeouts and error codes
- randomly sleep between 1 and 20 seconds
- randomly throw errors `500`, `503`, `504`, `429`, `507`. 

### Level 5: Invalid time stamps
- randomly use unix timestamps instead of RFC 3339
- randomly use RFC 2822 instead of RFC 3339

### Level 6: 1MB image
- randomly bloat json with a profile picture

### Level 7: Large page size
- increase page size, to ca 700M

# Open Ideas
- [ ] account activation
- [ ] inline indirection (chaning the name of the `.usage`)
- [ ] URL indirection replace accounts with url, which redirects to another endpoint wich contains the details
- [ ] in body errors although `200 OK` is returned. 

