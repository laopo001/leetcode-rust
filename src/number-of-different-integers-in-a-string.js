/**
 * @param {string} word
 * @return {number}
 */
var numDifferentIntegers = function (word) {
    let arr = [];
    let start = 0;
    let w = '';
    for (let i = 0; i < word.length; i++) {
        if (!isNaN(+word[i]) && start == 0) {
            start = 1;
            w += word[i];
        } else if (!isNaN(+word[i]) && start == 1) {
            if (w == '0') {
                w = '';
            }
            w += word[i];
        } else if (start == 1) {
            arr.push(w);
            start = 0;
            w = "";
        }
    }
    if (start == 1) {
        arr.push(w);
    }
    let set = new Set(arr);
    return set.size;
};

numDifferentIntegers("0a00");