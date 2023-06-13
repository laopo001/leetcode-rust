/**
 * @param {number[][]} grid
 * @return {number}
 */

// 140 ms, faster than 68.40% 
var shortestBridge = function (grid) {
    if (grid.length == 0) {
        return 0
    }
    outerloop:
    for (let i = 0; i < grid.length; i++) {
        for (let j = 0; j < grid[i].length; j++) {
            const x = grid[i][j];
            if (x == 1) {
                add(grid, i, j, 2)
                break  outerloop;
            }
        }
    }
    // console.log(grid);
    let set = new Set();
    for (let i = 0; i < grid.length; i++) {
        for (let j = 0; j < grid[i].length; j++) {
            const x = grid[i][j];
            if (x == 2) {
                set.add(i + ',' + j)
            }
        }
    };
    return add2(grid, set, 0)
}
function verify(grid, i, j) {
    if (i >= 0 && i < grid.length && j >= 0 && j < grid[0].length) {
        return true
    } else {
        return false
    }
}

function add(grid, i, j, v) {
    if (verify(grid, i, j)) {
        if (grid[i][j] == 1) {
            grid[i][j] = v
            add(grid, i + 1, j, v)
            add(grid, i - 1, j, v)
            add(grid, i, j + 1, v)
            add(grid, i, j - 1, v)
        }
    } else {
        return;
    }
}
function add2run(grid, i, j, newSet) {
    if (verify(grid, i, j)) {
        let v = grid[i][j];
        if (v == 0) {
            newSet.add(i + ',' + j)
        } else if (v == 1) {
            return 1;
        }
        grid[i][j] = 2
    }
}

function add2(grid, set, res) {
    let newSet = new Set();

    for (const x of set) {
        let [i, j] = x.split(',').map(x => +x);
        if (add2run(grid, i + 1, j, newSet) == 1) {
            return res;
        }
        if (add2run(grid, i - 1, j, newSet) == 1) {
            return res;
        }
        if (add2run(grid, i, j + 1, newSet) == 1) {
            return res;
        }
        if (add2run(grid, i, j - 1, newSet) == 1) {
            return res;
        }
    }
    return add2(grid, newSet, res + 1)
}
var res = shortestBridge([[0, 1, 0], [0, 0, 0], [0, 0, 1]])

console.log(res);