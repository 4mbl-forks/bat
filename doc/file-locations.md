# File Location and Range Syntax

## 1. Supported syntax

### File and exact locations

| Syntax | Meaning |
| --- | --- |
| `file.md` | Entire file |
| `file.md:12` | Line 12 |
| `file.md:12:5` | Line 12, column 5 |
| `file.md::5` | Column 5 across all lines |

### Line ranges

| Syntax | Meaning |
| --- | --- |
| `file.md:12-20` | Lines 12–20 |
| `file.md:12-` | Line 12 through EOF |
| `file.md:-20` | Beginning of file through line 20 |
| `file.md:-` | Entire file |

### Column ranges

| Syntax | Meaning |
| --- | --- |
| `file.md:12:5-10` | Columns 5–10 on line 12 |
| `file.md:12:5-` | Column 5 through EOL on line 12 |
| `file.md:12:-10` | Beginning of line 12 through column 10 |
| `file.md::5-10` | Columns 5–10 across all lines |
| `file.md::5-` | Column 5 through EOL on every line |
| `file.md::-10` | Beginning of every line through column 10 |

### Combined line and column ranges

| Syntax | Meaning |
| --- | --- |
| `file.md:12-20:5` | Lines 12–20, column 5 |
| `file.md:12-20:5-10` | Lines 12–20, columns 5–10 |
| `file.md:12-:5-10` | Line 12 through EOF, columns 5–10 |
| `file.md:-20:5-10` | Lines 1–20, columns 5–10 |
| `file.md:12-:5-` | Line 12 through EOF, column 5 through EOL |
| `file.md:-:5-10` | All lines, columns 5–10 |

## 2. Grammar

```text
LOCATION := FILE
          | FILE ":" LINE_SELECTOR
          | FILE ":" LINE_SELECTOR ":" COLUMN_SELECTOR
          | FILE "::" COLUMN_SELECTOR

LINE_SELECTOR   := INTEGER
                 | INTEGER "-" INTEGER
                 | INTEGER "-"
                 | "-" INTEGER
                 | "-"

COLUMN_SELECTOR := INTEGER
                 | INTEGER "-" INTEGER
                 | INTEGER "-"
                 | "-" INTEGER
                 | "-"
```

## 3. Semantics

- Line and column numbers are 1-based.
- Range endpoints are inclusive.
- An exact number selects one line or column.
- `N-M` selects from N through M.
- `N-` selects from N through the end.
- `-N` selects from the beginning through N.
- `-` selects the entire dimension.
- An omitted line selector means all lines.
- An omitted column selector means all columns.
- Column selection applies independently to each selected line.
- `FILE::COLUMN_SELECTOR` selects columns across the entire file.
- A column selector requires a line selector syntactically, except for the explicit `FILE::COLUMN_SELECTOR` form.
- Coordinates must be positive integers. Explicit range starts must not exceed explicit range ends.
- Out-of-bounds endpoints are handled gracefully, without errors that interrupt normal viewing.

## 4. Parsing and backward compatibility

- Ordinary file arguments and existing CLI behavior are preserved.
- An existing file matching the complete argument takes precedence over interpreting its suffix as a location.
- Valid filenames containing colons and Windows drive-letter paths are preserved.
- Unrelated command-line options are not interpreted as file locations.
- Existing line-range options remain compatible with the shorthand.
- Malformed location syntax does not cause crashes or unintended file selection.
- Multiple file arguments and existing stdin behavior retain their established semantics.
