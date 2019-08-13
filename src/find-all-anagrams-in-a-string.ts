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

var findAnagrams = function (s, p) {
    if (p.length > s.length) {
        return [];
    }
    let obj = {};
    let len = p.length;
    function run(str) {
        for (let i = 0; i < str.length; i++) {
            if (obj[str[i]] == null) {
                obj[str[i]] = 1;
            } else {
                obj[str[i]]++;
            }
        }
    }
    run(p);

    let res = [];
    let map = {};
    for (let k in obj) {
        map[k] = obj[k];
    }
    let q = 0;
    for (let i = 0; i < s.length; i++) {
        if (obj[s[i]] == null) {
            for (let k in map) {
                obj[k] = map[k];
            }
            q = 0;
            continue;
        } else {
            q += 1;
            obj[s[i]] -= 1;
        }
        let b = true;
        for (let k in obj) {
            b = b && obj[k] == 0;
        }
        if (b) {
            res.push(i + 1 - len);
        }
        if (q >= len) {
            let z = s[i + 1 - len];
            obj[z] += 1;
        }
    }
    return res;
};