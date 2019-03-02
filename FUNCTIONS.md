# Functions

List of functions implemented and provided by default with the `Formatter`.

  * add
  * div
  * greater
  * max
  * min
  * mod
  * mul
  * muldiv
  * sub
  * and
  * or
  * not
  * xor
  * if
  * if2
  * if3
  * ifequal
  * ifgreater
  * iflonger
  * select
  * abbr
  * ascii (if the `unicode-normalization` optional dependency is enabled, which is the default)
  * caps
  * caps2
  * cut
    * `cut(text, n)` returns the leftmost n characters of the text passed by argument (it is the same as left).
  * directory
    * `directory(path_text)` returns the name of the immediate parent directory of a path passed by argument.
    * `directory(path_text, n)` returns the name of the immediate parent directory of a path passed by argument.
  * directory_path
    * `directory_path(path_text)` returns the full path to the parent directory of a path passed by argument.
  * ext
    * `ext(path_text)` returns the extension part of a file path passed by argument.
  * filename
    * `filename(path_text)` returns the filename part (without the extension) of a path passed by argument.
  * hex
    * `hex(number)` returns a string of the hexadecimal representation of the number passed by argument.
  * insert
    * `insert(original_text, inserted_text, n)` returns a string containing `original_text` with `inserted_text` added in after `n` characters. 
  * left
    * `left(text, n)` returns the leftmost n characters of the text passed by argument (it is the same as cut).
  * right
    * `right(text, n)` returns the rightmost n characters of the text passed by argument.
  * substr
    * `substr(text, from, to)` returns a sub-string of the text passed by argument, starting at the from-th character and ending at the to-th character.