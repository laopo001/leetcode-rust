/**
 * @param {string} s
 * @param {number} k
 * @return {string}
 */
var decodeAtIndex = function (s, k) {
    let str = [];
    for (let i = 0; i < s.length; i++) {
        const x = s[i];
        let n = +x;
        if (isNaN(n)) {
            str.push(x);
        } else {
            let l = str.map(x => x);
            for (let i = 1; i < n; i++) {
                Array.prototype.push.apply(str, l);
                if (str.length > k - 1) {
                    return str[k - 1];
                }
            }
        }
    }
    return str[k - 1];
};

var decodeAtIndex = function (s, k) {
    let len = 0;
    let isDigit = false

    for (let v of s) {
        if (v >= '0' && v <= '9') {
            len *= +v
            isDigit = true
        } else {
            len++
            if (len === k && !isDigit) {
                return s[k - 1]
            }
        }
    }

    for (let i = s.length - 1; i >= 0; i--) {
        const v = s[i]
        if (v >= '0' && v <= '9') {
            len = Math.ceil(len / +v) // Math.floor() wont work because we endup leaving few strings
            k %= len
        } else {
            if (k === 0 || k === len) {
                return v
            }
            len--
        }
    }
};

decodeAtIndex("leet2code3"
    , 10)