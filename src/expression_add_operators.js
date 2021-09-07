/**
 * @param {string} num
 * @param {number} target
 * @return {string[]}
 */



var addOperators = function (num, target) {
    console.log(num, target);
    if (num.length > 1 && num[0] == 0) {
    } else {
        if (+num === target) {
            return [num];
        }
    }
    // if (+num === target) { return [num]; }
    let operators = ['+', '-', '*'];
    let res = [];
    for (let i = 1; i < num.length; i++) {
        for (let j = 0; j < operators.length; j++) {
            const o = operators[j];
            let left = num.slice(0, i);
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
            } else if (o === '*') {
                let x = num.slice(i, i + 1);
                if (num.slice(i + 1, num.length).length === 0) {
                    if (left * x == target) {
                        res.push(left + o + x);
                        continue;
                    }

                }
                for (let k = 0; k < operators.length; k++) {
                    const o2 = operators[k];
                    if (o2 === '+') {
                        let q = addOperators(
                            num.slice(i + 1, num.length),
                            target - +left * x
                        );
                        for (let i = 0; i < q.length; i++) {
                            const right = q[i];
                            res.push(left + o + x + o2 + right);
                        }
                    } else if (o2 === '-') {
                        let q = addOperators(
                            num.slice(i + 1, num.length),
                            +left * x - target
                        );
                        for (let i = 0; i < q.length; i++) {
                            const right = q[i];
                            res.push(left + o + x + o2 + right);
                        }
                    } else if (o2 === '*' && target % (+left * x) == 0) {
                        let q = addOperators(
                            num.slice(i + 1, num.length),
                            target / (+left * x)
                        );
                        for (let i = 0; i < q.length; i++) {
                            const right = q[i];
                            res.push(left + o + x + o2 + right);
                        }
                    }
                }
            }
        }
    }
    return res;
};

addOperators("105", 5)


addOperators("123", 6)

