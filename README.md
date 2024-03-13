# Habits
Command line tool to track your habits.

## Features
* Configure your habits in a simple configuration file.
* Quickly check off habits from the command line.
* Track your progress over time.
* Visualize your progress with a terminal based interface.
* Export your progress to a CSV file.
* Use the built-in web server to view your progress in a web browser.

## Configuration
```
running:
  goal: 5
  unit: km
  type: counter
  frequency: daily
  start: 2021-01-01
  end: 2021-12-31
```
this configuration file will create a habit called `running` with a goal of 5 km, a counter type, a daily frequency, starting on 2021-01-01 and ending on 2021-12-31.

```
reading:
  goal: 100
  unit: pages
  type: counter
  frequency: weekly
  start: 2021-01-01
  end: 2021-12-31
```
this configuration file will create a habit called `reading` with a goal of 10 pages, a counter type, a daily frequency, starting on 2021-01-01 and ending on 2021-12-31.

```
adventure:
  type: done
  frequency: monthly
```

Generally, available fields are:
* `goal`: the goal of the habit (e.g. 5 km, 100 pages, 5 times)
* `unit`: the unit of the goal (e.g. km, pages) - left out indicates an "unit" unit (e.g. 5 times)
* `type`: the type of the habit - counter (you can count up to the goal), done (you either did it or not), time (you can count the time you did it), abstain (you can count the time you didn't do it)
* `frequency`: the frequency of the habit - daily, weekly, monthly, yearly
* `start` and `end`: the start and end date of the habit - left out indicates no start or end date