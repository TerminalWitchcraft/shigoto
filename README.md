# Shigoto

[![Build Status](https://travis-ci.org/TerminalWitchcraft/shigoto.svg?branch=master)](https://travis-ci.org/TerminalWitchcraft/shigoto)

Shigoto is a command line task management utility. Currently it is in its early stages of development and hence it is unstable.

# Usage

First, lets start with the three basic usage: `add`, `done` and `list`

## Adding a Task

```
$ sht add "Buy milk"
```

## Listing all Tasks 

```
$ sht list

┌────┬──────────────────────────┬───────────────────────┬──────────┬───────────────────────┬────────────┬────────────┐
| ID | Name                     | Created               | Priority | Due                   | Completed? | Tags       |
├====┼==========================┼=======================┼==========┼=======================┼============┼============┤
| 1  | Complete the time module | 18:32 Tue, 14 Aug 18' | High     | 18:32 Tue, 14 Aug 18' | false      | hobby,rust |
└────┴──────────────────────────┴───────────────────────┴──────────┴───────────────────────┴────────────┴────────────┘

```

## Marking a Task as Done

```
$ sht done task_id
```
