let V = 1
var guess = function (num) {
    if (num < V) {
        return 1;
    } else if (num > V) {
        return -1;
    } else {
        return 0
    }
}
// 68 ms, faster than 92.27%
var guessNumber = function (n) {
    if (guess(0) === 0) {
        return 0;
    }
    if (guess(n) === 0) {
        return n;
    }
    let left = 0;
    let right = n;
    let middle = Math.floor((left + right) / 2);
    while (1) {
        if (guess(middle) === 1) {
            left = middle;
            middle = Math.floor((left + right) / 2);
        } else if (guess(middle) === -1) {
            right = middle;
            middle = Math.floor((left + right) / 2);
        } else {
            return middle;
        }
    }
};

var res = guessNumber(10)
console.log(res);