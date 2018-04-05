# Shigoto

[![Build Status](https://travis-ci.org/hiteshpaul/shigoto.svg?branch=master)](https://travis-ci.org/hiteshpaul/shigoto)

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
┌────┬──────────┬───────────────────────┬──────────┬───────────────────────┬────────────┬───────┐
| ID | Name     | Created               | Priority | Due                   | Completed? | Tags  |
├====┼==========┼=======================┼==========┼=======================┼============┼=======┤
| 1  | Buy milk | 09:28 Thu, 05 Apr 18' | Medium   | 09:28 Thu, 05 Apr 18' | false      | hello |
└────┴──────────┴───────────────────────┴──────────┴───────────────────────┴────────────┴───────┘
```

## Marking a Task as Done

```
$ sht done task_id
```
