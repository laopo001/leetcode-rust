/**
 * @param {number} n
 * @return {number}
 */
var calculate = function (low, high) {
    if (low >= high)
        return 0;
    let minres = Infinity;
    for (let i = Math.floor((low + high) / 2); i <= high; i++) {
        let res = i + Math.max(calculate(i + 1, high), calculate(low, i - 1));
        minres = Math.min(res, minres);
    }
    return minres;
}

var getMoneyAmount = function (n) {
    return calculate(1, n);
};
let res = getMoneyAmount(10);
console.log(res);