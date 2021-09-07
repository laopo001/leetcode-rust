/**
 * @param {string} s
 * @return {number}
 */


// var removePalindromeSub = function (s) {
//    if (s.length === 0) {
//        return 0;
//    }
//     let map = {};
//     debugger;
//     function isPalindromic(s, start, end) {
//         if (map[start + ',' + end] !== undefined) {
//             return map[start + ',' + end];
//         }
//         if (start == end) return true;
//         if (s[start] == s[end] && start + 1 == end) return true;
//         let res = s[start] == s[end] && isPalindromic(s, start + 1, end - 1);
//         map[start + ',' + end] = res;
//         return res;
//     }
//     let a = 0;
//     let b = s.length - 1;
//     let c = 0;
//     while (true) {
//         if (isPalindromic(s, a, b)) {
//             if (b == s.length - 1) {
//                 c++;
//                 break;
//             } else {
//                 a = b + 1;
//                 b = s.length - 1;
//                 c++;
//             }

//         } else {
//             b--;
//         }
//     }
//     return c;
// };

function isPalindromic(s, start, end) {
    if (start == end) return true;
    if (s[start] == s[end] && start + 1 == end) return true;
    let res = s[start] == s[end] && isPalindromic(s, start + 1, end - 1);
    return res;
}

var removePalindromeSub = function (s) {
    if (s.length === 0) {
        return 0;
    }
    if (isPalindromic(s, 0, s.length - 1)) {
        return 1;
    }
    return 2;
};

removePalindromeSub("bbaabaaa")

removePalindromeSub("abb")