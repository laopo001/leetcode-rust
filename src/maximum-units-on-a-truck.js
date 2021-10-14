/**
 * @param {number[][]} boxTypes
 * @param {number} truckSize
 * @return {number}
 */
// 164 ms, faster than 13.11% 
 var maximumUnits = function (boxTypes, truckSize) {

    boxTypes = boxTypes.sort((a, b) => b[1] - a[1])
    let res = 0;
    let c = 0;
    for (let i = 0; i < boxTypes.length; i++) {
        const x = boxTypes[i];
        c++;
        res += x[1]
        if (c >= truckSize) {
            break;
        }
        x[0]--;
        if (x[0] > 0) {
            i--;
        }
    }
    return res;
};
