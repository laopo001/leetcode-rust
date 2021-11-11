/**
 * @param {string} s
 * @return {boolean}
 */
// 80 ms, faster than 63.55%
var halvesAreAlike = function (s) {
    let a = s.slice(0, s.length / 2);
    let b = s.slice(s.length / 2, s.length);
    return count(a) == count(b);
};

function count(str) {
    let c = 0;
    for (let i = 0; i < str.length; i++) {
        const x = str[i];
        switch (x.toLowerCase()) {
            case 'a': c++; break;
            case 'e': c++; break;
            case 'i': c++; break;
            case 'o': c++; break;
            case 'u': c++; break;
        }
    }
    return c
}