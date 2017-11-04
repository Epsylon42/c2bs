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
            sum = sum + i \cdot a
            i = i + 1
        }

        while (condition) {
            block; $otherBlock$
            arg := \frac %{ 1 %} %{ 2 %}
            $functionCall %( $arg$ %)$
        }

        res := sum
    }

    for (i from 0 to 10) {
        dosomething
    }
}
