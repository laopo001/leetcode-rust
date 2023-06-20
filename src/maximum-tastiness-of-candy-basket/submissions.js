/**
 * @param {number[]} price
 * @param {number} k
 * @return {number}
 */
var maximumTastiness = function (price, k) {
    price.sort((a, b) => a - b)
    let right = price[price.length - 1] - price[0];
    let left = 0;
    while (left < right) {
        let mid = (left + right + 1) >> 1;
        if (check(mid, price, k)) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    return left
};

function check(mid, price, k) {
    let c = 1;
    let pre = price[0]
    for (let i = 1; i < price.length; i++) {
        if (price[i] - pre >= mid) {
            pre = price[i]
            c++;
            if (c == k) {
                return true;
            }
        }
    }
    return false;
}

console.log(
    maximumTastiness([13, 5, 1, 8, 21, 2], 3)
)
