/**
 * @param {string} s
 * @param {string} p
 * @return {number[]}
 */
var findAnagrams = function (s, p) {
    if (p.length > s.length) {
        return [];
    }
    let obj = {};
    let len = p.length;
    function run(str) {
        if (str.length == 0) {
            return [];
        }
        if (str.length == 1) {
            return [str];
        }
        let res = [];
        const c = str[0];
        let z = run(str.slice(1));
        z.forEach(s => {
            for (let i = 0; i < s.length + 1; i++) {
                let left = s.slice(0, i);
                let right = s.slice(i);
                res.push(left + c + right);
            }
        })
        return res;
    }
    let arr = run(p);
    arr.forEach(x => {
        if (obj[x] == null) {
            obj[x] = true;
        }
    });
    let res = [];
    for (let i = 0; i < s.length; i++) {
        if (s.slice(i, i + len) in obj) {
            res.push(i);
        }
    }
    return res;
};