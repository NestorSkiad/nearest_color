# Nearest Color

See **Nearest Color GUI** to get a better idea of what this project is about.

This version runs through all 8-bit colors to find the nearest named color for each.

The program's result has been included in output.txt.

## Performance

#### Single threaded

Uses 0.6MB RAM, 10% CPU.

```shell
PS C:\Users\Nestor\IdeaProjects\nearest_color> Measure-Command { .\target\release\nearest_color.exe > output.txt }

Days              : 0
Hours             : 0
Minutes           : 7
Seconds           : 52
Milliseconds      : 318
Ticks             : 4723188546
```

#### Per-color multithreaded

Uses 2MB RAM, 100% CPU.

```shell
PS C:\Users\Nestor\IdeaProjects\nearest_color> Measure-Command { .\target\release\nearest_color.exe > output.txt }

Minutes           : 6
Seconds           : 10
Milliseconds      : 842
Ticks             : 3708427107
```

#### Multithreaded from color generator

Uses ~700MB RAM, 100% CPU.

```shell
PS C:\Users\Nestor\IdeaProjects\nearest_color> Measure-Command { .\target\release\nearest_color.exe > output.txt }

Minutes           : 0
Seconds           : 55
Milliseconds      : 553
Ticks             : 555532715
```

### Rust Crates

* serde
* csv