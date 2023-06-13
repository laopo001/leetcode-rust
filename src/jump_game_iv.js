/**
 * @param {number[]} arr
 * @return {number}
 */
var minJumps = function (arr) {
    if (arr.length === 0) return 0;
    // number : [index1, index2...]
    let map = new Map();
    for (let i = 0; i < arr.length; i++) {
        if (i !== 0 && i !== arr.length - 1 && arr[i] === arr[i + 1] && arr[i] === arr[i - 1]) continue;
        if (map.has(arr[i])) map.get(arr[i]).push(i);
        else map.set(arr[i], [i]);
    }

    let dp = new Array(arr.length).fill(Infinity);
    dp[0] = 0;
    let tag = '';

    function calDp(dp) {
        for (let i = 1; i < dp.length; i++) {
            let min = Infinity;
            let a = dp[i - 1] + 1;
            let b = i + 1 < arr.length ? dp[i + 1] + 1 : Infinity;
            let others = map.get(arr[i]);
            for (let other of others) {
                if (other !== i) min = Math.min(min, dp[other] + 1);
            }
            min = Math.min(a, b, min);
            dp[i] = min;
        }
        return dp.join('-');
    }

    let curTag = calDp(dp);
    while (curTag !== tag) {
        tag = curTag;
        curTag = calDp(dp);
    }

    return dp[arr.length - 1];
};



// Time Limit Exceeded
// var minJumps = function (arr) {
//     let map = {};
//     let dp = [];
//     for (let i = 0; i < arr.length; i++) {
//         const element = arr[i];
//         if (map[element]) {
//             map[element].push(i);
//         } else {
//             map[element] = [i];
//         }
//         dp[i] = i;
//     }

//     function run(index, val, jump) {
//         if (dp[index] < val) {
//             // console.log('------');
//             return;
//         }
//         dp[index] = Math.min(val, dp[index]);

//         let element = arr[index];
//         if (map[element] && jump) {
//             for (let i = 0; i < map[element].length; i++) {
//                 const x = map[element][i];
//                 if (x != index) {
//                     if (dp[x] > val + 1 && val + 1 < dp[dp.length - 1]) {
//                         run(x, val + 1, false);
//                     }
//                 }
//             }
//         }

//         let i = index + 1;
//         while (i < arr.length) {
//             if (dp[i] > val + (i - index) && val + (i - index) < dp[dp.length - 1]) {
//                 dp[i] = val + (i - index)
//                 run(i, val + (i - index), true);
//             } else {
//                 break;
//             }
//             i++;
//         }

//         i = index - 1;
//         while (i >= 0) {
//             if (dp[i] > val + (index - i) && val + (index - i) < dp[dp.length - 1]) {
//                 dp[i] = val + (index - i)
//                 run(i, val + (index - i), true);
//             } else {
//                 break;
//             }
//             i--;
//         }

//         // console.log(dp);
//     }
//     for (let i = 0; i < arr.length; i++) {
//         const element = arr[i];
//         run(i, i, true)
//     }
//     // run(0, 0, true)
//     // console.log(arr.map((x, i) => {
//     //     return {
//     //         val: x,
//     //         dp: dp[i]
//     //     }
//     // }));
//     return dp[dp.length - 1];
// };

// var res = minJumps([100, -23, -23, 404, 100, 23, 23, 23, 3, 404]);
var res = minJumps([11, 22, 7, 7, 7, 7, 7, 7, 7, 22, 13]);
// var res = minJumps([-76, 3, 66, -32, 64, 2, -19, -8, -5, -93, 80, -5, -76, -78, 64, 2, 16]);
// var res = minJumps([25, -28, -51, 61, -74, -51, -30, 58, 36, 68, -80, -64, 25, -30, -53, 36, -74, 61, -100, -30, -52]);

console.log(res);
