/**
 * @param {string} num
 * @param {number} target
 * @return {string[]}
 */


var addOperators = function (num, target) {
    if (num.length > 1 && num[0] == 0) {
    } else {
        if (+num === target) {
            return [num];
        }
    }
    let operators = ['+', '-'];
    let res = [];
    for (let i = 1; i <= num.length; i++) {
        for (let j = 0; j < operators.length; j++) {
            const o = operators[j];
            let left = num.slice(0, i);

            if (left.length === 1) {
                if (o === '+') {
                    let q = addOperators(num.slice(i, num.length), target - +left);
                    for (let i = 0; i < q.length; i++) {
                        const right = q[i];
                        res.push(left + o + right);
                    }
                } else if (o === '-') {
                    let q = addOperators(num.slice(i, num.length), +left - target);
                    for (let i = 0; i < q.length; i++) {
                        const right = q[i];
                        res.push(left + o + right);
                    }
                }
            } else {
                if (left.length > 1 && left[0] == 0) {

                } else {
                    if (o === '+') {
                        let q = addOperators(num.slice(i, num.length), target - +left);
                        for (let i = 0; i < q.length; i++) {
                            const right = q[i];
                            res.push(left + o + right);
                        }
                    } else if (o === '-') {
                        let q = addOperators(num.slice(i, num.length), +left - target);
                        for (let i = 0; i < q.length; i++) {
                            const right = q[i];
                            res.push(left + o + right);
                        }
                    }
                }
                let accumulate = 1;
                let arr = [];
                for (let i = 0; i < left.length; i++) {
                    const element = left[i];
                    accumulate = accumulate * element;
                    arr.push(element);
                }
                if (num.slice(i, num.length).length === 0 && accumulate === target) {
                    res.push(arr.join('*'))
                    break;
                }
                if (o === '+') {
                    let q = addOperators(num.slice(i, num.length), target - +accumulate);
                    for (let i = 0; i < q.length; i++) {
                        const right = q[i];
                        res.push(arr.join('*') + o + right);
                    }
                } else if (o === '-') {
                    let q = addOperators(num.slice(i, num.length), +accumulate - target);
                    for (let i = 0; i < q.length; i++) {
                        const right = q[i];
                        res.push(arr.join('*') + o + right);
                    }
                }
            }

        }
    }

    return res;
};


// var res = addOperators("105", 5)

// var res = addOperators("232", 8)

// var res = addOperators("123", 6)

// let res = addOperators("3456237490", 9191);

// var res = addOperators("00", 0)

var res = addOperators("123456789", 45)

console.log(res);