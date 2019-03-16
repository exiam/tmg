# tmg

Bash command for simple time management reports.

The command allow you to create per-month reports (as *.txt files) in which you will retrieve your executed tasks, per day.

For example:

```
# Content of 2019-03.txt report

2019-03-16@15:45 -> project:add_mailer (start)
2019-03-16@16:00 -> project:add_mailer (stop)
```

## Installation

Move or copy the command under `bin` directory.

```
cp ./tmg /usr/local/bin
```

## Documentation

### `start`

Write a "start" action for the given task.

```
tmg start task_name [-t=HH:MM,--time=HH:MM;-d,--diff]
```

`--time` option allow you to specify the time (hour:minutes) instead of taking current time.

`--diff` option display the time between current and last command execution, useful for time tracking (work in progress).

### `stop`

Write a "stop" action for the given task.

```
tmg stop task_name [-t=HH:MM,--time=HH:MM;-d,--diff]
```

`--time` option allow you to specify the time (hour:minutes) instead of taking current time.

`--diff` option display the time between current and last command execution, useful for time tracking (work in progress).

### `clear`

Clear report file.

```
tmg clear
```

### `view`

Display current report file.

```
tmg view
```