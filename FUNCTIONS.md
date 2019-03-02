# Functions

List of functions implemented and provided by default with the `Formatter`.


### Arithmetic

  * add
     `$add(number1, number2, ...)` returns the sum of all integer values passed by argument.
  * div
     `$div(number1, number2)` returns the integer `number1` divided by the integer `number2`.
  * greater
     `$greater(number1, number2)` returns the truth value for the integer `number1` > the integer `number2`.
  * max
     `$max(number1, number2, ...)` returns the maximum of all integer number values passed by argument.
  * min
     `$min(number1, number2, ...)` returns the maximum of all integer number values passed by argument.
  * mod
     `$mod(number1, number2)` returns the integer `number1` modulo the integer `number2`.
  * mul
     `$mul(number1, number2, ...)` returns the product of all integer number values passed by argument.
  * muldiv
     `$muldiv(number1, number2, number3)` returns `number1` * `number2` / `number3`.
  * sub
     `$sub(number1, number2, ...)` returns the integer `number1` minus all the subsequent integer numbers passed by argument.

### Logic

  * and
    * `$and(expr, ...)` returns true if all arguments evaluate to true (logical and).
  * or
    * `$or(expr, ...)` returns true if at least one argument evaluate to true (logical or).
  * not
    * `$not(expr)` returns false if the argument evaluates to true and true if the argument evaluates to false (logical not).
  * xor
    * `$xor(expr, ...)` returns true if an odd number of argument evaluates to true and false otherwise (logical exclusive or).

### Control flow

  * if
    * `$if(cond, then, else)` returns the value `then` if `cond` evaluates to true and the value `else` otherwise.
  * if2
    * `$if2(expr, else)` returns the value `expr` if it evaluates to true and the value `else` otherwise.
  * if3
    * `$if3(expr1, expr2, ..., else)` returns the first value among `expr1`, `expr2`, ... that evaluates to true and the value `else` if none do.
  * ifequal
    * `$ifequal(n1, n2, then, else)` returns the value `then` if the integer `n1` is equal to the number `n2` and the value `else` otherwise.
  * ifgreater
    * `$ifgreater(n1, n2, then, else)` returns the value `then` if the integer `n1` > the integer `n2` and the value `else` otherwise.
  * iflonger
    * `$iflonger(text, len, then, else)` returns the value `then` if `text` has more than `n` characters and the value `else` otherwise.
  * select
    * `$select(n, value1, value2, value3, ...)` takes a number `n` and any number of values as arguments and returns the n-th value.

### Strings

  * abbr
    * `$abbr(text)` returns a string with the first characters of each word in `text`.
  * ascii (if the `unicode-normalization` optional dependency is enabled, which is the case by default)
    * `$ascii(text)` converts `text` to ASCII. Any character that can not be converted to ASCII is removed or replaced.
  * caps
    * `$caps(text)` changes the first character of each word in `text` to uppercase, and all the other characters to lowercase.
  * caps2
    * `$caps2(text)` changes the first character of each word in `text` to uppercase and does not affect the other characters.
  * cut
    * `$cut(text, n)` returns the leftmost n characters of the text passed by argument (it is the same as left).
  * directory
    * `$directory(path_text)` returns the name of the immediate parent directory of a path passed by argument.
    * `$directory(path_text, n)` returns the name of the immediate parent directory of a path passed by argument.
  * directory_path
    * `$directory_path(path_text)` returns the full path to the parent directory of a path passed by argument.
  * ext
    * `$ext(path_text)` returns the extension part of a file path passed by argument.
  * filename
    * `$filename(path_text)` returns the filename part (without the extension) of a path passed by argument.
  * hex
    * `$hex(number)` returns a string of the hexadecimal representation of the number passed by argument.
  * insert
    * `$insert(original_text, inserted_text, n)` returns a string containing `original_text` with `inserted_text` added in after `n` characters. 
  * left
    * `$left(text, n)` returns the leftmost n characters of the text passed by argument (it is the same as cut).
  * right
    * `$right(text, n)` returns the rightmost n characters of the text passed by argument.
  * substr
    * `$substr(text, from, to)` returns a sub-string of the text passed by argument, starting at the from-th character and ending at the to-th character.