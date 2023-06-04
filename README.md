# Meroshare CLI written in RUST

Are you tired of having to manually fill shares. Well this project has you covered. You can check list of open shares, fill one and come back and view the results and much more.

## How to use?

Clone the repo or download the executable from the releases pages and run the executable. If you clone, just run `cargo run` on the root of the project.

## Users List

The script requires a JSON file containing the details about the users. If a `users.json` file is found in the same directory as the executable, the JSON is automatically picked and used, if not, the script prompts you to enter a path of the JSON file on starting.

    [
      {
        "dp": "xxx",
        "username": "xxxxx",
        "password": "xxxxxxxx",
        "crn": "xxxxx",
        "pin": "xxxx",
        "name": "xxxxxx",
        "asbaBankIndex": 1,
        "tags":["family"]
      },
      {
        "dp": "xxx",
        "username": "xxxxx",
        "password": "xxxxxxxx",
        "crn": "xxxxx",
        "pin": "xxxx",
        "name": "xxxxxx",
      },
    ]

`dp` is the code of the capital that can be found in login page of meroshare. Example: `16700` for Mahalaxmi Bikas Bank.
`username` is the username used for logging in using meroshare.
`password` password is the password used to login to meroshare.
`crn` and `pin` are self describable.
`name` is used for logging purpose and can be anything you wish.
`asbaBankIndex (optional)` if you have multiple banks linked, this indicates the index of the bank to be used. Leave the default to be 1.
`tags (optional)` Often times we want to calculate total assets and profits of multiple users at once. We can do that with `tags`. Currently only the family tag is supported.
Download [data.json.example](users.json.example), and rename it as `users.json`. Fill all the user's info and you are ready to roll.
