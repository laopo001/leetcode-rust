/**
 * @param {number[][]} coordinates
 * @return {boolean}
 */
 var checkStraightLine = function (coordinates) {
    let x = coordinates[0][0] - coordinates[1][0]
    let y = coordinates[0][1] - coordinates[1][1];

    let k = y / x;
    if (x == 0) {
        k = x / y;
        let b = coordinates[0][0] - k * coordinates[0][1];
        // console.log(k, b)
        for (let i = 0; i < coordinates.length; i++) {
            if (coordinates[i][0] != k * coordinates[i][1] + b) {
                return false;
            }
        }
        return true;
    }
    // coordinates[1][1] = k * coordinates[0][0] + b;
    let b = coordinates[0][1] - k * coordinates[0][0];
    // console.log(k, b)
    for (let i = 0; i < coordinates.length; i++) {
        if (coordinates[i][1] != k * coordinates[i][0] + b) {
            return false;
        }
    }
    return true;
};