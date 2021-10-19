/**
 * @param {number} n
 * @param {number[][]} mines
 * @return {number}
 */
// 388 ms, faster than 63.12%
var orderOfLargestPlusSign = function (n, mines) {
    let matrix = [];
    for (let i = 0; i < n; i++) {
        let arr = [];
        for (let j = 0; j < n; j++) {
            arr.push(1);
        }
        matrix.push(arr);
    }
    for (let i = 0; i < mines.length; i++) {
        const element = mines[i];
        matrix[element[0]][element[1]] = 0;
    }
    console.log(matrix);

    let down = [];
    let right = [];
    for (let i = 0; i < n; i++) { // y
        for (let j = 0; j < n; j++) { // x
            if (right[i] == null) {
                right[i] = []
            }
            if (down[i] == null) {
                down[i] = []
            }
            down[i][j] = 0;
            right[i][j] = 0;
            if (matrix[i][j] == 1) {
                if (i - 1 == -1) {
                    down[i][j] = 1
                } else {
                    down[i][j] = down[i - 1][j] + 1;
                }
                if (j - 1 == -1) {
                    right[i][j] = 1
                } else {
                    right[i][j] = right[i][j - 1] + 1;
                }
            }
        }
    }

    let up = [];
    let left = [];
    for (let i = n - 1; i >= 0; i--) {
        for (let j = n - 1; j >= 0; j--) {
            if (up[j] == null) {
                up[j] = []
            }
            if (left[i] == null) {
                left[i] = []
            }
            up[i][j] = 0;
            left[i][j] = 0;
            if (matrix[i][j] == 1) {
                if (i + 1 == n) {
                    up[i][j] = 1
                } else {
                    up[i][j] = up[i + 1][j] + 1;
                }
                if (j + 1 == n) {
                    left[i][j] = 1
                } else {
                    left[i][j] = left[i][j + 1] + 1;
                }
            }
        }
    }
    let res = 0;
    for (let i = 0; i < n; i++) { // y
        for (let j = 0; j < n; j++) { // x
            res = Math.max(res, Math.min(left[i][j], up[i][j], right[i][j], down[i][j]))
        }
    }
    return res;
};

orderOfLargestPlusSign(5, [[4, 2]])