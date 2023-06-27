function f(s) {
  let cnt = 0;
  let ch = 'z';
  for (let c of s) {
    if (c < ch) {
      ch = c;
      cnt = 1;
    } else if (c == ch) {
      cnt++;
    }
  }
  return cnt;
}

var numSmallerByFrequency = function (queries, words) {
  let count = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  for (let s of words) {
    count[f(s)]++;
  }
  for (let i = 9; i >= 0; i--) {
    count[i] += count[i + 1];
  }
  res = [];
  for (let s of queries) {
    res.push(count[f(s) + 1]);
  }
  return res;
};

console.log(
  numSmallerByFrequency(
    ["bbb", "cc"], ["a", "aa", "aaa", "aaaa"]
  )
)