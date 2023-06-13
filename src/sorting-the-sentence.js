/**
 * @param {string} s
 * @return {string}
 */
// 72 ms, faster than 75.26%
var sortSentence = function (s) {
    let strs = s.split(' ');
    strs = strs.map(x => {
        return {
            str: x.slice(0, x.length - 1),
            v: +(x[x.length - 1])
        }
    })
    strs = strs.sort((a, b) => a - b);
    return strs.map(x => x.str).join(' ');
};