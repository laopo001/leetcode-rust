/**
 * @param {string} num
 * @param {number} target
 * @return {string[]}
 */

function addOperators(num, target) {
    if (num.length === 0) return [];
    if (num.length === 1) {
        if (+num[0] === target) {
            return [num[0]];
        } else {
            return [];
        }
    }
    let ans = [];
    dfs(num, target, 1, +num[0], num[0], ans);
    return ans;
};

function dfs(num, target, p, lastInt, curry, ans) {
    if (p === num.length) {
        if (eval(curry) === target) {
            ans.push(curry);
        }
        return;
    }

    dfs(num, target, p + 1, +num[p], curry + '+' + num[p], ans);
    dfs(num, target, p + 1, +num[p], curry + '-' + num[p], ans);
    dfs(num, target, p + 1, +num[p], curry + '*' + num[p], ans);
    if (Number(lastInt) !== 0) {
        dfs(num, target, p + 1, +`${lastInt}${num[p]}`, curry + num[p], ans);
    }
}

//////////////

var run = function (arr1, arr2, str, left, index, num, target, res) {

    if (index == num.length) {
        if (+arr1.join('') == target) {
            res.push(arr1.join(''));
        }
        if (eval(str) == target) {
            res.push(str);
        }
        return;
    }
    let q = num[index];
    // run([...arr1, q], [], str, left, index + 1, num, target, res);
    run([...arr1, q], [], str, left, index + 1, num, target, res);
    run([], [...arr2, q], str, left, index + 1, num, target, res);

    if (arr1.length > 0) {
        let z = addOperators(num.slice(index, num.length), target - (+arr1.join('')))
        for (let i = 0; i < z.length; i++) {
            const element = z[i];
            res.push(arr1.join('') + '+' + element);
        }
        z = addOperators(num.slice(index, num.length), (+arr1.join('')) - target)
        for (let i = 0; i < z.length; i++) {
            const element = z[i];
            res.push(arr1.join('') + '-' + element);
        }
    }

}


var addOperators = function (num, target) {
    console.log(...arguments);
    if (num.length === 0) return [];
    if (num.length === 1) {
        if (+num[0] === target) {
            return [num[0]];
        } else {
            return [];
        }
    }
    let res = [];
    run([], [], "", undefined, 0, num, target, res);
    return res;
};

/////////////

/**
 * @param num
 * @param target
 * @returns {number}
 */
 var addOperators = (num, target) => {
    let res = [], ans = [];
    const backTrack = (num0, val, pre, k) => {
        if (num0.length < 1) {
            if (val === target) {
                ans.push(res.slice(0, k).join(''));
            }
            return;
        }
        for (let i = 0; i < num0.length; i++) {
            if (k === 0) {
                res[k] = (num0.slice(0, i + 1));
                backTrack(num0.slice(i + 1), Number(res[k]), Number(res[k]), k + 1);
            } else {
                res[k] = ('+');
                res[k + 1] = num0.slice(0, i + 1);
                backTrack(num0.slice(i + 1), val + Number(res[k + 1]), Number(res[k + 1]), k + 2);
                res[k] = ('-');
                res[k + 1] = num0.slice(0, i + 1);
                backTrack(num0.slice(i + 1), val - Number(res[k + 1]), -Number(res[k + 1]), k + 2);
                res[k] = ('*');
                res[k + 1] = num0.slice(0, i + 1);
                backTrack(num0.slice(i + 1), val - pre + pre * Number(res[k + 1]), pre * Number(res[k + 1]), k + 2);
            }
            if (num0[0] === '0') break;
        }
    };
    backTrack(num, 0, 0, 0);
    return ans;
};



var res = addOperators("105", 5)

// var res = addOperators("232", 8)

// var res = addOperators("123", 6)

// let res = addOperators("3456237490", 9191);

// var res = addOperators("00", 0)

// var res = addOperators("123456789", 45)

console.log(res);