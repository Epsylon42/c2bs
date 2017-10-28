## c2bs

c2bs is a program with a terrible name built for a righteous cause. It convets pseudocode into TeX which compiles to flowcharts. You might want to learn TeX (at least some of it) to use c2bs. It's not required though.

Also, you should not use c2bs yet. Unless you are ready to deal with nonexistent error messages and TeX things. 

*ahem* and bad tutorials *ahem*

### usage

1. `c2bs prog.alg > prog.tex`
2. `xelatex prog.tex`
3. `???`
4. you have a `prog.pdf`

In Ubuntu xelatex can be installed with `sudo apt install texlive-xetex`

### input structure

Flowcharts are surrounded with `flowchart { ... }`

They start with name, input variables and output variables. Input and output are optional.

Flowcharts consist of expressions. Currently, three are supported:
* if-else
* while
* block

Expressions are separated with newlines or semicolons.

Conditions inside of `if` and `while` are also a kind of block, so most thing that apply to block, also apply to the conditions.


```
flowchart {
    name: flowchartName  // name consists of alphanumeric characters
    in: var1, var2  // input variables (optional)
    out: var1, var2  // output variables (optional)
    
    if ( condition ) {
        doSomething1
        doSomething2
    } else {  // else is optional
        doSomethingElse
    }
    
    if ( condition ) {
        doSomethingWithoutElse
    }
    
    while ( condition ) {
        doSomethingInLoop
    }
}
```
*comments are not part of the syntax (now)*

### escaping

Some characters are special and have to be escaped in order to be interpreted correctly.

These are:
* `;`
* `{` and `}`
* `(` and `)`
* `\`

To escape a character is to put a `%` before it:
* `%;`
* `%{`
* `%(`
* `%%`
* etc

Third point is an escaped percent sign. Since percent sign is used for escaping, you have to escape the percent itself to use it literally.

Any character can be escaped, but for some of them it's required. 
Currently these are:
* `{`
* `}`
* `(`
* `)`
* `%`
* `;`

### just TeX things

Since rendering awesome things is hard, c2bs doesn't make PDF's itself. It uses TeX as its backend instead. This leads to it having some cool features. Also a couple of pitfalls, but mostly features.

TeX uses commands that start with a backslash to represent special characters.

* `\neq` not equal
* `\ge` greater than or equal
* `\le` less that or equal
* `\cdot` multiplication dot
* `\sqrt{number}` square root of number
* `\frac{numerator}{denominator}` fancy fraction because `/` is not fancy enough
* etc

They can only be used in math mode. That is, be surrounded with dollar signs (`$ math expression thing $`). But, c2bs surrounds block expressions with dollar signs automatically so that you don't have to worry about it. But you can still add dollar signs to disable math mode. It may be useful since math mode handles spaces weirdly.

`mathVariable = $NotMathThing$`

Also, since brackets are special characters in c2bs, they have to be escaped, so the above commands will look like this:
* `\sqrt %{ number %}`
* `\frac %{ numerator %} %{ denominator %}`

### common problems and their solutions

##### space problems
A result of math mode weirdness.
Something like this: `var = SomeFunction%(%)` will look ugly in math mode. You might want do disable math mode for SomeFunction: `var = $SomeFunction%(%)$`. This will still look bad, but in a different way. That is, there won't be a space between the assignment and the function. The correct way to do the thing is `var =$ SomeFunction%(%)$`. Spaces work like expected in not-math mode.

In short, the solution to space problems is to add dollar signs and more spaces until it worls. Or learn TeX, I guess.

##### overlapping elements and other formatting problems
Wait until manual offsets are implemented. Or edit the program output. Honestly, the first solution is better.

##### bad error messages
¯\\\_(ツ)\_/¯
More like none at all, amrite?
May be fixed in the future when I learn how to handle errors in the parser library I use.

### features that may or may not happen

- [x] flowchart names
- [x] input and output variables
- [x] blocks
- [x] if
- [x] while
- [ ] for
- [ ] do-while
- [ ] I/O blocks
- [ ] comments
- [ ] manually changing offsets between expressions
- [x] not backslash as the escaping symbol
- [ ] style configuration
- [ ] web interface
- [ ] good error messages

## TL;DR: Learn TeX
