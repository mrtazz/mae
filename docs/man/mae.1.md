---
title: mae
section: 1
footer: mae VERSION_PLACEHOLDER
header: mae User's Manual
author: Daniel Schauenberg <d@unwiredcouch.com>
date: DATE_PLACEHOLDER
---

<!-- This is the sniple(1) man page, written in Markdown. -->
<!-- To generate the roff version, run `make man` -->

# NAME

mae — a tool to list and save attachments from emails


# SYNOPSIS

`mae list [--since] [--type] [--maildir]`

`mae extract [--since] [--type] [--maildir]`

`mae stats [-maildir]`



# EXAMPLES

`mae list`
: Lists attachments found in emails..

`mae extract`
: Extracts attachments from emails.

`mae stats`
: Print stats about attachments.


# DESCRIPTION

`mae` is a command line utility to extract attachments from emails. The main
purpose is to store PDFs somewhere else.


# META OPTIONS AND COMMANDS

`--help`
: Show list of command-line options.

`version`
: Show version of mae.



# AUTHOR

mae is maintained by mrtazz.

**Source code:** `https://github.com/mrtazz/mae`

# REPORTING BUGS

- https://github.com/mrtazz/mae/issues

# SEE ALSO

- https://github.com/mrtazz/vim-mae
