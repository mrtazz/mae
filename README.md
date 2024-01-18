# mae - mail attachment extractor

## Overview

`mae` is a command line tool to extract attachments from mails


## Usage

```
% mae export --maildir=test/fixtures/simple --output-dir=test/tmp_output
```

```
% man mae
Usage: mae [COMMAND]

Commands:
  extract  Extract all attachments from mails in given maildir
  list     List all available things
  version  Print version and exit
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### With offlineimap

```
postsynchook = mae extract --maildir=/home/mrtazz/mails/INBOX --output-dir ~/Documents
```

## Installation

There are pre-built artifacts [on the releases page](https://github.com/mrtazz/mae/releases)
or on macOS it's available in homebrew:

```
brew tap mrtazz/oss
brew install mrtazz/oss/mae
```

## Thanks

Inspiration for this came from the fabulous [@till](https://github.com/till)
