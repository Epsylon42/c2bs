flowchart {
    name: example
    in: a, b, c
    out: res

    if ( a < 0 ) {
        res := $NaN$
    } else {
        sum := 0
        i := b
        while ( i < c ) {
            sum = sum + i \\cdot a
            i = i + 1
        }

        while (condition) {
            block; otherBlock
        }

        res := sum
    }
}
