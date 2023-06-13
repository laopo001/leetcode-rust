/**
 * @param {string} s
 * @return {number}
 */

// 312 ms, faster than 19.61%
var minimumDeletions = function (s) {
    let a = 0;
    let b = 0;
    let z = [];
    let start_a = 0;
    for (let i = 0; i < s.length; i++) {
        const x = s[i];
        if (a != 0 && b != 0) {
            if (x == 'b') {
                // res += Math.min(a, b);
                z.push([a, b])
                a = 0;
                b = 1;
            } else if (x == 'a') {
                a++;
            }
        } else if (a == 0 && b != 0) {
            if (x == 'b') {
                b++;
            } else if (x == 'a') {
                a = 1;
            }
        } else {
            if (x == 'b') {
                b = 1;
                if (start_a != 0) {
                    z.push([start_a, 0])
                }
                start_a = 0;
            } else if (x == 'a') {
                start_a++;
            }
        }
    }
    if (start_a != 0) {
        z.push([start_a, b])
    }
    if (a != 0 || b != 0) {
        z.push([a, b])
    }
    let w = Infinity;
    let o = z.map(x => x[0]).reduce((a, b) => {
        return a + b
    }, 0)
    w = Math.min(
        w, o
    );
    let p = 0;
    for (let i = 0; i < z.length; i++) {
        p += z[i][1];
        o -= z[i][0]
        w = Math.min(
            w, (p + o)
        );
    }

    return w;
};
var res = minimumDeletions("bbaaabaabbabbbab")
console.log(res);