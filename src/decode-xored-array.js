/**
 * @param {number[]} encoded
 * @param {number} first
 * @return {number[]}
 */
// 193 ms, faster than 15.87% 
var decode = function (encoded, first) {
    let arr = [first];
    for (let i = 0; i < encoded.length; i++) {
        const x = encoded[i] ^ first;
        first = x;
        arr.push(x)
    }
    return arr;
};