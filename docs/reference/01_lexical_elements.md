# Lexical Elements

Kern's tokens can be categorized into the following:

- Identifiers
- Keywords
- Literals
- Punctuation
- Operators

Additionally, there are a lot of non-tokens, such as comments or whitespace.

## Identifiers
Identifiers are names of variables, functions and user-defined bundled data, like structs or arrays. The first character can be any letter, subsequent characters are more letters, digits or an underscore. Identifiers are case-sensitive, meaning that names like `foo`, `FOO` and `Foo` are not identical.

As a regular expression, identifiers are described like this:
```
[a-zA-Z][_a-zA-Z\d]*
```

## [WIP] Keywords
Keywords are intrinsic identifiers of the programming language. They only consist of lowercase letters. The only exception are data types, which also contain digits. As of now, the current keywords are planned:

```
# Data Types
isize i8 i16 i32 i64 i128
usize i8 u16 u32 u64 u128
fixsize fix8 fix16 fix32 fix64 fix128
flt32 flt64 flt128 bflt16
bool

# Literals
null true false

# Control Flow
if else match
loop for while break skip
return defer

# Contracts
pre inv post

# User-Defined Types
struct union enum pack

# Top-Level Definitions
let fn use

# Intermediate Clauses
as to in of

# Qualifiers
const pub vol comptime

# Test Units
assert test

# Coroutines
corout run yield cancel

# Interop
export extern asm

# Casts
bitcast valcast

# Type Introspection and Layout
align size type
```

## Literals
Literals are constant values.

### Integers
Integers can be binary, octal, decimal or hexadecimal values. They can be prefixed with a unary plus or minus sign, but they are not part of the integer.

Decimal numbers have no base prefix. They are constructed like this:
- `42`
- `-2343`
- `9`

Binary numbers are prefixed with `0b` or `0B` and only allow `0` or `1` as a digit. Some examples:
- `0b00111010`
- `-0B01101101`

Octal numbers are prefixed with `0o` or `0O` and allow the numbers from `0` to `7` inclusively per digit. Some examples:
- `0o23456452`
- `-0O34`

Hexadecimal numbers use the `0x` or `0X` prefix. Their digits range from `0` to `9` and `a` or `A` to `f` or `F`, while those letters represent the values `10` to `15` on a single digit. Some examples:
- `0x7623bfe2`
- `-0XDEADBEEF`

Integer constants can contain as many leading zeroes (after the base prefix) as the user wishes. This will emit no errors or warnings and can be useful for readability in certain scenarios.

### Real Numbers
Real number constants use digits, a decimal point, plus/minus signs and letters for exponents. A real number consists of an integer part and a mantissa part.

The construction of the integer part is identical to pure integer constants. The digit representation of the mantissa depends on the base of the integer.

Kern has two different types of real number types: Floating-point numbers (or floats) and fixed-point numbers (fixed).

The real number can be defined without an integer or a mantissa. The decimal dot is optional when the real constant needs no explicit mantissa, but writing only the mantissa requires that dot. A constant must not solely contain of a dot. The following examples are valid for both float- and fixed-point variables:
- `3.1415`
- `-0o3453.34`
- `3`
- `0.0`
- `0.3442`
- `7.62`
- `.50`
- `234.`

Exponents are exclusive to decimal and hexadecimal float values. The letter `e`/`E` (base 10) or `p`/`P` (base 2), an optional unary plus/minus and an integer value can be attached to a decimal and hexadecimal value respectively. Here are some valid examples:
- `5e2`
- `-32.3E-2`
- `0x234.ep3`

### Characters and Strings
A character is any symbol, such as letters, digits or special characters. They are declared with `"` double quotes at the beginning and the end of a symbol. For example, `"A"` is a character constant. Typically, characters are 8-bit sized unsigned integers (`u8`) per default to represent the ASCII values corresponding to the symbol. Other characters outside of the ASCII or UTF-8 character table require different encodings, usually just a higher bit size.

Characters can also contain escape sequences. They are prefixed with a `\` backslash and have one specific character, but they work internally as one symbol. `"\n"` for example counts as one character. Escape sequences offer special functionalities for delimiting, formatting or other actions.

Here is a list of current escape sequences:
- `\\` (Backslash character)
- `\"` (Double quotation mark)
- `\b` (Backspace)
- `\e` (Escape character)
- `\n` (New line)
- `\r` (Carriage return)
- `\t` (Horizontal tab)
- `\v` (Vertical tab)
- `\xhh` (Character code, exactly two hex digits, up to 0x7f when ASCII)
- `\0` (Null terminal)
- `\uhhhh` (24-bit unicode character code, up to six hex digits)

Strings are character sequences that also start and end with a `"`. Strings can also be prefixed with a `$` sign that indicate the string processing mode. The following modes exist:
- No prefix (no modifiers)
- `$r` (Raw strings, escape sequences have no effects and will be printed)
- `$b` (Byte encoding, only ASCII characters)
- `$c` (C-compatible strings)

Example of a raw ASCII string: `let text point to u8 = $rb"Raw text."`

## Punctuation
Punctuation are token separators and syntactic units. The following punctuators exist:
- `{}` (Scope)
- `,` (Value separator)
- `;` (Statement separator, not necessary when the line only contains one statement)
- `->` (Function return type)
- `=>` (Match arm)
- `<T,U>` (Syntax for generic types `T` and `U` of parameters)
- `""` (Strings and characters)
- `$` (String processing mode)
- `#` (Comments)
- `##` (Doc comments)

### [WIP] Operators
Operators are tokens that perform calculations and ensure accesses. The following operators exist (expand list later on):
- `=` (Assignment)
- `+` (Addition, unary positive)
- `-` (Subtraction, unary negative)
- `*` (Multiplication, variable dereference)
- `/` (Division of floats, integer division)
- `%` (Remainder)
- `and` (Boolean and bitwise AND, variable reference)
- `or` (Boolean and bitwise OR)
- `not` (Boolean NOT, Bitwise One's complement)
- `xor` (Bitwise XOR)
- `shl` (Bit Left Shift)
- `shr` (Bit Right Shift)
- `eq` (Is equal)
- `neq` (Is not equal)
- `gt` (Greater than)
- `gte` (Greater than equal)
- `lt` (Less than)
- `lte` (Less than equal)
- `.` (Field member)
- `..` (Range)
- `..=` (Inclusive range)
- `()` (Grouping expressions)
- `[]` (Indexing, slices)
- `_` (Wildcard)
- `:` (Quantifier, indicates either dimension or instructs a value to be set for the given amout of elements)
- `^` (Variable dereference)
- `@` (Variable reference)

Kern does not support compound operators, such as `+=`. It also does not have prefix or postfix incrementations/decrementations, such as `c++`.